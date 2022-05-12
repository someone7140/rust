use actix_cors::Cors;
use actix_web::http;
use actix_web::App;
use actix_web::HttpServer;

mod controller {
    pub mod search_controller;
}

mod service {
    pub mod search_service;
}

mod model {
    pub mod api {
        pub mod search_condition_response;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:19006")
            .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(controller::search_controller::get_search_condition)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
