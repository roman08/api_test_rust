use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

// Handler para la ruta de salud
async fn health_check(thread_index: web::Data<u16>) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("thread-id", thread_index.to_string()))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::StatusCode,
        test,
        web::{self, service},
        App,
    };

    use super::health_check;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health_check(web::Data::new(5)).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res
            .headers()
            .get("thread-id")
            .map(|h| h.to_str().ok())
            .flatten();

        assert_eq!(data, Some("5"));
    }

    //     #[actix_rt::test]
    //     async fn health_check_integration_works() {
    //         let app = App::new().app_data(web::Data::new(5u16)).configure(service);
    //     }
}
