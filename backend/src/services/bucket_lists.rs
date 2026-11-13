use actix_web::{post, put, web, HttpResponse, Responder};

use crate::{
    auth::CurrentUser,
    models::bucket_lists::{BucketListCreate, BucketListUpdate},
    repository,
    state::AppState,
};

#[utoipa::path(
    tag = "Bucket Lists",
    params(("X-User-Id" = i64, Header, description = "Logged-in user id (temporary, until login exists)")),
    request_body = BucketListCreate,
    responses(
        (status = 201, description = "Created, returns the new bucket list id", body = i64),
        (status = 401, description = "Missing or invalid X-User-Id header"),
        (status = 500, description = "Database error")
    )
)]
#[post("")]
async fn create(
    state: web::Data<AppState>,
    user: CurrentUser,
    bucket_list: web::Json<BucketListCreate>,
) -> impl Responder {
    match repository::bucket_lists::create(&state.db, user.id, bucket_list.into_inner()).await {
        Ok(new_id) => HttpResponse::Created().json(new_id),
        Err(e) => {
            log::error!("create bucket list for user {}: {e}", user.id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    tag = "Bucket Lists",
    params(
        ("id" = i64, Path, description = "Bucket list id"),
        ("X-User-Id" = i64, Header, description = "Logged-in user id (temporary, until login exists)")
    ),
    request_body = BucketListUpdate,
    responses(
        (status = 204, description = "Updated"),
        (status = 401, description = "Missing or invalid X-User-Id header"),
        (status = 404, description = "Bucket list not found or not owned by the user"),
        (status = 500, description = "Database error")
    )
)]
#[put("/{id}")]
async fn update(
    state: web::Data<AppState>,
    user: CurrentUser,
    id: web::Path<i64>,
    bucket_list: web::Json<BucketListUpdate>,
) -> impl Responder {
    match repository::bucket_lists::update_by_id(&state.db, *id, user.id, bucket_list.into_inner())
        .await
    {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("update bucket list({}) for user {}: {e}", *id, user.id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(create);
    cfg.service(update);
}
