use super::TaskStatus;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Text,
};

impl TaskStatus {
    pub fn to_bytes(&self) -> u8 {
        match self {
            TaskStatus::Queued => 0x1,
            TaskStatus::Running => 0x2,
            TaskStatus::Completed => 0x3,
            TaskStatus::Failed => 0x4,
        }
    }
}

impl From<u8> for TaskStatus {
    fn from(byte: u8) -> Self {
        match byte {
            0x1 => TaskStatus::Queued,
            0x2 => TaskStatus::Running,
            0x3 => TaskStatus::Completed,
            0x4 => TaskStatus::Failed,
            _ => TaskStatus::Queued,
        }
    }
}

impl<DB: Backend> FromSql<Text, DB> for TaskStatus
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text = match String::from_sql(bytes) {
            Ok(text) => text,
            Err(e) => return Err(e.into()),
        };
        match text.as_str() {
            "Queued" => Ok(TaskStatus::Queued),
            "Running" => Ok(TaskStatus::Running),
            "Completed" => Ok(TaskStatus::Completed),
            "Failed" => Ok(TaskStatus::Failed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for TaskStatus
where
    str: ToSql<Text, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> serialize::Result {
        match self {
            TaskStatus::Queued => "Queued".to_sql(out),
            TaskStatus::Running => "Running".to_sql(out),
            TaskStatus::Completed => "Completed".to_sql(out),
            TaskStatus::Failed => "Failed".to_sql(out),
        }
    }
}
