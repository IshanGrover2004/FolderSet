use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::verify;

use crate::{AppState, SigninRequest, SignupRequest, jwt::create_jwt};

#[get("/")]
async fn handle_root() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/signup")]
async fn handle_signup(
    request: web::Json<SignupRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut app_data = data.users.lock().expect("Unable to access the DB");

    let hashed_password =
        bcrypt::hash(request.password.clone(), 7).expect("Unable to hash user password");

    if app_data.contains_key(&request.email) {
        HttpResponse::Conflict().body("Username already exists")
    } else {
        app_data.insert(request.email.clone(), hashed_password);
        println!("SignedUp successfully");
        HttpResponse::Ok().body("User created successfully")
    }
}

#[post("/signin")]
async fn handle_signin(
    request: web::Json<SigninRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let app_data = data.users.lock().expect("Unable to access the DB");
    if let Some(hashed_password) = app_data.get(&request.email) {
        if verify(request.password.clone(), hashed_password)
            .expect("Unable to compare hashed password")
        {
            println!("SignedIn successfully");

            // Generate JWT key here & send as response
            match create_jwt(&request.email) {
                Ok(token) => HttpResponse::Ok().body(token),
                Err(_) => HttpResponse::InternalServerError().body("Error creating token"),
            }
        } else {
            HttpResponse::Unauthorized().body("Invalid Credentials")
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid Credentials")
    }
}
