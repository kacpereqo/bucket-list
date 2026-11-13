pub mod auth;
pub mod models;
pub mod repository;
pub mod router;
pub mod services;
pub mod state;

use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::Compress,
    web, App, Error,
};
use sqlx::SqlitePool;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

/// The whole application: middleware, routes and Swagger UI.
/// Used by `main.rs` and by the tests, so tests run against the real setup.
pub fn build_app(
    pool: SqlitePool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(Compress::default())
        .app_data(web::Data::new(AppState { db: pool }))
        .into_utoipa_app()
        .configure(router::configure)
        .openapi_service(|api| SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", api))
        .into_app()
}
