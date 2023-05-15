use maud::{Markup, html};

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            script src="https://unpkg.com/htmx.org@1.9.2" {}
        }
    }
}

pub(crate) fn page(content: Markup) -> Markup {
    html! {
        html {
            (body(content))
        }
    }
}
