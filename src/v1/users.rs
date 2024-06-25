use std::sync::{Arc, Mutex};

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use uuid::Uuid;

use crate::repository::{MemoryRepository, Repository};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").route("/{user_id}", web::get().to(get_user)));
}

async fn get_user(
    user_id: web::Path<Uuid>,
    repo: web::Data<Arc<Mutex<MemoryRepository>>>,
) -> HttpResponse {
    let repo = repo.lock().unwrap();
    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
