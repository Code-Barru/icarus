use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

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
pub struct HomeTemplate {
    pub title: String,
    pub welcome_text: String,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub title: String,
    pub about_text: String,
}
#[derive(Template)]
#[template(path = "content.html")]
pub struct ContentTemplate {
    pub title: String,
    pub entries: Vec<String>,
}

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate {}
