use std::{env, net::SocketAddr};

use axum::{
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Form, Router,
};
use axum_macros::debug_handler;
use dotenvy::dotenv;
use maud::{html, Markup};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::services::ServeDir;

mod page;

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
        .route("/api/wink", post(create_wink))
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(format!("Failed to bind server to {addr}").as_str());
}

async fn index() -> Markup {
    page::page(html! {
        img height="100" src="/static/wink.png" {}
        p { "Click the button to wink!" }
        (component_create_wink())
    })
}

fn component_wink(wink: String) -> Markup {
    html! {
        div class="wink" {
            p id="wink" { "gow.ink/"(wink) }
            clipboard-copy _="on click toggle .hidden on .copy-icon" class="copy" for="copy-wink" {
                i class="fa-regular fa-copy copy-icon" {}
                i class="hidden green fa-regular fa-circle-check copy-icon" {}
            }
            span id="copy-wink" { "https://www.gow.ink/"(wink) }
        }
    }
}

fn component_create_wink() -> Markup {
    html! {
        form hx-post="/api/wink" hx-target="#wink" {
            input type="text" name="url" placeholder="URL" {}
            button type="submit" { "Wink!" }
        }
        p id="wink" {}
    }
}

#[derive(Serialize)]
struct Wink {
    name: String,
}

#[derive(Deserialize)]
struct CreateWink {
    url: String,
}

#[debug_handler]
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

fn parse_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
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
    .unwrap();

    component_wink(rand_string)
}
