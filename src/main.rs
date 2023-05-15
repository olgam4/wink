use std::{env, net::SocketAddr};

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router, response::Response,
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
    println!("{}, {}", env::var("DATABASE_URL").unwrap(), env::var("PORT").unwrap());

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
        h1 { "Wink" }
        p { "Click the button to wink!" }
        (component_create_wink())
    })
}

fn component_wink(wink: String) -> Markup {
    html! {
        p id="wink" { (wink) }
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
    let url = sqlx::query!("SELECT url FROM winks WHERE name = $1", wink)
        .fetch_optional(&pool)
        .await
        .expect("can't fetch wink")
        .unwrap()
        .url;

    Response::builder()
        .status(301)
        .header("HX-Location", url.clone())
        .header("Location", url)
        .body("".to_string())
        .unwrap()
}

async fn create_wink(State(pool): State<PgPool>, Form(payload): Form<CreateWink>) -> Markup {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    sqlx::query!(
        r#"
        INSERT INTO winks (name, url)
        VALUES ($1, $2)
        "#,
        rand_string,
        payload.url
    )
    .execute(&pool)
    .await
    .unwrap();

    component_wink(rand_string)
}
