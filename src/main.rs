use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use folderset::handler::{handle_root, handle_signin, handle_signup};
use sqlx::postgres::PgPoolOptions;
use folderset::AppState;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("🚀 Server started successfully on {}", addr);

    
    let app_state = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(handle_root)
            .service(handle_signup)
            .service(handle_signin)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
