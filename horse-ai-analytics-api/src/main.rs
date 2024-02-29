use actix_web::{
    guard,
    web::{self, Data, ServiceConfig},
    HttpResponse, Result,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mongodb::{options::ClientOptions, Client};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::Error;
use shuttle_secrets::SecretStore;

use graphql_object::{horse_enum, horse_model, horse_mutation, horse_query};
use struct_def::common_struct;

mod graphql_object {
    pub mod horse_enum;
    pub mod horse_model;
    pub mod horse_mutation;
    pub mod horse_query;
}

mod repository {
    pub mod account_users_repository;
}

mod service {
    pub mod auth {
        pub mod google_auth_service;
    }
    pub mod jwt_service;
}

mod struct_def {
    pub mod common_struct;
    pub mod db_model;
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
    // MongoDBクライアントの取得
    let mongo_client_option_result = match secrets.get("DB_CONNECT") {
        Some(db_connect) => {
            let op = ClientOptions::parse(db_connect).await;
            op
        }
        None => return Err(Error::Database("Can not get db config".to_string())),
    };

    let mongo_client_result = match mongo_client_option_result {
        Ok(mongo_client_option) => {
            let client = Client::with_options(mongo_client_option);
            client
        }
        Err(error) => return Err(Error::Database(error.to_string())),
    };

    let mongo_db = match (secrets.get("DB_NAME"), mongo_client_result) {
        (Some(db_name), Ok(mongo_client)) => {
            let db = mongo_client.database(&db_name);
            db
        }
        (_, _) => return Err(Error::Database("Can not get db client".to_string())),
    };

    // graphqlスキーマ
    let schema = Schema::build(
        horse_query::Query,
        horse_mutation::Mutation,
        EmptySubscription,
    )
    .register_output_type::<horse_enum::ErrorType>()
    .data(common_struct::CommonContext { secrets, mongo_db })
    .finish();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(sdl_export));
    };
    Ok(config.into())
}
