use maud::{html, Markup};

use crate::page;

pub async fn index() -> Markup {
    page::page(html! {
        a href="/login" { "Login" }
        a href="/signup" { "Sign up" }
        img height="100" src="/static/wink.png" {}
        p { "Click the button to wink!" }
        (create_wink())
    })
}

pub async fn login_page() -> Markup {
    page::page(login())
}

pub async fn signup_page() -> Markup {
    page::page(html! {
        form method="POST" action="/api/signup" {
            input type="email" name="email" placeholder="Email" {}
            input type="password" name="password" placeholder="Password" {}
            button type="submit" { "Sign up" }
        }
    })
}

pub fn wink(wink: String) -> Markup {
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

pub fn create_wink() -> Markup {
    html! {
        form hx-post="/api/wink" hx-target="#wink" {
            input type="text" name="url" placeholder="URL" {}
            button type="submit" { "Wink!" }
        }
        p id="wink" {}
    }
}

pub fn login() -> Markup {
    html! {
        form hx-post="/api/login" {
            input type="text" name="username" placeholder="Username" {}
            input type="password" name="password" placeholder="Password" {}
            button type="submit" { "Login" }
        }
    }
}