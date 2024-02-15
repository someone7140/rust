use async_graphql::*;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn test_query(
        &self,
        #[graphql(validator(list, max_length = 10))] names: Vec<String>,
    ) -> String {
        return "aaaaa".to_string();
    }
}
