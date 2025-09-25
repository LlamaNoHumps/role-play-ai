use sea_orm::{
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait,
    entity::prelude::DeriveEntityModel,
};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub image: String,
    pub jwt_secret: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
