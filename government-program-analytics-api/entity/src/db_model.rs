// 府省庁
#[derive(Debug, toasty::Model)]
pub struct Ministry {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 局・庁
#[derive(Debug, toasty::Model)]
pub struct Agency {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 部
#[derive(Debug, toasty::Model)]
pub struct Department {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 課
#[derive(Debug, toasty::Model)]
pub struct Section {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 室
#[derive(Debug, toasty::Model)]
pub struct Office {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 担当者
#[derive(Debug, toasty::Model)]
pub struct PersonInCharge {
    #[key]
    pub id: i64,
    #[unique]
    pub name: String,
}

// 法令
#[derive(Debug, toasty::Model)]
pub struct Law {
    #[key]
    pub id: String,
    pub name: String,
    pub law_number: String,
}

// 基本情報
#[derive(Debug, toasty::Model)]
pub struct Project {
    #[key]
    pub project_id: i64,
    pub project_name: String,
    pub responsible_ministry_id: i64,
    pub ministry_id: i64,
    pub agency_id: Option<i64>,
    pub department_id: Option<i64>,
    pub section_name: Option<i64>,
    pub office_name: Option<i64>,
    pub person_in_charge_id: Option<i64>,
    pub project_purpose: String,
    pub project_situation: String,
    pub project_overview: String,
    pub overview_url: Option<String>,
    pub start_year: Option<i64>,
    pub end_year: Option<i64>,
    pub policy: Option<String>,
    pub policy_measure: Option<String>,
    pub policy_url: Option<String>,
    pub law_id: Option<String>,
    pub law_article: Option<String>,
    pub law_paragraph: Option<String>,
    pub law_item: Option<String>,
    pub plan_notification_name: Option<String>,
    pub plan_notification_url: Option<String>,
    pub subsidy_target: Option<String>,
    pub subsidy_rate: Option<String>,
    pub subsidy_limit: Option<String>,
    pub subsidy_url: Option<String>,
}

// 関連事業
#[derive(Debug, toasty::Model)]
pub struct RelationProject {
    #[key]
    pub project_id: i64,
    #[key]
    pub relation_project_id: i64,
    pub relationship: String,
}
