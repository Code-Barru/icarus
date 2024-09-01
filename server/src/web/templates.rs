use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::agents::models::AgentEntry;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => {
                eprintln!("Failed to render template: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

#[derive(Template)]
#[template(path = "agents.html")]
pub struct AgentsTemplate {
    pub agents: Vec<AgentEntry>,
}
#[derive(Template)]
#[template(path = "agent.html")]
pub struct SingleAgentTemplate {
    pub agent: AgentEntry,
}

#[derive(Template)]
#[template(path = "tasks.html")]
pub struct TasksTemplate {
    pub tasks: Vec<crate::tasks::models::TaskEntry>,
}

#[derive(Template)]
#[template(path = "task.html")]
pub struct SingleTaskTemplate {}

#[derive(Template)]
#[template(path = "payloads.html")]
pub struct PayloadsTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}
