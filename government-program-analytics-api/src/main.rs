use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Schema,
    http::{GraphQLPlaygroundConfig, playground_source},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::Html,
    routing::{get, post},
};
use tokio::net::TcpListener;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, world!"
    }
}

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let listener = TcpListener::bind("0.0.0.0:5030")
        .await
        .expect("failed to bind");

    println!("Server running on http://localhost:5030/graphql");

    axum::serve(listener, app).await.expect("server error");
}
