use sea_orm::{
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait,
    entity::prelude::DeriveEntityModel,
};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub prompt: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
