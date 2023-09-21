use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration};

#[derive(Debug)]
struct Task {
    status: String,
}

type TaskMap = Arc<Mutex<HashMap<u64, Task>>>;

#[post("/create")]
async fn create_task(data: web::Data<TaskMap>) -> impl Responder {
    let mut task_map = data.lock().unwrap();
    let task_id = task_map.len() as u64;
    task_map.insert(
        task_id,
        Task {
            status: "Pending".to_string(),
        },
    );

    // Simulate the task taking 60 seconds
    let data_clone = data.clone();
    let task_id_clone = task_id;
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(60)).await;
        let mut task_map = data_clone.lock().unwrap();
        if let Some(task) = task_map.get_mut(&task_id_clone) {
            task.status = "Completed".to_string();
        }
    });

    HttpResponse::Ok().json(format!("Task created with ID: {}", task_id))
}

#[get("/status/{task_id}")]
async fn get_task_status(data: web::Data<TaskMap>, task_id: web::Path<u64>) -> impl Responder {
    let task_map = data.lock().unwrap();
    if let Some(task) = task_map.get(&task_id) {
        HttpResponse::Ok().json(&task.status)
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_map: TaskMap = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(task_map.clone())
            .service(create_task)
            .service(get_task_status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
