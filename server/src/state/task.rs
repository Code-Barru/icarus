use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use shared::packets::TaskResponse;
use uuid::Uuid;

use super::GlobalState;
use crate::schema::tasks::dsl as task_dsl;
use crate::tasks::model::{Task, UpdateTask};

impl GlobalState {
    pub async fn get_tasks(&self) -> Result<Vec<Task>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;

        task_dsl::tasks.load::<Task>(&mut *conn)
    }

    pub async fn get_task(&self, id: Uuid) -> Result<Option<Task>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        task_dsl::tasks
            .filter(task_dsl::id.eq(id))
            .first::<Task>(&mut *conn)
            .optional()
    }

    pub async fn create_task(&self, task: &Task) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::insert_into(task_dsl::tasks)
            .values(task)
            .execute(&mut *conn)
    }

    pub async fn update_task(&self, task: TaskResponse) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;

        let mut update = UpdateTask {
            id: task.task_uuid,
            status: task.status,
            result: None,
        };

        if task.result != None {
            update.result = Some(String::from_utf8(task.result.unwrap()).unwrap());
        }

        diesel::update(task_dsl::tasks.filter(task_dsl::id.eq(update.id)))
            .set(update)
            .execute(&mut *conn)
    }
}
