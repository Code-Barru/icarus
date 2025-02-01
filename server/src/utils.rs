use crate::state::GlobalState;
use axum::{Router, handler::HandlerWithoutStateExt};
use diesel::{Connection, PgConnection};
use http::StatusCode;
use std::env;
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::{Level, info};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(conn) => {
            info!("Connected to database");
            conn
        }
        Err(e) => panic!("Error connecting to {}: {}", database_url, e),
    }
}

pub fn get_router(state: GlobalState) -> Router {
    Router::new()
        .nest("/agents", crate::agents::routes::get_router(&state))
        .nest("/dist", get_serve_dir())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

pub fn get_serve_dir() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "404 Not Found")
    }

    let service_404 = handle_404.into_service();

    let serve_dir = ServeDir::new("dist/").not_found_service(service_404);

    Router::new().fallback_service(serve_dir)
}
