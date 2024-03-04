use actix_cors::Cors;

pub fn get_cors_setting(origin: &String) -> Cors {
    Cors::default()
        .allowed_origin(origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .supports_credentials()
}
