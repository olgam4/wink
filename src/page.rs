use maud::{Markup, html};

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            
            script src="https://unpkg.com/htmx.org@1.9.2" {}
        }
    }
}

fn head () -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { "Wink" }
            link rel="shortcut icon" type="image/x-icon" href="/static/favicon.ico" {}
            link rel="stylesheet" href="/static/reset.css" {}
            link rel="stylesheet" href="/static/main.css" {}
        }
    }
}

pub(crate) fn page(content: Markup) -> Markup {
    html! {
        html {
            (head())
            (body(content))
        }
    }
}
