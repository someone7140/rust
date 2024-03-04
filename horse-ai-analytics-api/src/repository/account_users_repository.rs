use mongodb::{bson::Document, error::Error, results::InsertOneResult, Database};

use crate::struct_def::db_model;

pub async fn find_one_user_by_filter(
    db: Database,
    filter: Document,
) -> Option<db_model::AccountUsers> {
    let collection = db.collection::<db_model::AccountUsers>(db_model::ACCOUNT_USERS_COLLECTION);
    let result = collection.find_one(filter, None).await;
    return result.unwrap();
}

pub async fn add_user(
    db: Database,
    add_user: db_model::AccountUsers,
) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<db_model::AccountUsers>(db_model::ACCOUNT_USERS_COLLECTION);
    let insert_one_result = collection.insert_one(add_user, None).await;
    return insert_one_result;
}
