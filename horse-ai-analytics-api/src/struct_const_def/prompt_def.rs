use phf::phf_ordered_map;

pub const HYPHEN: &str = "-";

#[derive(Clone)]
pub struct PromptHorseInfo {
    pub name: String,
    pub umanity_code: String,
    pub waku_num: Option<i32>,
    pub uma_num: Option<i32>,
    pub gender_and_age: String,
    pub charge_weight: f32,
    pub belonging: String,
    pub trainer: String,
    pub all_results: String,
    pub recent_results: String,
    pub career_prize_money: String,
    pub father: String,
    pub mother: String,
    pub mother_father: String,
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
            charge_weight: 0.0,
            belonging: HYPHEN.to_string(),
            trainer: HYPHEN.to_string(),
            all_results: HYPHEN.to_string(),
            recent_results: HYPHEN.to_string(),
            career_prize_money: HYPHEN.to_string(),
            father: HYPHEN.to_string(),
            mother: HYPHEN.to_string(),
            mother_father: HYPHEN.to_string(),
        }
    }
}

pub const CSV_COLUMN_MAP: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "waku_num" => "枠番",
    "uma_num" => "馬番",
    "name" => "馬名",
    "gender_and_age" => "性齢",
    "charge_weight" => "負担重量",
    "belonging" => "所属",
    "trainer" => "調教師",
    "all_results" => "全成績",
    "career_prize_money" => "合計獲得賞金",
    "father" => "父",
    "mother" => "母",
    "mother_father" => "母父",
};
