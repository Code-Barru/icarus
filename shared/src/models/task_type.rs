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
            TaskType::FileUpload => 0x2,
            TaskType::FileDownload => 0x3,
        }
    }
}

impl From<u8> for TaskType {
    fn from(byte: u8) -> Self {
        match byte {
            0x1 => TaskType::ShellCommand,
            0x2 => TaskType::FileUpload,
            0x3 => TaskType::FileDownload,
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
            "FileUpload" => Ok(TaskType::FileUpload),
            "FileDownload" => Ok(TaskType::FileDownload),
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
            TaskType::FileUpload => "FileUpload".to_sql(out),
            TaskType::FileDownload => "FileDownload".to_sql(out),
        }
    }
}
