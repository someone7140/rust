use crate::model::result_response::ResultResponse;
use crate::service::instagram_service;

pub fn execute_check_instagram(user_id: String, password: String) -> ResultResponse {
    instagram_service::instagram_follow_check();
    return ResultResponse {
        success_flag: true,
        text: user_id,
    };
}
