use sea_orm::{
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait,
    entity::prelude::DeriveEntityModel,
};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub timestamp: i64,
    pub text: String,
    pub tts_id: Option<i32>,
    pub image_id: Option<i32>,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
