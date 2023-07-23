mod controller {
    pub mod execute_controller;
}

mod model {
    pub mod result_response;
}

mod service {
    pub mod instagram_service;
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    println!(
        "{:?}",
        controller::execute_controller::execute_check_instagram(
            "id".to_string(),
            "password".to_string()
        )
    )
}
