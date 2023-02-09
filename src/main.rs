use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
mod todolist;
use todolist::services;  
struct AppState {
    todoList_entries:Mutex<Vec<TodoListEntry>>
}
#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: u32,
    date: String,
    title: String,
}
#[get("/")]
async fn index() -> String{
    "This is a Health Check".to_string()
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    let app_data = web::Data::new(AppState{
        todoList_entries:Mutex::new(vec![])
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}

