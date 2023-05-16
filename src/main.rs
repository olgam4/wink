use std::{env, net::SocketAddr};

use argon2::{Argon2, password_hash::SaltString, PasswordHasher};
use axum::{
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Form, Router,
};
use components::wink;
use dotenvy::dotenv;
use maud::Markup;
use nanoid::nanoid;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::services::ServeDir;
use utils::parse_url;

use crate::components::{index, login_page, signup_page};

mod page;
mod components;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/:wink", get(get_wink))
        .route("/login", get(login_page))
        .route("/signup", get(signup_page))
        .route("/api/wink", post(create_wink))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(format!("Failed to bind server to {addr}").as_str());
}

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn signup(State(pool): State<PgPool>, Form(create_user): Form<CreateUser>) -> Response<String> {
    let salt = SaltString::generate(thread_rng());

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(create_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let id = nanoid!();

    let saltedhash = format!("{}{}", salt, hash);

    sqlx::query!(
        "INSERT INTO users (id, email, password) VALUES ($1, $2, $3)",
        id,
        create_user.email,
        saltedhash
    )
    .execute(&pool)
    .await
    .unwrap();

    Response::builder()
        .status(301)
        .header("HX-Location", "/")
        .header("Location", "/")
        .body("".to_string())
        .unwrap()
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

async fn login(State(pool): State<PgPool>, Form(login_user): Form<LoginUser>) -> Response<String> {
    let user = sqlx::query!(
        "SELECT * FROM users WHERE email = $1",
        login_user.email,
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    let argon2 = Argon2::default();
    let saltedhash = user.unwrap().password;
    let length = env::var("SALT_LENGTH").unwrap().parse::<usize>().unwrap();
    let salt = SaltString::new(&saltedhash[..length]).unwrap();

    let hashed_login = argon2
        .hash_password(login_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let salted_login = format!("{}{}", salt, hashed_login);

    if salted_login == saltedhash {
        println!("Logged in!");
        Response::builder()
            .status(301)
            .header("HX-Location", "/")
            .header("Location", "/")
            .body("".to_string())
            .unwrap()
    } else {
        println!("Wrong password!");
        Response::builder()
            .status(301)
            .header("HX-Location", "/login")
            .header("Location", "/login")
            .body("".to_string())
            .unwrap()
    }
}

async fn get_wink(State(pool): State<PgPool>, Path(wink): Path<String>) -> Response<String> {
    let wink = sqlx::query!("SELECT * FROM winks WHERE name = $1", wink)
        .fetch_optional(&pool)
        .await
        .expect("can't fetch wink")
        .unwrap();

    sqlx::query!(
        "UPDATE winks SET hit_counter = hit_counter + 1 WHERE name = $1",
        wink.name
    )
    .execute(&pool)
    .await
    .expect("can't update wink");

    Response::builder()
        .status(301)
        .header("HX-Location", wink.url.clone())
        .header("Location", wink.url)
        .body("".to_string())
        .unwrap()
}

#[derive(Deserialize)]
struct CreateWink {
    url: String,
}

async fn create_wink(State(pool): State<PgPool>, Form(payload): Form<CreateWink>) -> Markup {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let url = parse_url(payload.url.as_str());

    sqlx::query!(
        r#"
        INSERT INTO winks (name, url)
        VALUES ($1, $2)
        "#,
        rand_string,
        url,
    )
    .execute(&pool)
    .await
    .expect("can't insert wink");

    wink(rand_string)
}
