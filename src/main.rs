mod layout;
use actix_web::{get, post, web::{Form, Path, ServiceConfig, Data}, HttpRequest, Result as ActicxResult};
use awc::Client;
use shuttle_actix_web::ShuttleActixWeb;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Deserialize, Serialize)]
struct FormData {
  test: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Todo {
  user_id: i16,
  id: i16,
  title: String,
  completed: bool
}

#[post("/{name}")]
async fn hello_world(path: Path<String>, form: Form<FormData>) -> ActicxResult<Markup> {

  let name = path.into_inner();
  Ok(html! {
    h1 { (form.test)}
    h2 { "My name is "(name) }
  })
}
#[get("/{id}")]
async fn index(req: HttpRequest, path: Path<String>) -> ActicxResult<Markup> {
  let url = req.url_for("jplaceholder", [path.into_inner()]).unwrap();
  let client = Client::default();

   // Create request builder and send request
   let response = 
    client.get(url.as_str())
      .send()     // <- Send request
      .await; 
    let todo = response.unwrap().json::<Todo>().await.unwrap();

    println!("Response: {:?}", todo);
    // todo.completed = true;
  Ok(
    layout::layout(html! {
      form hx-post="/theo" hx-trigger="submit" hx-target="body"  hx-swap="outerHTML" {
        input name="test" {}
        button type="submit"  {(todo.user_id)}
        h1 {(todo.title)}
        input type="checkbox" checked[todo.completed] {}
        pre {(to_string_pretty(&todo)?)}
      }
    })
  )
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  let config = move |cfg: &mut ServiceConfig| {
    cfg.service(hello_world);
    cfg.service(index);
    cfg.external_resource("jplaceholder", "https://jsonplaceholder.typicode.com/todos/{id}");
  };

  Ok(config.into())
}
