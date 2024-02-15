use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn test_query(
        &self,
        #[graphql(validator(list, max_length = 10))] names: Vec<String>,
    ) -> String {
        let a = 11111;
        let b = 3333;
        let sum = a + b;
        return (sum).to_string();
    }
}
