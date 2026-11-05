use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};

use crate::{repository, state::AppState};

#[utoipa::path(
    tag = "Health",
    responses(
        (status = 200)
    )
)]
#[get("/db")]
async fn get_health(state: web::Data<AppState>) -> impl Responder {
    let unixepoch = repository::health::get_unixepoch_as_json_string(&state.db)
        .await
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(unixepoch)
}

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_health);
}
