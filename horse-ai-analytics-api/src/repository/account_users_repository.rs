use mongodb::{bson::Document, Database};

use crate::struct_def::db_model;

pub async fn find_one_user_by_filter(
    db: Database,
    filter: Document,
) -> Option<db_model::AccountUsers> {
    let collection = db.collection::<db_model::AccountUsers>(db_model::ACCOUNT_USERS_COLLECTION);
    let result = collection.find_one(filter, None).await;
    return result.unwrap();
}
