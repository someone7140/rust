use crate::struct_const_def::db_model;
use mongodb::{error::Error, results::InsertOneResult, Database};

pub async fn add_vote_result(
    db: Database,
    vote_result: db_model::VoteResult,
) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<db_model::VoteResult>(db_model::VOTE_RESULTS_COLLECTION);
    let insert_one_result = collection.insert_one(vote_result, None).await;
    return insert_one_result;
}
