use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use folderset::handler::{handle_root, handle_signin, handle_signup};
use folderset::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("🚀 Server started successfully on {}", addr);

    let app_state = web::Data::new(AppState {
        users: std::sync::Mutex::new(std::collections::HashMap::new()),
    });

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
