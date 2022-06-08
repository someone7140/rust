use actix_cors::Cors;
use actix_web::http;
use actix_web::App;
use actix_web::HttpServer;
use dotenv;
use std::env;

mod controller {
    pub mod update_event_info_controller;
}

mod gather {
    pub mod gather_event_data;
}

mod model {
    pub mod db {
        pub mod event_collection;
        pub mod event_info_collection;
    }
}

mod repository {
    pub mod event_repository;
    pub mod event_search_info_repository;
    pub mod mongodb_client;
}

mod service {
    pub mod update_event_service;
}

mod util {
    pub mod date_util;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "prod".to_string(),
    };
    // 環境毎にファイルを配置して読み込み
    dotenv::from_filename(".env.".to_string() + &environment).ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(controller::update_event_info_controller::update_event_info)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}