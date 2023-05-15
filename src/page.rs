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
