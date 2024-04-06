use mongodb::{
    bson::{doc, Document},
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Database,
};

use crate::struct_const_def::db_model;

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

pub async fn edit_user(
    db: Database,
    edit_user: db_model::AccountUsers,
) -> Result<UpdateResult, Error> {
    let collection = db.collection::<db_model::AccountUsers>(db_model::ACCOUNT_USERS_COLLECTION);
    let filter = doc! { "_id": edit_user.clone().id };
    let update_set = doc! {"$set": doc! { "name": edit_user.name, "user_setting_id": edit_user.user_setting_id }};
    let update_one_result = collection.update_one(filter, update_set, None).await;
    return update_one_result;
}
