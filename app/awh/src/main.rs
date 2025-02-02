use std::env;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use user::user_controller::user_controller;

mod user;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db_connection = Database::connect(db_url).await.unwrap();
    Migrator::up(&db_connection, None).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_header().allow_any_method().allow_any_origin().supports_credentials();
        App::new()
            .app_data(Data::new(db_connection.clone()))
            .wrap(cors)
            .service(user_controller())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}