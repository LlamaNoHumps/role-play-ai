use sea_orm::{
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
    prelude::StringLen,
    sea_query::{ArrayType, ValueType, ValueTypeErr},
};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Waiting,
    Started,
    Completed,
    Failed,
}

impl ValueType for TaskStatus {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(value)) => match value.as_str() {
                "waiting" => Ok(Self::Waiting),
                "started" => Ok(Self::Started),
                "completed" => Ok(Self::Completed),
                "failed" => Ok(Self::Failed),
                _ => Err(ValueTypeErr),
            },
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "TaskStatus".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

impl From<TaskStatus> for Value {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Waiting
            | TaskStatus::Started
            | TaskStatus::Completed
            | TaskStatus::Failed => Self::String(Some(Box::new(value.to_string()))),
        }
    }
}

impl TryGetable for TaskStatus {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: String = res.try_get_by(index)?;

        match value.as_str() {
            "waiting" => Ok(Self::Waiting),
            "started" => Ok(Self::Started),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err(TryGetError::DbErr(DbErr::Type(format!(
                "task status value should be one of waiting, started, completed and failed: {}",
                value
            )))),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Waiting => write!(f, "waiting"),
            Self::Started => write!(f, "started"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
        }
    }
}
