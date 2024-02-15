use actix_web::{
    guard,
    web::{self, Data, ServiceConfig},
    HttpResponse, Result,
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

use graphql_object::horse_query;

mod graphql_object {
    pub mod horse_query;
}

type ApiSchema = Schema<horse_query::Query, EmptyMutation, EmptySubscription>;

async fn index(schema: Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn sdl_export(schema: Data<ApiSchema>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(schema.sdl()))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let schema = Schema::build(horse_query::Query, EmptyMutation, EmptySubscription).finish();
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(sdl_export));
    };

    Ok(config.into())
}
