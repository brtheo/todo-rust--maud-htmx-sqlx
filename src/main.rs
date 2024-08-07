mod layout;
use actix_web::{Responder, HttpResponse,get, post, web::{Data, Form, Path, ServiceConfig}, HttpRequest, Result as ActixResult};
use awc::Client;
use shuttle_actix_web::ShuttleActixWeb;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, json};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use chrono::Utc;
use std::{fmt, str::FromStr};
struct State {
  db: SqlitePool,
}

#[derive(Deserialize, Serialize)]
struct FormData {
  test: String,
}
#[derive(Debug, PartialEq,Deserialize, Serialize,Clone)]
enum Priority {
    Low,
    Medium,
    High,
}

impl FromStr for Priority {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.to_lowercase().as_str() {
          "low" => Ok(Priority::Low),
          "medium" => Ok(Priority::Medium),
          "high" => Ok(Priority::High),
          _ => Err(()),
      }
  }
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Todo {
  id: Option<i16>,
  title: String,
  description: String,
  due_date: Option<String>,
  priority: Priority,
  status: Option<String>,
  created_at: Option<String>,
  updated_at: Option<String>
}

#[get("/")]
async fn index(state: Data<State>) -> ActixResult<Markup> {

  Ok(html! {
    h1 { "hello world"}
  })
}
#[get("/insert-todo")]
async fn insert_todoget(state: Data<State>) -> impl Responder {
  let todo= Todo {
    id: None,
    description: String::from("some todo"),
    title: String::from("Todo1"),
    due_date: None,
    priority: Priority::Medium,
    status: None,
    created_at: None,
    updated_at: None,
  };
  match insert_todo(&state.db, todo).await {
    Ok(id) => HttpResponse::Ok().json(json!({ "id": id })),
    Err(_) => HttpResponse::InternalServerError().finish(),
}
}




async fn insert_todo(pool: &SqlitePool, todo: Todo) -> Result<i64,sqlx::Error> {
  let now = Utc::now().to_rfc3339();
  let todo_priority = todo.priority.to_string();
  let result = sqlx::query!(
      r#"
      INSERT INTO todos (title, description, due_date, priority, status, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?)
      "#,
      todo.title,
      todo.description,
      todo.due_date,
      todo_priority,
      todo.status,
      now,
      now
  )
  .execute(pool)
  .await?;

  Ok(result.last_insert_rowid())
}

// #[get("/{id}")]
// async fn get_id(req: HttpRequest, path: Path<String>) -> ActixResult<Markup> {
//   let url = req.url_for("jplaceholder", [path.into_inner()]).unwrap();
//   let client = Client::default();

//    // Create request builder and send request
//    let response = 
//     client.get(url.as_str())
//       .send()     // <- Send request
//       .await; 
//     let todo = response.unwrap().json::<Todo>().await.unwrap();
//     let debug = todo.clone();
//     println!("Response: {:?}", todo);
//     // todo.completed = true;
//   Ok(
//     layout::layout(html! {
//       form hx-post="/theo" hx-trigger="submit" hx-target="body"  hx-swap="outerHTML" {
//         input name="test" {}
//         button type="submit"  {(todo.id.unwrap())}
//         h1 {(todo.title)}
//         input type="checkbox" checked[todo.status.unwrap() == "completed"] {}
//         pre {(to_string_pretty(&debug)?)}
//       }
//     })
//   )
// }

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  let database_url = "sqlite://todos.db";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool");
  let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(Data::new(State { db: pool.clone() }));
    cfg.service(index);
    cfg.service(insert_todoget);
    // cfg.service(get_id);
    cfg.external_resource("jplaceholder", "https://jsonplaceholder.typicode.com/todos/{id}");
  };

  Ok(config.into())
}
