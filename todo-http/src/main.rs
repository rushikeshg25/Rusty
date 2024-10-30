use actix_web::{web,App,HttpResponse,HttpServer,Responder};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo{
    id:u64,
    title:String,
    completed:bool
}

struct AppState{
    todo_list:Mutex<Vec<Todo>>
}

async fn create_todo(app_state:web::Data<AppState>,todo: web::Json<Todo>) -> impl Responder{
    let mut todo_list=app_state.todo_list.lock().unwrap();
    let new_todo=Todo{
        id:todo_list.len() as u64 +1,
        title:todo.title.clone(),
        completed:todo.completed,
    };
    todo_list.push(new_todo.clone());
    HttpResponse::Ok().json(new_todo)
}

async fn delete_todo(app_state: web::Data<AppState>, todo_id: web::Path<u64>) -> impl Responder {
    let mut todo_list = app_state.todo_list.lock().unwrap();
    let id = *todo_id;
    
    if let Some(pos) = todo_list.iter().position(|todo| todo.id == id) {
        todo_list.remove(pos);
        HttpResponse::Ok().json(format!("Todo with ID {} deleted", id))
    } else {
        HttpResponse::NotFound().json(format!("Todo with ID {} not found", id))
    }
}



async fn get_todos(app_state:web::Data<AppState>)-> impl Responder{
    let todo_list=app_state.todo_list.lock().unwrap();
    HttpResponse::Ok().json(todo_list.to_vec())
}


#[actix_web::main]
async fn main()->std::io::Result<()>{
    let app_state=web::Data::new(AppState{
        todo_list:Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos",web::post().to(create_todo))
            .route("/todos",web::get().to(get_todos))
            .route("/todos/{id}", web::delete().to(delete_todo))

    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
