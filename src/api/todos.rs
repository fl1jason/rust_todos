use crate::{models::todo::Todo, repository::todos::TodoService};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("")]
pub async fn create_todo(db: web::Data<TodoService>, new_todo: web::Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }   
}

#[get("/{id}")]
pub async fn get_todo_by_id(db: web::Data<TodoService>, id: web::Path<String>) -> HttpResponse {
    let todo = db.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[get("")]
pub async fn get_todos(db: web::Data<TodoService>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[delete("/{id}")]
pub async fn delete_todo_by_id(db: web::Data<TodoService>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo_by_id(&id);
    match todo {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/{id}")]
pub async fn update_todo_by_id(
    db: web::Data<TodoService>,
    id: web::Path<String>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}
