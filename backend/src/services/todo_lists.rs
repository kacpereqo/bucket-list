use actix_web::{get, post, put, web, HttpResponse, Responder};

use crate::{models::todo_items::*, repository, state::AppState};

#[utoipa::path(
    tag = "Todo Lists",
    params(("id" = i64, Path, description = "Todo list id")),
    responses(
        (status = 200, body = Vec<TodoItem>),
        (status = 500, description = "Database error")
    )
)]
#[get("/{id}/items")]
async fn get_items(state: web::Data<AppState>, id: web::Path<i64>) -> impl Responder {
    match repository::todo_items::get_by_todo_list_id(&state.db, *id).await {
        Ok(todo_items) => HttpResponse::Ok().json(todo_items),
        Err(e) => {
            log::error!("get_items({}): {e}", *id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    tag = "Todo Lists",
    params(("id" = i64, Path, description = "Todo list id")),
    request_body = TodoItemCreate,
    responses(
        (status = 201, description = "Created, returns the new item id", body = i64),
        (status = 500, description = "Database error")
    )
)]
#[post("/{id}/items")]
async fn create_item(
    state: web::Data<AppState>,
    id: web::Path<i64>,
    todo_item: web::Json<TodoItemCreate>,
) -> impl Responder {
    match repository::todo_items::create(&state.db, *id, todo_item.into_inner()).await {
        Ok(new_id) => HttpResponse::Created().json(new_id),
        Err(e) => {
            log::error!("create_item({}): {e}", *id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_items);
    cfg.service(create_item);
}
