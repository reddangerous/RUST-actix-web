use actix_web::{get,post, delete, web,put, HttpResponse, Responder };

use crate::{AppState, TodoListEntry};
use super::models::{CreateEntryData, UpdateEntryData};


#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
   HttpResponse::Ok().json(data.todoList_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj:web::Json<CreateEntryData>) -> impl Responder {
    let mut todolist_entries = data.todoList_entries.lock().unwrap();
    let mut max_id: u32  = 0;
   for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    };
    todolist_entries.push(TodoListEntry{
        id: max_id + 1,
        date: param_obj.date.clone(),
        title: param_obj.title.clone(),
    });
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("/todolist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path:web::Path<u32>, param_obj:web::Json<UpdateEntryData>) -> impl Responder {
    let id:u32 = path.into_inner();
    let mut todolist_entries = data.todoList_entries.lock().unwrap();
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
        break;
        }
    }
    HttpResponse::Ok().json(todolist_entries.to_vec())
    }

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path:web::Path<u32>) -> impl Responder {
    let id:u32 = path.into_inner();
    let mut todolist_entries = data.todoList_entries.lock().unwrap();
    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries);
    cfg.service(create_entry);
    cfg.service(update_entry);
    cfg.service(delete_entry);
}
   