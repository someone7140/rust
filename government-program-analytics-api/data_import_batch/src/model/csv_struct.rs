use serde::Deserialize;

// 1-1_基本情報_組織情報
#[derive(Debug, Deserialize)]
pub struct BasicInfo11Organization {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "事業名")]
    pub project_name: String,

    #[serde(rename = "所管府省庁")]
    pub responsible_ministry_name: String,

    #[serde(rename = "府省庁")]
    pub ministry_name: String,

    #[serde(rename = "局・庁")]
    pub agency_name: Option<String>,

    #[serde(rename = "部")]
    pub department_name: Option<String>,

    #[serde(rename = "課")]
    pub section_name: Option<String>,

    #[serde(rename = "室")]
    pub office_name: Option<String>,

    #[serde(rename = "作成責任者")]
    pub person_in_charge_name: Option<String>,
}

// 1-2_基本情報_事業概要等
#[derive(Debug, Deserialize)]
pub struct BasicInfo12Overview {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "事業の目的")]
    pub project_purpose: String,

    #[serde(rename = "現状・課題")]
    pub project_situation: String,

    #[serde(rename = "事業の概要")]
    pub project_overview: String,

    #[serde(rename = "事業概要URL")]
    pub overview_url: Option<String>,

    #[serde(rename = "事業開始年度")]
    pub start_year: Option<i64>,

    #[serde(rename = "事業終了（予定）年度")]
    pub end_year: Option<i64>,
}

// 1-3_基本情報_政策・施策、法令等
#[derive(Debug, Deserialize)]
pub struct BasicInfo13Policy {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "政策")]
    pub policy: Option<String>,

    #[serde(rename = "施策")]
    pub policy_measure: Option<String>,

    #[serde(rename = "政策・施策URL")]
    pub policy_url: Option<String>,

    #[serde(rename = "法令名")]
    pub law_name: Option<String>,

    #[serde(rename = "法令番号")]
    pub law_number: Option<String>,

    #[serde(rename = "法令ID")]
    pub law_id: Option<String>,

    #[serde(rename = "条")]
    pub law_article: Option<String>,

    #[serde(rename = "項")]
    pub law_paragraph: Option<String>,

    #[serde(rename = "号・号の細分")]
    pub law_item: Option<String>,

    #[serde(rename = "計画通知名")]
    pub plan_notification_name: Option<String>,

    #[serde(rename = "計画通知等URL")]
    pub plan_notification_url: Option<String>,
}

// 1-4_基本情報_補助率等
#[derive(Debug, Deserialize)]
pub struct BasicInfo14Subsidy {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "補助対象")]
    pub subsidy_target: Option<String>,

    #[serde(rename = "補助率")]
    pub subsidy_rate: Option<String>,

    #[serde(rename = "補助上限等")]
    pub subsidy_limit: Option<String>,

    #[serde(rename = "補助率URL")]
    pub subsidy_url: Option<String>,
}

// 1-5_基本情報_関連事業
#[derive(Debug, Deserialize)]
pub struct BasicInfo15RelationProject {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "関連事業の事業ID")]
    pub relation_project_id: Option<i64>,

    #[serde(rename = "関連性")]
    pub relationship: Option<String>,
}

// 2-1_予算・執行_サマリ
#[derive(Debug, Deserialize)]
pub struct Budget21Summary {
    #[serde(rename = "予算事業ID")]
    pub project_id: i64,

    #[serde(rename = "予算年度")]
    pub budget_year: i32,

    #[serde(rename = "当初予算（合計）")]
    pub initial_budget: Option<f64>,

    #[serde(rename = "補正予算（合計）")]
    pub supplementary_budget: Option<f64>,

    #[serde(rename = "前年度からの繰越し（合計）")]
    pub carried_over_budget: Option<f64>,

    #[serde(rename = "予備費等（合計）")]
    pub contingency_budget: Option<f64>,

    #[serde(rename = "計（歳出予算現額合計）")]
    pub summary_budget: Option<f64>,

    #[serde(rename = "執行額（合計）")]
    pub executuon_budget: Option<f64>,

    #[serde(rename = "執行率")]
    pub executuon_rate: Option<f64>,

    #[serde(rename = "翌年度への繰越し(合計）")]
    pub carried_forward_budget: Option<f64>,

    #[serde(rename = "翌年度要求額（合計）")]
    pub request_budget: Option<f64>,

    #[serde(rename = "主な増減理由")]
    pub reason_for_change: Option<String>,
}
