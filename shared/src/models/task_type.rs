use super::TaskType;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Text,
};

impl TaskType {
    pub fn to_bytes(&self) -> u8 {
        match self {
            TaskType::ShellCommand => 0x1,
        }
    }
}

impl From<u8> for TaskType {
    fn from(byte: u8) -> Self {
        match byte {
            0x1 => TaskType::ShellCommand,
            _ => TaskType::ShellCommand,
        }
    }
}

impl<DB: Backend> FromSql<Text, DB> for TaskType
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text = match String::from_sql(bytes) {
            Ok(text) => text,
            Err(e) => return Err(e.into()),
        };
        match text.as_str() {
            "ShellCommand" => Ok(TaskType::ShellCommand),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for TaskType
where
    str: ToSql<Text, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> serialize::Result {
        match self {
            TaskType::ShellCommand => "ShellCommand".to_sql(out),
        }
    }
}
