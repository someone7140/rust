use actix_web::{
    guard,
    web::{self, Data, ServiceConfig},
    HttpResponse, Result,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

use graphql_object::{horse_enum, horse_mutation, horse_query};
use struct_def::common_struct;

mod graphql_object {
    pub mod horse_enum;
    pub mod horse_mutation;
    pub mod horse_query;
}

mod service {
    pub mod auth {
        pub mod google_auth_service;
    }
}

mod struct_def {
    pub mod common_struct;
}

type ApiSchema = Schema<horse_query::Query, horse_mutation::Mutation, EmptySubscription>;

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
    let schema = Schema::build(
        horse_query::Query,
        horse_mutation::Mutation,
        EmptySubscription,
    )
    .register_output_type::<horse_enum::ErrorType>()
    .data(common_struct::CommonContext { secrets })
    .finish();
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(sdl_export));
    };

    Ok(config.into())
}
