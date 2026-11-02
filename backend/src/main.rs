use actix_web::{get, web, App, HttpServer};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(
    responses(
        (status = 200, body = HelloResponse)
    )
)]
#[get("/hello")]
async fn hello() -> web::Json<HelloResponse> {
    web::Json(HelloResponse {
        message: "Hello, world!".into(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .into_utoipa_app()
            .service(hello)
            .openapi_service(|api| {
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}