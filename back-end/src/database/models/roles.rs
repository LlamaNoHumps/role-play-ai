use std::fmt::Display;

use sea_orm::{
    ActiveModelBehavior, ColIdx, ColumnType, DbErr, DerivePrimaryKey, DeriveRelation, EnumIter,
    PrimaryKeyTrait, QueryResult, TryGetError, TryGetable, Value,
    entity::prelude::DeriveEntityModel,
    prelude::StringLen,
    sea_query::{ArrayType, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub traits: String,
    pub image: String,
    pub gender: Gender,
    pub age_group: AgeGroup,
    pub voice_type: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn prompt(&self) -> String {
        format!(
            "你是一个角色扮演AI，以下是你的角色信息：\n{}\n{}。你的回复会被转换成语音，因此不能太长，限制在100个汉字以内。",
            self.description, self.traits
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Gender {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
}

impl ValueType for Gender {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(value)) => match value.as_str() {
                "male" => Ok(Self::Male),
                "female" => Ok(Self::Female),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Gender".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(8))
    }
}

impl From<Gender> for Value {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => Self::String(Some(Box::new("male".to_string()))),
            Gender::Female => Self::String(Some(Box::new("female".to_string()))),
        }
    }
}

impl TryGetable for Gender {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;

        match value.as_str() {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            _ => Err(TryGetError::DbErr(DbErr::Type(format!(
                "gender value should be one of male and female: {}",
                value
            )))),
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "male"),
            Self::Female => write!(f, "female"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum AgeGroup {
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "young")]
    Young,
}

impl ValueType for AgeGroup {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(value)) => match value.as_str() {
                "mature" => Ok(Self::Mature),
                "young" => Ok(Self::Young),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "AgeGroup".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(8))
    }
}

impl From<AgeGroup> for Value {
    fn from(value: AgeGroup) -> Self {
        match value {
            AgeGroup::Mature => Self::String(Some(Box::new("mature".to_string()))),
            AgeGroup::Young => Self::String(Some(Box::new("young".to_string()))),
        }
    }
}

impl TryGetable for AgeGroup {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;

        match value.as_str() {
            "mature" => Ok(Self::Mature),
            "young" => Ok(Self::Young),
            _ => Err(TryGetError::DbErr(DbErr::Type(format!(
                "voice type value should be one of mature and young: {}",
                value
            )))),
        }
    }
}

impl Display for AgeGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mature => write!(f, "mature"),
            Self::Young => write!(f, "young"),
        }
    }
}
