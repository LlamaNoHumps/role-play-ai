use sea_orm::{
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait,
    entity::prelude::DeriveEntityModel,
};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub last_dialog_timestamp: i64,
    #[sea_orm(column_type = "Text")]
    pub history: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
