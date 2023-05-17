use maud::{html, Markup};

use crate::page;

pub async fn index() -> Markup {
    page::page(html! {
        nav {
            div {
                a href="/login" { "Login" }
                a href="/signup" { "Sign up" }
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
            div.card {
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
            div.card {
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

pub fn wink(wink: String) -> Markup {
    html! {
        div.wink {
            p id="wink" { "gow.ink/"(wink) }
            clipboard-copy.copy _="on click toggle .hidden on .copy-icon" for="copy-wink" {
                i class="fa-regular fa-copy copy-icon" {}
                i class="hidden green fa-regular fa-circle-check copy-icon" {}
            }
            span.hidden id="copy-wink" { "https://www.gow.ink/"(wink) }
        }
    }
}
