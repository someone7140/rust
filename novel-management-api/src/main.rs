use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, http, middleware, routing::post, Extension, Router};
use reqwest::Method;
use sea_orm::{Database, DatabaseConnection};
use shuttle_runtime::{Error, SecretStore};
use tower_http::cors::CorsLayer;

use crate::{
    controller::{graphql_mutation::MutationRoot, graphql_query::QueryRoot},
    custom_middleware::auth_middleware::jwt_auth_middleware,
    model::common::context_info::{AuthContext, CommonContext},
};

mod controller {
    pub mod graphql_mutation;
    pub mod graphql_query;
}

mod custom_middleware {
    pub mod auth_middleware;
}

mod model {
    pub mod common {
        pub mod context_info;
    }
    pub mod graphql {
        pub mod graphql_error;
        pub mod graphql_guard;
        pub mod graphql_novel;
        pub mod graphql_user_account;
    }
}

mod repository {
    pub mod novel_repository;
    pub mod user_account_repository;
}

mod service {
    pub mod auth {
        pub mod google_auth_service;
        pub mod jwt_service;
        pub mod user_account_service;
    }
    pub mod common {
        pub mod data_time_service;
    }
    pub mod novel {
        pub mod novel_service;
    }
}

// DB接続のプール取得
async fn create_database_connection(
    secret: SecretStore,
    database_url: &str,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(database_url.to_owned());
    opt.max_connections(5)
        .min_connections(3)
        .connect_timeout(std::time::Duration::from_secs(20))
        .acquire_timeout(std::time::Duration::from_secs(20))
        .idle_timeout(std::time::Duration::from_secs(20))
        .max_lifetime(std::time::Duration::from_secs(20))
        .sqlx_logging(secret.get("SQL_DEBUG") == Some("true".to_string()));

    Database::connect(opt).await
}

// graphqlの設定
async fn graphql_handler(
    Extension(auth_context_opt): Extension<Option<AuthContext>>,
    State(context_state): State<CommonContext>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).data(context_state);
    if let Some(auth_context) = auth_context_opt {
        schema = schema.data(auth_context);
    }

    schema.finish().execute(req.into_inner()).await.into()
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // DB接続の取得
    let db_connect = if let Some(db_connect_str) = secret_store.get("DB_CONNECT") {
        create_database_connection(secret_store.clone(), &db_connect_str)
            .await
            .map_err(|error| Error::Database(error.to_string()))?
    } else {
        return Err(Error::Database("Can not get db config".to_string()));
    };

    // secret情報とDBをstateに格納
    let context_state: CommonContext = CommonContext {
        secrets: secret_store.clone(),
        db_connect: db_connect,
    };

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin([secret_store
            .get("FRONT_DOMAIN")
            .unwrap()
            .parse::<http::HeaderValue>()
            .unwrap()])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
            http::header::ACCEPT,
            http::HeaderName::from_static("apollo-require-preflight"),
            http::HeaderName::from_static("x-apollo-operation-name"),
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(middleware::from_fn_with_state(
            context_state.clone(),
            jwt_auth_middleware,
        ))
        .layer(cors)
        .with_state(context_state);

    Ok(app.into())
}
