use actix_cors::Cors;
use actix_web::http;
use actix_web::App;
use actix_web::HttpServer;
use dotenv;
use std::env;

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
    pub mod db {
        pub mod search_condition_collection;
    }
}

mod repository {
    pub mod mongodb_client;
    pub mod search_condition_repository;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 「ENVIRONMENT」という環境変数に事前に環境をセット
    let environment = match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "prod".to_string(),
    };
    // 環境毎にファイルを配置して読み込み
    dotenv::from_filename(".env.".to_string() + &environment).ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&env::var("FRONT_DOMAIN").unwrap())
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
