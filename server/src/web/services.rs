use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::get, Router};
use uuid::Uuid;

use crate::AppState;

use super::templates;
use super::templates::HtmlTemplate;

#[allow(unused_variables)]
async fn render_home(state: State<AppState>) -> impl IntoResponse {
    let template = templates::HomeTemplate {};
    HtmlTemplate(template).into_response()
}

async fn render_agents(state: State<AppState>) -> impl IntoResponse {
    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let template = templates::AgentsTemplate {
        agents: agents.clone(),
    };
    HtmlTemplate(template).into_response()
}

async fn render_single_agent(Path(uuid): Path<Uuid>, state: State<AppState>) -> impl IntoResponse {
    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let agent = match agents.iter().find(|agent| agent.uuid == uuid) {
        Some(agent) => agent,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let template = templates::SingleAgentTemplate {
        agent: agent.clone(),
    };
    HtmlTemplate(template).into_response()
}
#[allow(unused_variables)]
async fn render_tasks(_state: State<AppState>) -> impl IntoResponse {
    let template = templates::TasksTemplate {};
    HtmlTemplate(template).into_response()
}

async fn render_single_task(Path(_uuid): Path<Uuid>, _state: State<AppState>) -> impl IntoResponse {
    let template = templates::SingleTaskTemplate {};
    HtmlTemplate(template).into_response()
}

async fn render_payloads(_state: State<AppState>) -> impl IntoResponse {
    let template = templates::PayloadsTemplate {};
    HtmlTemplate(template).into_response()
}

async fn render_about(_state: State<AppState>) -> impl IntoResponse {
    let template = templates::AboutTemplate {};
    HtmlTemplate(template).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(render_home))
        .with_state(state.clone())
        .route("/agents", get(render_agents))
        .with_state(state.clone())
        .route("/agents/:uuid", get(render_single_agent))
        .with_state(state.clone())
        .route("/tasks", get(render_tasks))
        .with_state(state.clone())
        .route("/tasks/:uuid", get(render_single_task))
        .with_state(state.clone())
        .route("/payloads", get(render_payloads))
        .with_state(state.clone())
        .route("/about", get(render_about))
        .with_state(state.clone())
}
