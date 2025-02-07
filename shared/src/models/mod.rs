use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};

mod connection_type;
mod task_status;
mod task_type;

#[derive(Debug, Clone, Serialize, Deserialize, diesel::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TaskType {
    ShellCommand,
}

#[derive(Debug, Clone, Serialize, diesel::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TaskStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    Main,
    Update,
    FileUpload,
    FileDownload,
}
