use crate::state::GlobalState;
use axum::Router;
use diesel::{Connection, PgConnection};
use std::env;
use tower_http::trace::{self, TraceLayer};
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
