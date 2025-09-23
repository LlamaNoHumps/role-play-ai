pub mod init;
pub mod models;
pub mod status;

use std::sync::Arc;

use anyhow::Result;
use axum::Extension;
use sea_orm::{
    ActiveValue::{self, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

const DB_NAME: &str = "role-play-ai";

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

    pub async fn add_user(&self, username: &str, password_hash: &str) -> Result<i32> {
        let user = models::users::ActiveModel {
            id: ActiveValue::default(),
            username: Set(username.to_string()),
            password_hash: Set(password_hash.to_string()),
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

    pub async fn add_voice(&self, filename: &str) -> Result<i32> {
        let voice = models::voices::ActiveModel {
            id: ActiveValue::default(),
            filename: Set(filename.to_string()),
        };

        let res = models::voices::Entity::insert(voice)
            .exec(&self.connection)
            .await?;

        Ok(res.last_insert_id)
    }

    pub async fn get_voice_filename(&self, voice_id: i32) -> Result<String> {
        let voice = models::voices::Entity::find_by_id(voice_id)
            .one(&self.connection)
            .await?;

        voice
            .map(|v| v.filename)
            .ok_or_else(|| anyhow::anyhow!("Voice not found"))
    }

    pub async fn insert_conversation(
        &self,
        role_id: Option<i32>,
        timestamp: i64,
        text: &str,
        voice_id: Option<i32>,
    ) -> Result<i32> {
        let conversation = models::conversation::ActiveModel {
            id: ActiveValue::default(),
            role_id: Set(role_id),
            timestamp: Set(timestamp),
            text: Set(text.to_string()),
            voice_id: Set(voice_id),
        };

        let res = models::conversation::Entity::insert(conversation)
            .exec(&self.connection)
            .await?;

        Ok(res.last_insert_id)
    }

    pub async fn insert_role(&self, prompt: &str) -> Result<i32> {
        let role = models::roles::ActiveModel {
            id: ActiveValue::default(),
            prompt: Set(prompt.to_string()),
        };

        let res = models::roles::Entity::insert(role)
            .exec(&self.connection)
            .await?;

        Ok(res.last_insert_id)
    }

    pub fn into_layer(self) -> Extension<Arc<Self>> {
        Extension(Arc::new(self))
    }
}
