mod configs;
mod dao;
mod model;
mod utils;

use crate::configs::Configs;
use actix_web::{middleware, web, App, HttpServer, dev::Response};
use dao::{db_connection, user_create, DB};
use model::AppState;
use utils::AppError;

async fn index(app_state : web::Data<AppState>)  {
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let c = Configs::new("config.toml".to_string());

    let pool: DB = db_connection(c.get_database()).await;
    user_create(&pool).await.map(|row| {
        println!("result: {:?}", row);
    });

    HttpServer::new(move || {
        App::new()
            .app_data(AppState { pool: pool.clone() })
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(|| async { "Test\r\n" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
