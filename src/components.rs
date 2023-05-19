use axum::extract::State;
use axum_sessions::extractors::ReadableSession;
use base64::{engine::general_purpose, Engine};
use maud::{html, Markup};
use qrcode_generator::QrCodeEcc;
use sqlx::PgPool;

use crate::page;

pub async fn index(State(pool): State<PgPool>, session: ReadableSession) -> Markup {
    let session_id = session.get::<String>("session_id");
    let result = sqlx::query!(r#"SELECT * FROM sessions WHERE id = $1"#, session_id)
        .fetch_optional(&pool)
        .await
        .expect("can't fetch session");

    let is_logged_in = match result {
        Some(result) => result.expires > chrono::Local::now().timestamp(),
        None => false,
    };

    page::page(html! {
        nav {
            div {
                @match is_logged_in {
                    true => {
                        a id="winks" hx-get="/winks" hx-target="body" { "My Winks" }
                        a id="logout-button" hx-post="/api/logout" hx-target="body" { "Logout" }
                    },
                    false => {
                        a id="login-button" href="/login" { "Login" }
                        a id="signup-button" href="/signup" { "Sign up" }
                    }
                }
            }
        }
        img height="100" src="/static/wink.png" {}
        (create_wink())
    })
}

pub fn create_wink() -> Markup {
    html! {
        div.band {
            p { "Wink a link to a friend" }
            form hx-post="/api/wink" hx-target="#wink" {
                input type="text" name="url" placeholder="URL" {}
                button type="submit" { "Wink!" }
            }
            p id="wink" {}
        }
    }
}

pub async fn login_page() -> Markup {
    page::page(html! {
        div.full.gradient-eye-colors {
            div.card.form-auth {
                h1 { "Welcome back" }
                form method="POST" action="/api/login" {
                    input type="text" name="email" placeholder="Email" {}
                    input type="password" name="password" placeholder="Password" {}
                    button type="submit" {
                        i class="fa-solid fa-arrow-right white" {}
                    }
                }
            }
        }
    })
}

pub async fn signup_page() -> Markup {
    page::page(html! {
        div.full.gradient-eye-colors {
            div.card.form-auth {
                h1 { "Join us" }
                form method="POST" action="/api/signup" {
                    input type="email" name="email" placeholder="Email" {}
                    input type="password" name="password" placeholder="Password" {}
                    button type="submit" {
                        i class="fa-solid fa-arrow-right white" {}
                    }
                }
            }
        }
    })
}

pub struct Wink {
    name: String,
    url: String,
    hit_counter: i32,
}

pub async fn winks_page(State(pool): State<PgPool>, session: ReadableSession) -> Markup {
    let session = sqlx::query!(
        r#"SELECT * FROM sessions WHERE id = $1"#,
        session.get::<String>("session_id")
    )
    .fetch_optional(&pool)
    .await
    .expect("can't fetch session");

    let session = match session {
        Some(session) => session,
        None => {
            return page::page(html! {
                div.full.gradient-eye-colors {
                    div.card.form-auth.center {
                        h1 { "You are not logged in" }
                        a href="/" { "Go back" }
                    }
                }
            });
        }
    };

    let wink_objects = sqlx::query!(
        r#"SELECT * FROM winks WHERE name IN (SELECT wink_id FROM users_winks WHERE user_id = $1)"#,
        session.user_id
    )
    .fetch_all(&pool)
    .await
    .expect("can't fetch winks");

    let winks = wink_objects
        .iter()
        .map(|wink| Wink {
            name: wink.name.clone(),
            url: wink.url.clone(),
            hit_counter: wink.hit_counter,
        })
        .collect::<Vec<Wink>>();

    winks_component(winks)
}

pub fn winks_component(winks: Vec<Wink>) -> Markup {
    page::page(html! {
        div.full.gradient-eye-colors {
            div.back {
                a href="/" {
                    i class="fa-solid fa-arrow-left white big" {}
                }
            }
            div.card.overflow-y-scroll {
                h1 { "Your winks" }
                @for wink in winks {
                    p { (wink.name) " => " (wink.url) " - " (wink.hit_counter) }
                }
            }
        }
    })
}

pub fn wink(wink: String) -> Markup {
    let wink_link = format!("https://www.gow.ink/{}", wink);
    html! {
        div.wink {
            p id="wink" { "gow.ink/"(wink) }
            clipboard-copy.copy _="on click toggle .hidden on .copy-icon" for="copy-wink" {
                i class="fa-regular fa-copy copy-icon" {}
                i class="hidden green fa-regular fa-circle-check copy-icon" {}
            }
            span.hidden id="copy-wink" { (wink_link) }
            (qr_code(&wink_link))
        }
    }
}

fn qr_code(link: &String) -> Markup {
    let result: Vec<u8> = qrcode_generator::to_png_to_vec(link, QrCodeEcc::Low, 1024).unwrap();
    let b64 = general_purpose::STANDARD.encode(&result);

    html! {
        img width="100" src=(format!("data:image/png;base64,{}", b64)) {}
    }
}
