use std::{env, net::SocketAddr};

use axum::{
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Form, Router,
};
use components::wink;
use dotenvy::dotenv;
use hasher::Hasher;
use maud::Markup;
use nanoid::nanoid;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::services::ServeDir;
use utils::{parse_url, redirect, get_salt_lenght, get_random_string};

use crate::components::{index, login_page, signup_page};

mod components;
mod hasher;
mod page;
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

    println!("Listening on {addr}");

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

async fn signup(
    State(pool): State<PgPool>,
    Form(create_user): Form<CreateUser>,
) -> Response<String> {
    let id = nanoid!();

    let (hash, salt) = Hasher::hash(&create_user.password);
    let salted_hash = format!("{hash}{salt}");

    sqlx::query!(
        "INSERT INTO users (id, email, password) VALUES ($1, $2, $3)",
        id,
        create_user.email,
        salted_hash,
    )
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    redirect("/")
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

async fn login(State(pool): State<PgPool>, Form(login_user): Form<LoginUser>) -> Response<String> {
    let password = sqlx::query!("SELECT password FROM users WHERE email = $1", login_user.email,)
        .fetch_optional(&pool)
        .await
        .expect("Failed to fetch user");

    let password = match password {
        Some(password) => password.password,
        None => return redirect("/login"),
    };

    let length = get_salt_lenght();
    let salt = password.chars().rev().take(length).collect::<String>();
    let salt = salt.chars().rev().collect::<String>();

    let password = password.replace(&salt, "");

    let (hash, _) = Hasher::hash_with_salt(&login_user.password, &salt);

    if hash == password {
        redirect("/")
    } else {
        redirect("/login")
    }
}

async fn get_wink(State(pool): State<PgPool>, Path(wink): Path<String>) -> Response<String> {
    let wink = sqlx::query!("SELECT * FROM winks WHERE name = $1", wink)
        .fetch_optional(&pool)
        .await
        .expect("can't fetch wink")
        .expect("wink not found");

    sqlx::query!(
        "UPDATE winks SET hit_counter = hit_counter + 1 WHERE name = $1",
        wink.name
    )
    .execute(&pool)
    .await
    .expect("can't update wink");

    redirect(&wink.url)
}

#[derive(Deserialize)]
struct CreateWink {
    url: String,
}

async fn create_wink(State(pool): State<PgPool>, Form(payload): Form<CreateWink>) -> Markup {
    let url = parse_url(payload.url.as_str());
    let rand_string = get_random_string(8);

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
