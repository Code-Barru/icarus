use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{
    models::{TaskStatus, TaskType},
    packets::TaskRequest,
};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel()]

pub struct Task {
    pub id: Uuid,
    pub agent_uuid: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub parameters: Option<String>,
    pub result: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Task {
    pub fn new(agent_uuid: Uuid, tasktype: TaskType, parameters: Option<String>) -> Self {
        Task {
            id: Uuid::new_v4(),
            agent_uuid,
            task_type: tasktype,
            status: TaskStatus::Queued,
            parameters,
            result: None,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        }
    }

    pub fn to_packet(&self) -> TaskRequest {
        TaskRequest::new(self.id, self.task_type.clone(), self.parameters.clone())
    }
}

impl From<CreateTask> for Task {
    fn from(create_task: CreateTask) -> Self {
        Task {
            id: Uuid::new_v4(),
            agent_uuid: create_task.agent_uuid,
            task_type: create_task.task_type,
            status: TaskStatus::Queued,
            parameters: create_task.parameters,
            result: None,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub agent_uuid: Uuid,
    pub task_type: TaskType,
    pub parameters: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTask {
    pub id: Uuid,
    pub status: TaskStatus,
    pub result: Option<String>,
}
