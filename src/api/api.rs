use crate::{api::todos::*};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/todos")
            .service(create_todo)
            .service(get_todo_by_id)
            .service(get_todos)
            .service(delete_todo_by_id)
            .service(update_todo_by_id),
    ); 
}
