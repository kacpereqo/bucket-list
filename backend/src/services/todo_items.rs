use actix_web::{delete, patch, put, web, HttpResponse, Responder};

use crate::{models::todo_items::*, repository, state::AppState};

#[utoipa::path(
    tag = "Todo Items",
    params(("id" = i64, Path, description = "Todo item id")),
    request_body = TodoItemUpdate,
    responses(
        (status = 204, description = "Updated"),
        (status = 404, description = "Todo item not found"),
        (status = 500, description = "Database error")
    )
)]
#[put("/{id}")]
async fn update(
    state: web::Data<AppState>,
    id: web::Path<i64>,
    todo_item: web::Json<TodoItemUpdate>,
) -> impl Responder {
    match repository::todo_items::update_by_id(&state.db, *id, todo_item.into_inner()).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("update todo item({}): {e}", *id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    tag = "Todo Items",
    params(("id" = i64, Path, description = "Todo item id")),
    responses(
        (status = 204, description = "is_done toggled"),
        (status = 404, description = "Todo item not found"),
        (status = 500, description = "Database error")
    )
)]
#[patch("/{id}/toggle")]
async fn toggle_done(state: web::Data<AppState>, id: web::Path<i64>) -> impl Responder {
    match repository::todo_items::update_toggle_done_by_id(&state.db, *id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("toggle todo item({}): {e}", *id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    tag = "Todo Items",
    params(("id" = i64, Path, description = "Todo item id")),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Todo item not found"),
        (status = 500, description = "Database error")
    )
)]
#[delete("/{id}")]
async fn delete(state: web::Data<AppState>, id: web::Path<i64>) -> impl Responder {
    match repository::todo_items::delete_by_id(&state.db, *id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("delete todo item({}): {e}", *id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(update);
    cfg.service(toggle_done);
    cfg.service(delete);
}
