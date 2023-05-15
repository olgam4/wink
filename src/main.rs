use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod page;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/wink", post(create_wink))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to bind server to {addr}");
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

async fn create_wink(Form(payload): Form<CreateWink>) -> Markup {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    component_wink(rand_string)
}
