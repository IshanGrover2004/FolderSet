use crate::{jwt::create_jwt, AppState, SigninRequest, SignupRequest, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::verify;
use sqlx::PgPool;
use uuid::Uuid;

#[get("/")]
async fn handle_root() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/signup")]
async fn handle_signup(
    request: web::Json<SignupRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool: &PgPool = &data.pool;

    let hashed_password =
        bcrypt::hash(request.password.clone(), 7).expect("Unable to hash user password");

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        name: request.name.clone(),
        email: request.email.clone(),
        password: hashed_password,
    };

    let result = sqlx::query!(
        "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
        user.id,
        user.name,
        user.email,
        user.password
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(sqlx::Error::Database(err)) if err.constraint() == Some("users_email_key") => {
            HttpResponse::Conflict().body("Email already exists")
        }
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

#[post("/signin")]
async fn handle_signin(
    request: web::Json<SigninRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool: &PgPool = &data.pool;

    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password FROM users WHERE email = $1",
        request.email
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            if verify(&request.password, &user.password).expect("Unable to compare hashed password")
            {
                match create_jwt(&user.id) {
                    Ok(token) => HttpResponse::Ok().body(token),
                    Err(_) => HttpResponse::InternalServerError().body("Error creating token"),
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid Credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid Credentials"),
    }
}
