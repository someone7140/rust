mod controller {
    pub mod execute_controller;
}

mod model {
    pub mod result_response;
}

mod service {
    pub mod instagram_service;
}

use controller::execute_controller;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check_follow(user_id: String, password: String) -> model::result_response::ResultResponse {
    return execute_controller::execute_check_instagram(user_id, password);
}
