use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, routing::post, Router};
use sea_orm::{Database, DatabaseConnection};
use shuttle_runtime::{Error, SecretStore};

use crate::{
    controller::{graphql_mutation::MutationRoot, graphql_query::QueryRoot},
    model::common::context_info::CommonContext,
};

mod controller {
    pub mod graphql_mutation;
    pub mod graphql_query;
}

mod model {
    pub mod common {
        pub mod context_info;
    }
    pub mod graphql {
        pub mod graphql_error;
        pub mod graphql_user_account;
    }
}

mod repository {
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
}

// DB接続のプール取得
async fn create_database_connection(
    database_url: &str,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(database_url.to_owned());
    opt.max_connections(5)
        .min_connections(3)
        .connect_timeout(std::time::Duration::from_secs(20))
        .acquire_timeout(std::time::Duration::from_secs(20))
        .idle_timeout(std::time::Duration::from_secs(20))
        .max_lifetime(std::time::Duration::from_secs(20));

    Database::connect(opt).await
}

// graphqlの設定
async fn graphql_handler(
    State(context_state): State<CommonContext>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(context_state) // StateをGraphQLのcontextに追加
        .finish();

    schema.execute(req.into_inner()).await.into()
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // DB接続の取得
    let db_connect = if let Some(db_connect_str) = secret_store.get("DB_CONNECT") {
        create_database_connection(&db_connect_str)
            .await
            .map_err(|error| Error::Database(error.to_string()))?
    } else {
        return Err(Error::Database("Can not get db config".to_string()));
    };

    // secret情報とDBをstateに格納
    let context_state = CommonContext {
        secrets: secret_store.clone(),
        db_connect: db_connect,
    };

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .with_state(context_state);
    Ok(app.into())
}
