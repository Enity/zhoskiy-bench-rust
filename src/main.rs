use actix_web::{web, App, HttpServer, Responder, middleware, HttpResponse, error};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions, FromRow};
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Bear {
    name: String,
    kd_ratio: i32,
    love_suck_cock: bool,
}

async fn create(item: web::Json<Bear>, db: web::Data<MySqlPool>) -> impl Responder {
    println!("model: {:?}", &item);
    sqlx::query(
        r#"
            INSERT INTO bear ( name, kd_ratio, love_suck_cock )
            VALUES ( ?, ?, ? )
        "#
    )
    .bind(&item.name)
    .bind(&item.kd_ratio)
    .bind(&item.love_suck_cock)
    .execute(db.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(item.0) // <- send response
}

async fn ping() -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let port = std::env::var("PORT").unwrap();
    let host = std::env::var("HOST").unwrap();

    let db_pool = MySqlPoolOptions::new()
        .connect(&database_url).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .data(web::JsonConfig::default().limit(4096).error_handler(|e, _| {
                error::ErrorBadRequest(e)
            }))
            .service(
                web::scope("/api")
                    .route("/ping", web::get().to(ping))
                    .route("/create", web::post().to(create))
        )
    })
    .bind(format!("{}:{}", &host, &port))?;

    println!("Starting on {}", &port);
    server.run().await
}