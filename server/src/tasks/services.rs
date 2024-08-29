use super::models::{CreateTask, TaskEntry, TaskStatus, UpdateTask};
use crate::{agents::models::AgentEntry, AppState};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

#[get("/tasks")]
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let mut all_tasks = Vec::new();
    let agents = match data.agents_entries.lock() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    for agent in agents.iter() {
        all_tasks.extend(agent.tasks.iter().cloned());
    }
    HttpResponse::Ok().json(all_tasks)
}

#[get("/tasks/{taskUUID}")]
async fn get_task(data: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let agents = match data.agents_entries.lock() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    for agent in agents.iter() {
        for task in agent.tasks.iter() {
            if task.uuid == *path {
                return HttpResponse::Ok().json(task);
            }
        }
    }
    HttpResponse::NotFound().finish()
}

#[post("/tasks")]
async fn create_task(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateTask>,
) -> impl Responder {
    let mut agents = match data.agents_entries.lock() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let agent: &mut AgentEntry = match agents.iter_mut().find(|a| a.uuid == param_obj.agent) {
        Some(a) => a,
        None => return HttpResponse::NotFound().finish(),
    };

    let uuid = Uuid::new_v4();
    agent.tasks.push(TaskEntry {
        uuid,
        task_type: param_obj.task_type.clone(),
        agent: param_obj.agent.clone(),
        status: TaskStatus::Pending,
        response: "".to_string(),
        emitted_at: chrono::Utc::now().timestamp(),
        completed_at: 0,
    });
    HttpResponse::Created().json(json!({"task_uuid": uuid}))
}

#[put("/tasks/{taskUUID}")]
async fn update_task(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    param_obj: web::Json<UpdateTask>,
) -> impl Responder {
    let mut agents = match data.agents_entries.lock() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let agent: &mut AgentEntry = match agents.iter_mut().find(|a| a.uuid == param_obj.agent) {
        Some(a) => a,
        None => return HttpResponse::NotFound().finish(),
    };
    let task: Option<&mut TaskEntry> = agent.tasks.iter_mut().find(|t| t.uuid == *path);
    if let None = task {
        return HttpResponse::NotFound().finish();
    }
    match task {
        Some(t) => {
            t.status = param_obj.status.clone();
            t.response = param_obj.response.clone();
            if t.status == TaskStatus::Completed || t.status == TaskStatus::Failed {
                t.completed_at = chrono::Utc::now().timestamp();
            }
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
    cfg.service(create_task);
    cfg.service(update_task);
}
