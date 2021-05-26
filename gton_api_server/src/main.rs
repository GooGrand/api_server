use actix_web::{App, HttpRequest, HttpServer, Responder, web, HttpResponse};

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use gton_api_server::{
    DbPool,
    ChainConfig,
};

use serde::{Serialize,Deserialize};
use diesel_migrations::run_pending_migrations;
use gton_api_server::fee_giver::routes::{
    check_vote,
    get_vote_count,
};
use gton_api_server::gton_stats::routes::{
    gton_cost,
};
use dotenv;
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct UserVote {
    id: u64,
    address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // Create connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let config = ChainConfig::from_env();

    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => print!("migration success\n"),
        Err(e)=> print!("migration error: {}\n",&e),
    };

    // Start HTTP server
    use std::sync::Arc;
    use actix_web::http;
    let config = Arc::new(config);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://alpha.graviton.one")
            .allowed_origin("https://v1.graviton.one")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .data(pool.clone())
            .data(config.clone())
            .route("/api/gton_cost", web::get().to(gton_cost))
            .route("/api/check_vote", web::post().to(check_vote))
            .route("/api/check_vote", web::get().to(get_vote_count))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
