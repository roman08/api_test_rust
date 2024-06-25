mod repository;
mod user;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use repository::{MemoryRepository, RepositoryInjector};
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};
use uuid::Uuid;

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

// Handler para la ruta de salud
async fn health_check(thread_index: web::Data<u16>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("thread-id", thread_index.to_string()))
        .finish()
}

// Handler para obtener un usuario
async fn get_user(user_id: web::Path<Uuid>, repo: web::Data<RepositoryInjector>) -> HttpResponse {
    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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

    //BUILDING SHARED SERVER
    let repo = RepositoryInjector::new(MemoryRepository::default());
    let repo = web::Data::new(repo);

    //BUILDING SHARED SERVER
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("Starting thread {}", thread_index);

        App::new()
            .app_data(web::Data::new(thread_index))
            .app_data(web::Data::new(repo.clone()))
            .route("/", web::get().to(index))
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route("/health", web::get().to(health_check))
            .service(hello)
    })
    .bind(&address)
    .unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server in port {}: {:?}", port, err))
    .run()
    .await
}
