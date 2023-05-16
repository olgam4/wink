use maud::{html, Markup};

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)

            script src="https://unpkg.com/htmx.org@1.9.2" {}
        }
    }
}

fn head() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { "Wink" }

            link rel="shortcut icon" type="image/x-icon" href="/static/favicon.ico" {}
            link rel="stylesheet" href="/static/reset.css" {}
            link rel="stylesheet" href="/static/main.css" {}
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" {}

            script src="https://unpkg.com/@github/clipboard-copy-element@latest" defer="" {}
            script src="https://unpkg.com/hyperscript.org@0.9.8" {}
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
