use maud::{html, Markup};

pub fn layout(_body: Markup) -> Markup {
  html! {
        meta charset="utf-8";
        title { "title" }
        script src="https://unpkg.com/htmx.org@1.9.12" {}
    body {
      (_body)
    }
  }
}