use actix_web::{
    web::{self, Data, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mongodb::{options::ClientOptions, Client};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::Error;
use shuttle_runtime::SecretStore;

use graphql_object::{horse_enum, horse_mutation, horse_query};
use service::{auth::account_user_service, common_service};
use struct_const_def::common_struct;

mod graphql_object {
    pub mod horse_enum;
    pub mod horse_model;
    pub mod horse_mutation;
    pub mod horse_query;
    pub mod horse_role;
}

mod repository {
    pub mod account_users_repository;
    pub mod race_info_repository;
    pub mod race_memo_category_repository;
    pub mod vote_result_repository;
}

mod service {
    pub mod auth {
        pub mod account_user_service;
        pub mod google_auth_service;
    }
    pub mod common_service;
    pub mod external_info {
        pub mod external_info_common_service;
        pub mod external_info_main_service;
        pub mod netkeiba_service;
        pub mod tospo_keiba_service;
        pub mod umanity_service;
    }
    pub mod jwt_service;
    pub mod race_info {
        pub mod race_info_service;
        pub mod race_memo_category_service;
    }
    pub mod vote_result {
        pub mod vote_result_service;
    }
}

mod struct_const_def {
    pub mod common_struct;
    pub mod db_model;
    pub mod prompt_def;
}

type ApiSchema = Schema<horse_query::Query, horse_mutation::Mutation, EmptySubscription>;

async fn index(
    schema: Data<ApiSchema>,
    secrets_data: Data<SecretStore>,
    req: HttpRequest,
    graphql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = graphql_req.into_inner();
    // secretからjwtのシークレットキーを取得
    if let Some(jwt_secret) = secrets_data.clone().get("JWT_SECRET") {
        // ヘッダーから認証情報を取得
        if let Some(auth_context) =
            account_user_service::get_token_from_authorization_header(req.headers(), jwt_secret)
        {
            // 認証情報の取得に成功したらrequestのcontextにセット
            request = request.data(auth_context);
        }
    }
    schema.execute(request).await.into()
}

async fn sdl_export(schema: Data<ApiSchema>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(schema.sdl()))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
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
    .data(common_struct::CommonContext {
        secrets: secrets.clone(),
        mongo_db,
    })
    .finish();
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(schema.clone()))
            .app_data(Data::new(secrets.clone()))
            .service(
                web::resource("/graphql")
                    .wrap(common_service::get_cors_setting(
                        &(secrets.clone().get("FRONT_DOMAIN").unwrap()),
                    ))
                    .to(index),
            )
            .service(
                web::resource("/sdl")
                    .wrap(common_service::get_cors_setting(
                        &(secrets.clone().get("FRONT_DOMAIN").unwrap()),
                    ))
                    .to(sdl_export),
            );
    };
    Ok(config.into())
}
