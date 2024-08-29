use super::models::{AgentEntry, AgentStatus, CreateAgent};
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

#[post("/agents/register")]
async fn create_agent(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateAgent>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    let mut agents = match data.agents_entries.lock() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let ip = match request.peer_addr() {
        Some(addr) => addr.ip().to_string(),
        None => "".to_string(),
    };
    let uuid = Uuid::new_v4();
    agents.push(AgentEntry {
        uuid,
        status: AgentStatus::Online,
        tasks: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
        last_seen_at: chrono::Utc::now().timestamp(),
        ip,
        hostname: param_obj.hostname.clone(),
        platform: param_obj.platform.clone(),
    });
    HttpResponse::Created().json(json!({"agent_uuid": uuid}))
}

#[get("/agents")]
async fn get_agents(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.agents_entries.lock().unwrap().to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_agent);
    cfg.service(get_agents);
}
