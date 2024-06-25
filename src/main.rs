mod health;
mod repository;
mod user;
mod v1;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use repository::MemoryRepository;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, Mutex,
};

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

// Handler para la ruta principal
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hola Ruts")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8001".to_string());
    let address = format!("127.0.0.1:{}", port);

    let thread_counter = Arc::new(AtomicU16::new(1));

    // Construir el repositorio compartido envuelto en Arc y Mutex
    let repo = Arc::new(Mutex::new(MemoryRepository::default()));

    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("Starting thread {}", thread_index);

        App::new()
            .app_data(web::Data::new(thread_index))
            .app_data(web::Data::new(repo.clone()))
            .route("/", web::get().to(index))
            .configure(v1::service)
            .configure(health::service)
            .service(hello)
    })
    .bind(&address)
    .unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server in port {}: {:?}", port, err))
    .run()
    .await
}
