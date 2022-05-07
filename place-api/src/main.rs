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
    HttpServer::new(|| App::new().service(controller::search_controller::get_search_condition))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
