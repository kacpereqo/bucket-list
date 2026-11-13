use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    auth::CurrentUser,
    models::bucket_lists::{BucketList, BucketListWithLists},
    repository,
    state::AppState,
};

#[utoipa::path(
    tag = "Me",
    params(("X-User-Id" = i64, Header, description = "Logged-in user id (temporary, until login exists)")),
    responses(
        (status = 200, description = "Bucket lists the user owns or that are shared with them", body = Vec<BucketList>),
        (status = 401, description = "Missing or invalid X-User-Id header"),
        (status = 500, description = "Database error")
    )
)]
#[get("/buckets")]
async fn get_buckets(state: web::Data<AppState>, user: CurrentUser) -> impl Responder {
    match repository::bucket_lists::get_accessible_by_user_id(&state.db, user.id).await {
        Ok(buckets) => HttpResponse::Ok().json(buckets),
        Err(e) => {
            log::error!("get buckets for user {}: {e}", user.id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    tag = "Me",
    params(("X-User-Id" = i64, Header, description = "Logged-in user id (temporary, until login exists)")),
    responses(
        (status = 200, description = "Bucket lists the user can see, with their todo lists and items", body = Vec<BucketListWithLists>),
        (status = 401, description = "Missing or invalid X-User-Id header"),
        (status = 500, description = "Database error")
    )
)]
#[get("/overview")]
async fn get_overview(state: web::Data<AppState>, user: CurrentUser) -> impl Responder {
    match repository::bucket_lists::get_accessible_with_lists_and_items_by_user_id(
        &state.db, user.id,
    )
    .await
    {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            log::error!("get overview for user {}: {e}", user.id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_buckets);
    cfg.service(get_overview);
}
