use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::Mutex;
mod tasks;
use agents::models::AgentEntry;

mod agents;

struct AppState {
    agents_entries: Mutex<Vec<AgentEntry>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        agents_entries: Mutex::new(Vec::new()),
    });
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(tasks::services::config)
            .configure(agents::services::config)
            .wrap(Logger::new("%a \"%r\" - %s"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
