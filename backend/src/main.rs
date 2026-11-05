pub mod repository;
pub mod router;
pub mod services;
pub mod state;

use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use state::AppState;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_url = "sqlite:database .db?mode=rwc";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Cannot connect to db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration error");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .into_utoipa_app()
            .configure(router::configure)
            .openapi_service(|api| {
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
