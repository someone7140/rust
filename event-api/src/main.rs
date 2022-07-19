use actix_cors::Cors;
use actix_files as fs;
use actix_web::http;
use actix_web::App;
use actix_web::HttpServer;
use dotenv;
use std::env;

mod controller {
    pub mod get_event_info_controller;
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
    pub mod api {
        pub mod event_info_master_response;
    }
}

mod repository {
    pub mod event_repository;
    pub mod event_search_info_repository;
    pub mod mongodb_client;
}

mod service {
    pub mod get_event_service;
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
    // ポートの取得
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&env::var("FRONT_DOMAIN").unwrap())
            .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(fs::Files::new("/contents", "asset/").show_files_listing())
            .service(controller::update_event_info_controller::update_event_info)
            .service(controller::get_event_info_controller::get_event_master)
            .service(controller::get_event_info_controller::get_event_list)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
