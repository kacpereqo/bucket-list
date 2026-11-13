use utoipa_actix_web::scope;

use crate::services;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/api")
            .service(scope::scope("/health").configure(services::health::configure))
            .service(scope::scope("/me").configure(services::me::configure))
            .service(scope::scope("/users").configure(services::users::configure))
            .service(scope::scope("/comments").configure(services::comments::configure))
            .service(scope::scope("/buckets").configure(services::bucket_lists::configure))
            .service(scope::scope("/lists").configure(services::todo_lists::configure))
            .service(scope::scope("/items").configure(services::todo_items::configure)),
    );
}
