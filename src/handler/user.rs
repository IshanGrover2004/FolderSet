use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::verify;
use diesel::prelude::*;

use crate::{
    database::models::{User, NewUser},
    database::schema::users::dsl::*,
    database::DbPool,
    jwt::create_jwt,
    SigninRequest,
    SignupRequest,
};

#[get("/")]
async fn handle_root() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/signup")]
pub async fn handle_signup(
    request: web::Json<SignupRequest>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");

    match NewUser::from_signup_request(&request) {
        Ok(new_user) => {
            diesel::insert_into(users)
                .values(&new_user)
                .execute(conn) 
                .expect("Error saving new user");

            HttpResponse::Ok().body("User created successfully")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error hashing password"),
    }
}

#[post("/signin")]
pub async fn handle_signin(
    request: web::Json<SigninRequest>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");

    match users
        .filter(email.eq(&request.email))
        .first::<User>(conn)
    {
        Ok(user) => {
            if verify(&request.password, &user.password).expect("Unable to verify password") {
                let token = create_jwt(&user.id).expect("Error creating token");
                HttpResponse::Ok().body(token)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
