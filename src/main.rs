use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;


use folderset::database::establish_connection;
use folderset::handler::user::{handle_root, handle_signin, handle_signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let pool = establish_connection();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("🚀 Server started successfully on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(handle_root)
            .service(handle_signup)
            .service(handle_signin)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
