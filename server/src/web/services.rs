use axum::{response::IntoResponse, routing::get, Router};

use super::templates;
use super::templates::HtmlTemplate;

pub async fn render_home() -> impl IntoResponse {
    let template = templates::HomeTemplate {
        title: "Hello".to_string(),
        welcome_text: "world".to_string(),
    };
    HtmlTemplate(template)
}
pub async fn render_about() -> impl IntoResponse {
    let template = templates::AboutTemplate {
        title: "about".to_string(),
        about_text: "this is about".to_string(),
    };
    HtmlTemplate(template)
}
pub async fn render_content() -> impl IntoResponse {
    let template = templates::ContentTemplate {
        title: "content".to_string(),
        entries: vec!["entry1".to_string(), "entry2".to_string()],
    };
    HtmlTemplate(template)
}

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(render_home))
        .route("/about", get(render_about))
        .route("/content", get(render_content))
}
