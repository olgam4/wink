use std::{env, net::SocketAddr};

use axum::{
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Form, Router,
};
use axum_sessions::{
    async_session::MemoryStore,
    extractors::{ReadableSession, WritableSession},
    SessionLayer,
};
use components::wink;
use dotenvy::dotenv;
use hasher::Hasher;
use maud::Markup;
use nanoid::nanoid;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::services::ServeDir;
use utils::{get_random_string, get_salt_lenght, parse_url, redirect};

use crate::components::{index, login_page, signup_page, winks_page};

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

    let store = MemoryStore::new();
    let secret = get_random_string(128);
    let session_layer = SessionLayer::new(store, secret.as_str().as_bytes()).with_secure(true);

    let app = Router::new()
        .route("/", get(index))
        .route("/:wink", get(get_wink))
        .route("/login", get(login_page))
        .route("/signup", get(signup_page))
        .route("/winks", get(winks_page))
        .route("/api/wink", post(create_wink))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/signup", post(signup))
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(session_layer)
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

async fn login(
    State(pool): State<PgPool>,
    mut session: WritableSession,
    Form(login_user): Form<LoginUser>,
) -> Response<String> {
    let result = sqlx::query!("SELECT * FROM users WHERE email = $1", login_user.email,)
        .fetch_optional(&pool)
        .await
        .expect("Failed to fetch user");

    let (password, user_id) = match result {
        Some(result) => (result.password, result.id),
        None => return redirect("/login"),
    };

    let length = get_salt_lenght();
    let salt = password.chars().rev().take(length).collect::<String>();
    let salt = salt.chars().rev().collect::<String>();

    let password = password.replace(&salt, "");

    let (hash, _) = Hasher::hash_with_salt(&login_user.password, &salt);

    if hash == password {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60 * 60 * 24))
            .expect("valid timestamp")
            .timestamp();
        let id = nanoid!();

        sqlx::query!(
            r"
             INSERT INTO sessions (id, user_id, expires)
             VALUES ($1, $2, $3)
             ",
            id,
            user_id,
            expiration,
        )
        .execute(&pool)
        .await
        .expect("Failed to insert session");

        session.insert("session_id", id).unwrap();

        redirect("/")
    } else {
        redirect("/login")
    }
}

async fn logout(mut session: WritableSession) -> Response<String> {
    session.remove("session_id");
    println!("session: {:?}", session);

    redirect("/")
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

async fn create_wink(
    State(pool): State<PgPool>,
    session: ReadableSession,
    Form(payload): Form<CreateWink>,
) -> Markup {
    let url = parse_url(payload.url.as_str());
    let name = get_random_string(8);

    sqlx::query!(
        r#"
        INSERT INTO winks (name, url)
        VALUES ($1, $2)
        "#,
        name,
        url,
    )
    .execute(&pool)
    .await
    .expect("can't insert wink");

    let session_id = session.get::<String>("session_id");
    match session_id {
        Some(_) => {
            let result = sqlx::query!(r#"SELECT * FROM sessions WHERE id = $1"#, session_id)
                .fetch_optional(&pool)
                .await
                .expect("can't fetch session");

            let user_id = match result {
                Some(result) => result.user_id,
                None => return wink(name),
            };

            sqlx::query!(
                r#"
                INSERT INTO users_winks (id, user_id, wink_id)
                VALUES ($1, $2, $3)
                         "#,
                nanoid!(),
                user_id,
                name,
            )
            .execute(&pool)
            .await
            .expect("can't insert user_wink");

            wink(name)
        }
        None => wink(name),
    }
}
