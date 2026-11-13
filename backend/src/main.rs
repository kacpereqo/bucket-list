use actix_web::HttpServer;
use backend::{build_app, init_db};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:database.db?mode=rwc".into());
    let pool = init_db(&database_url)
        .await
        .expect("Cannot initialize database");

    HttpServer::new(move || build_app(pool.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
