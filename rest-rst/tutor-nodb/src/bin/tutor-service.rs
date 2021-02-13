use actix_web::{web, middleware, App, HttpServer};
use std::io;
use std::sync::Mutex;
use env_logger;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let shared_data = web::Data::new(AppState {
        health_check_response: String::from("All good! I'm OK."),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![])
    });

    let app = move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
