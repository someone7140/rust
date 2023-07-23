use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct ResultResponse {
    pub success_flag: bool,
    pub text: String,
}
