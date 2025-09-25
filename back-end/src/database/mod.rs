pub mod init;
pub mod models;
pub mod status;

use anyhow::Result;
use chrono::Utc;
use models::roles::{Column, Entity, Gender, VoiceType};
use sea_orm::{
    ActiveValue::{self, Set},
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::Expr,
};

const DB_NAME: &str = "role-play-ai";

pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub has_more: bool,
}

pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(username: &str, password: &str, endpoint: &str) -> Result<Self> {
        let connection =
            sea_orm::Database::connect(format!("mysql://{}:{}@{}", username, password, endpoint))
                .await?;

        Self::create_database_if_not_exists(&connection).await?;

        let connection = sea_orm::Database::connect(format!(
            "mysql://{}:{}@{}/{}",
            username, password, endpoint, DB_NAME
        ))
        .await?;

        Ok(Self { connection })
    }

    pub async fn add_user(&self, username: &str, password_hash: &str, image: &str) -> Result<i32> {
        let jwt_secret = generate_jwt_secret();

        let user = models::users::ActiveModel {
            id: ActiveValue::default(),
            username: Set(username.to_string()),
            password_hash: Set(password_hash.to_string()),
            image: Set(image.to_string()),
            jwt_secret: Set(jwt_secret),
        };

        let res = models::users::Entity::insert(user)
            .exec(&self.connection)
            .await?;

        Ok(res.last_insert_id)
    }

    pub async fn verify_user(&self, username: &str, password_hash: &str) -> Result<bool> {
        let user = models::users::Entity::find()
            .filter(models::users::Column::Username.eq(username))
            .one(&self.connection)
            .await?;

        Ok(user
            .map(|u| u.password_hash == password_hash)
            .unwrap_or(false))
    }

    pub async fn get_user(&self, username: &str) -> Result<models::users::Model> {
        let user = models::users::Entity::find()
            .filter(models::users::Column::Username.eq(username))
            .one(&self.connection)
            .await?;

        user.ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    pub async fn create_conversation_table(&self, user_id: i32, role_id: i32) -> Result<()> {
        let table_name = format!("conv_{}_{}", user_id, role_id);
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS `{}` LIKE `conversation_template`",
            table_name
        );

        self.connection
            .execute(sea_orm::Statement::from_string(
                self.connection.get_database_backend(),
                sql,
            ))
            .await?;

        let existing = models::conversations::Entity::find()
            .filter(models::conversations::Column::UserId.eq(user_id))
            .filter(models::conversations::Column::RoleId.eq(role_id))
            .one(&self.connection)
            .await?;

        if existing.is_some() {
            return Ok(());
        }

        let conversation = models::conversations::ActiveModel {
            id: ActiveValue::default(),
            user_id: Set(user_id),
            role_id: Set(role_id),
            last_dialog_timestamp: Set(Utc::now().timestamp()),
        };

        models::conversations::Entity::insert(conversation)
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn list_conversations_paginated(
        &self,
        user_id: i32,
        offset: i64,
        limit: i64,
    ) -> Result<PaginatedResult<models::conversations::Model>> {
        use sea_orm::{PaginatorTrait, QueryOrder};

        let paginator = models::conversations::Entity::find()
            .filter(models::conversations::Column::UserId.eq(user_id))
            .order_by_desc(models::conversations::Column::LastDialogTimestamp)
            .paginate(&self.connection, limit as u64);

        let num_pages = paginator.num_pages().await?;
        let total = paginator.num_items().await?;

        let page_number = (offset / limit) as u64;
        let items = paginator.fetch_page(page_number).await?;
        let has_more = (page_number + 1) < num_pages;

        Ok(PaginatedResult {
            items,
            total: total as i64,
            has_more,
        })
    }

    pub async fn update_conversation_last_dialog_timestamp(
        &self,
        user_id: i32,
        role_id: i32,
        timestamp: i64,
    ) -> Result<()> {
        models::conversations::Entity::update_many()
            .col_expr(
                models::conversations::Column::LastDialogTimestamp,
                Expr::value(timestamp),
            )
            .filter(models::conversations::Column::UserId.eq(user_id))
            .filter(models::conversations::Column::RoleId.eq(role_id))
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn list_dialogs(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<Vec<models::conversation_template::Model>> {
        let table_name = format!("conv_{}_{}", user_id, role_id);
        let sql = format!("SELECT * FROM `{}` ORDER BY timestamp ASC", table_name);

        let res = self
            .connection
            .query_all(sea_orm::Statement::from_string(
                self.connection.get_database_backend(),
                sql,
            ))
            .await?;

        let dialogs = res
            .into_iter()
            .map(|row| models::conversation_template::Model {
                id: row.try_get("", "id").unwrap_or(0),
                is_user: row.try_get("", "is_user").unwrap_or(false),
                timestamp: row.try_get("", "timestamp").unwrap_or(0),
                text: row.try_get("", "text").unwrap_or_default(),
                voice: row.try_get("", "voice").ok(),
            })
            .collect::<Vec<models::conversation_template::Model>>();

        Ok(dialogs)
    }

    pub async fn list_dialogs_paginated(
        &self,
        user_id: i32,
        role_id: i32,
        offset: i64,
        limit: i64,
    ) -> Result<PaginatedResult<models::conversation_template::Model>> {
        let table_name = format!("conv_{}_{}", user_id, role_id);

        let count_sql = format!("SELECT COUNT(*) as total FROM `{}`", table_name);
        let count_res = self
            .connection
            .query_one(sea_orm::Statement::from_string(
                self.connection.get_database_backend(),
                count_sql,
            ))
            .await?;

        let total: i64 = count_res
            .and_then(|row| row.try_get("", "total").ok())
            .unwrap_or(0);

        let sql = format!(
            "SELECT * FROM `{}` ORDER BY timestamp ASC LIMIT {} OFFSET {}",
            table_name, limit, offset
        );

        let res = self
            .connection
            .query_all(sea_orm::Statement::from_string(
                self.connection.get_database_backend(),
                sql,
            ))
            .await?;

        let dialogs = res
            .into_iter()
            .map(|row| models::conversation_template::Model {
                id: row.try_get("", "id").unwrap_or(0),
                is_user: row.try_get("", "is_user").unwrap_or(false),
                timestamp: row.try_get("", "timestamp").unwrap_or(0),
                text: row.try_get("", "text").unwrap_or_default(),
                voice: row.try_get("", "voice").ok(),
            })
            .collect::<Vec<models::conversation_template::Model>>();

        let has_more = (offset + limit) < total;

        Ok(PaginatedResult {
            items: dialogs,
            total,
            has_more,
        })
    }

    pub async fn add_dialog(
        &self,
        user_id: i32,
        role_id: i32,
        is_user: bool,
        timestamp: i64,
        text: &str,
        voice: Option<String>,
    ) -> Result<i32> {
        let table_name = format!("conv_{}_{}", user_id, role_id);
        let sql = format!(
            "INSERT INTO `{}` (is_user, timestamp, text, voice) VALUES (?, ?, ?, ?)",
            table_name
        );

        let res = self
            .connection
            .execute(sea_orm::Statement::from_sql_and_values(
                self.connection.get_database_backend(),
                sql,
                vec![
                    sea_orm::Value::from(is_user),
                    sea_orm::Value::from(timestamp),
                    sea_orm::Value::from(text),
                    sea_orm::Value::from(voice),
                ],
            ))
            .await?;

        self.update_conversation_last_dialog_timestamp(user_id, role_id, timestamp)
            .await?;

        Ok(res.last_insert_id() as i32)
    }

    pub async fn add_role(
        &self,
        name: &str,
        description: &str,
        traits: &str,
        image: &str,
        gender: Gender,
        voice_type: VoiceType,
    ) -> Result<i32> {
        let role = models::roles::ActiveModel {
            id: ActiveValue::default(),
            name: Set(name.to_string()),
            description: Set(description.to_string()),
            traits: Set(traits.to_string()),
            image: Set(image.to_string()),
            gender: Set(gender),
            voice_type: Set(voice_type),
        };

        let res = models::roles::Entity::insert(role)
            .exec(&self.connection)
            .await?;

        Ok(res.last_insert_id)
    }

    pub async fn get_role(&self, role_id: i32) -> Result<models::roles::Model> {
        let role = models::roles::Entity::find_by_id(role_id)
            .one(&self.connection)
            .await?;

        role.ok_or_else(|| anyhow::anyhow!("Role not found"))
    }

    pub async fn list_roles_paginated(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<PaginatedResult<models::roles::Model>> {
        use sea_orm::{PaginatorTrait, QueryOrder};

        let paginator = models::roles::Entity::find()
            .order_by_asc(models::roles::Column::Id)
            .paginate(&self.connection, limit as u64);

        let num_pages = paginator.num_pages().await?;
        let total = paginator.num_items().await?;

        let page_number = (offset / limit) as u64;
        let items = paginator.fetch_page(page_number).await?;
        let has_more = (page_number + 1) < num_pages;

        Ok(PaginatedResult {
            items,
            total: total as i64,
            has_more,
        })
    }

    pub async fn search_roles(&self, keyword: &str) -> Result<Vec<models::roles::Model>> {
        let keyword = format!("%{}%", keyword);

        let roles = Entity::find()
            .filter(
                Expr::col(Column::Name)
                    .like(&keyword)
                    .or(Expr::col(Column::Description).like(&keyword))
                    .or(Expr::col(Column::Traits).like(&keyword)),
            )
            .all(&self.connection)
            .await?;

        Ok(roles)
    }

    pub async fn get_history(&self, user_id: i32, role_id: i32) -> Result<String> {
        let dialogs = self.list_dialogs(user_id, role_id).await?;
        let history = dialogs
            .into_iter()
            .map(|d| {
                if d.is_user {
                    format!("User: {}", d.text)
                } else {
                    format!("Assistant: {}", d.text)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(history)
    }

    pub async fn delete_conversation(&self, user_id: i32, role_id: i32) -> Result<()> {
        let table_name = format!("conv_{}_{}", user_id, role_id);
        let drop_sql = format!("DROP TABLE IF EXISTS `{}`", table_name);

        self.connection
            .execute(sea_orm::Statement::from_string(
                self.connection.get_database_backend(),
                drop_sql,
            ))
            .await?;

        models::conversations::Entity::delete_many()
            .filter(models::conversations::Column::UserId.eq(user_id))
            .filter(models::conversations::Column::RoleId.eq(role_id))
            .exec(&self.connection)
            .await?;

        Ok(())
    }
}

fn generate_jwt_secret() -> String {
    use uuid::Uuid;
    format!(
        "{}_{}",
        Uuid::new_v4().to_string().replace('-', ""),
        chrono::Utc::now().timestamp()
    )
}
