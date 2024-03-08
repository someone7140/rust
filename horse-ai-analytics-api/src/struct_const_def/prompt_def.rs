pub const HYPHEN: &str = "-";

#[derive(Clone)]
pub struct PromptHorseInfo {
    pub name: String,
    pub umanity_code: String,
    pub waku_num: Option<i32>,
    pub uma_num: Option<i32>,
    pub gender_and_age: String,
    pub weight: f32,
    pub belonging: String,
    pub trainer: String,
}

impl PromptHorseInfo {
    // 初期値を返す
    pub fn new() -> PromptHorseInfo {
        PromptHorseInfo {
            name: HYPHEN.to_string(),
            umanity_code: HYPHEN.to_string(),
            waku_num: None,
            uma_num: None,
            gender_and_age: HYPHEN.to_string(),
            weight: 0.0,
            belonging: HYPHEN.to_string(),
            trainer: HYPHEN.to_string(),
        }
    }
}
