use crate::{
    model::csv_struct::{
        BasicInfo11Organization, BasicInfo12Overview, BasicInfo13Policy, BasicInfo14Subsidy,
        BasicInfo15RelationProject,
    },
    util::{csv_util, db_util::batch_insert},
};
use entity::db_model::{
    Agency, Department, Law, Ministry, Office, PersonInCharge, Project, RelationProject, Section,
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
use toasty::{
    Db,
    stmt::{List, Query},
};

// 基本情報のインポート
pub async fn import_basic_info(db: &mut Db) -> Result<(), Box<dyn Error>> {
    // 各csvから取り込み
    let mut basic_info_map = import_basicinfo11_organization(db).await?;
    import_basicinfo12_overview(&mut basic_info_map).await?;
    import_basicinfo13_policy(db, &mut basic_info_map).await?;
    import_basicinfo14_subsidy(&mut basic_info_map).await?;
    import_basicinfo15_relation_project(db).await?;

    // projectの登録
    Query::<List<Project>>::all().delete().exec(db).await?;
    let projects: Vec<&Project> = basic_info_map.values().collect();
    batch_insert::<Project, _, _>(db, &projects, Project::create_many, |project, item| {
        item.project_id(project.project_id)
            .project_name(project.project_name.clone())
            .responsible_ministry_id(project.responsible_ministry_id)
            .ministry_id(project.ministry_id)
            .agency_id(project.agency_id)
            .department_id(project.department_id)
            .section_name(project.section_name)
            .office_name(project.office_name)
            .person_in_charge_id(project.person_in_charge_id)
            .project_purpose(project.project_purpose.clone())
            .project_situation(project.project_situation.clone())
            .project_overview(project.project_overview.clone())
            .overview_url(project.overview_url.clone())
            .start_year(project.start_year)
            .end_year(project.end_year)
            .policy(project.policy.clone())
            .policy_measure(project.policy_measure.clone())
            .policy_url(project.policy_url.clone())
            .law_id(project.law_id.clone())
            .law_article(project.law_article.clone())
            .law_paragraph(project.law_paragraph.clone())
            .law_item(project.law_item.clone())
            .plan_notification_name(project.plan_notification_name.clone())
            .plan_notification_url(project.plan_notification_url.clone())
            .subsidy_target(project.subsidy_target.clone())
            .subsidy_rate(project.subsidy_rate.clone())
            .subsidy_limit(project.subsidy_limit.clone())
            .subsidy_url(project.subsidy_url.clone())
    })
    .await?;

    Ok(())
}

// 1-1_基本情報_組織情報のcsv
pub async fn import_basicinfo11_organization(
    db: &mut Db,
) -> Result<HashMap<i64, Project>, Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;

    // 1-1_基本情報_組織情報のcsv取り込み
    let basicinfo11_organization_itr =
        csv_util::read_csv::<BasicInfo11Organization>("1-1", &folder_path)?;
    let organization_rows = basicinfo11_organization_itr.collect::<Result<Vec<_>, _>>()?;

    // 府省庁の変数
    let mut ministry_id_counter: i64 = 1; // 府省庁IDの割り振りカウンター
    let mut ministry_name_set = HashSet::<String>::new(); // 登録済みの府省庁名を保持
    let mut ministry_name_id_map = HashMap::<String, i64>::new(); // 名前からidを取得する用途
    let mut ministries: Vec<(i64, String)> = Vec::new();

    // 局・庁
    let mut agency_id_counter: i64 = 1; // 局・庁IDの割り振りカウンター
    let mut agency_name_set = HashSet::<String>::new(); // 登録済みの局・庁名を保持
    let mut agency_name_id_map = HashMap::<String, i64>::new(); // 局・庁名からIDを取得する用途
    let mut agencies: Vec<(i64, String)> = Vec::new();

    // 部
    let mut department_id_counter: i64 = 1; // 部IDの割り振りカウンター
    let mut department_name_set = HashSet::<String>::new(); // 登録済みの部名を保持
    let mut department_name_id_map = HashMap::<String, i64>::new(); // 部名からIDを取得する用途
    let mut departments: Vec<(i64, String)> = Vec::new();

    // 課
    let mut section_id_counter: i64 = 1; // 課IDの割り振りカウンター
    let mut section_name_set = HashSet::<String>::new(); // 登録済みの課名を保持
    let mut section_name_id_map = HashMap::<String, i64>::new(); // 課名からIDを取得する用途
    let mut sections: Vec<(i64, String)> = Vec::new();

    // 室
    let mut office_id_counter: i64 = 1; // 室IDの割り振りカウンター
    let mut office_name_set = HashSet::<String>::new(); // 登録済みの室名を保持
    let mut office_name_id_map = HashMap::<String, i64>::new(); // 室名からIDを取得する用途
    let mut offices: Vec<(i64, String)> = Vec::new();

    // 担当者用の変数
    let mut person_in_charge_id_counter: i64 = 1; // 担当者IDの割り振りカウンター
    let mut person_in_charge_name_set = HashSet::<String>::new(); // 登録済みの担当者名を保持
    let mut person_in_charge_name_id_map = HashMap::<String, i64>::new(); // 担当者名からIDを取得する用途
    let mut people_in_charge: Vec<(i64, String)> = Vec::new();

    let mut intermediate_data = HashMap::with_capacity(organization_rows.len());

    for org_val in &organization_rows {
        // 府省庁が新しく登場したら登録対象とする
        if !org_val.ministry_name.clone().is_empty()
            && ministry_name_set.insert(org_val.ministry_name.clone())
        {
            ministries.push((ministry_id_counter, org_val.ministry_name.clone()));
            ministry_name_id_map.insert(org_val.ministry_name.clone(), ministry_id_counter);
            ministry_id_counter += 1;
        }

        // 局・庁が新しく登場したら登録対象とする
        if let Some(name) = &org_val.agency_name {
            if !name.is_empty() && agency_name_set.insert(name.clone()) {
                agencies.push((agency_id_counter, name.clone()));
                agency_name_id_map.insert(name.clone(), agency_id_counter);
                agency_id_counter += 1;
            }
        }

        // 部が新しく登場したら登録対象とする
        if let Some(name) = &org_val.department_name {
            if !name.is_empty() && department_name_set.insert(name.clone()) {
                departments.push((department_id_counter, name.clone()));
                department_name_id_map.insert(name.clone(), department_id_counter);
                department_id_counter += 1;
            }
        }

        // 課が新しく登場したら登録対象とする
        if let Some(name) = &org_val.section_name {
            if !name.is_empty() && section_name_set.insert(name.clone()) {
                sections.push((section_id_counter, name.clone()));
                section_name_id_map.insert(name.clone(), section_id_counter);
                section_id_counter += 1;
            }
        }

        // 室が新しく登場したら登録対象とする
        if let Some(name) = &org_val.office_name {
            if !name.is_empty() && office_name_set.insert(name.clone()) {
                offices.push((office_id_counter, name.clone()));
                office_name_id_map.insert(name.clone(), office_id_counter);
                office_id_counter += 1;
            }
        }

        // 担当者が新しく登場したら登録対象とする
        if let Some(name) = &org_val.person_in_charge_name {
            if !name.is_empty() && person_in_charge_name_set.insert(name.clone()) {
                people_in_charge.push((person_in_charge_id_counter, name.clone()));
                person_in_charge_name_id_map.insert(name.clone(), person_in_charge_id_counter);
                person_in_charge_id_counter += 1;
            }
        }

        intermediate_data.insert(
            org_val.project_id,
            Project {
                project_id: org_val.project_id,
                project_name: org_val.project_name.clone(),
                responsible_ministry_id: *ministry_name_id_map
                    .get(&org_val.responsible_ministry_name)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!(
                                "所管府省庁 '{}' が府省庁名として登録されていません",
                                org_val.responsible_ministry_name
                            ),
                        )
                    })?,
                ministry_id: *ministry_name_id_map
                    .get(&org_val.ministry_name)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("府省庁 '{}' が登録されていません", org_val.ministry_name),
                        )
                    })?,
                agency_id: org_val
                    .agency_name
                    .as_ref()
                    .and_then(|name| agency_name_id_map.get(name))
                    .copied(),
                department_id: org_val
                    .department_name
                    .as_ref()
                    .and_then(|name| department_name_id_map.get(name))
                    .copied(),
                section_name: org_val
                    .section_name
                    .as_ref()
                    .and_then(|name| section_name_id_map.get(name))
                    .copied(),
                office_name: org_val
                    .office_name
                    .as_ref()
                    .and_then(|name| office_name_id_map.get(name))
                    .copied(),
                person_in_charge_id: org_val
                    .person_in_charge_name
                    .as_ref()
                    .and_then(|name| person_in_charge_name_id_map.get(name))
                    .copied(),
                project_purpose: "".to_string(),
                project_situation: "".to_string(),
                project_overview: "".to_string(),
                overview_url: None,
                start_year: None,
                end_year: None,
                policy: None,
                policy_measure: None,
                policy_url: None,
                law_id: None,
                law_article: None,
                law_paragraph: None,
                law_item: None,
                plan_notification_name: None,
                plan_notification_url: None,
                subsidy_target: None,
                subsidy_rate: None,
                subsidy_limit: None,
                subsidy_url: None,
            },
        );
    }

    // 全件クリア
    Query::<List<Ministry>>::all().delete().exec(db).await?;
    Query::<List<Agency>>::all().delete().exec(db).await?;
    Query::<List<Department>>::all().delete().exec(db).await?;
    Query::<List<Section>>::all().delete().exec(db).await?;
    Query::<List<Office>>::all().delete().exec(db).await?;
    Query::<List<PersonInCharge>>::all()
        .delete()
        .exec(db)
        .await?;

    // 登録
    batch_insert::<Ministry, _, _>(db, &ministries, Ministry::create_many, |(id, name), m| {
        m.id(*id).name(name.clone())
    })
    .await?;
    batch_insert::<Agency, _, _>(db, &agencies, Agency::create_many, |(id, name), a| {
        a.id(*id).name(name.clone())
    })
    .await?;
    batch_insert::<Department, _, _>(
        db,
        &departments,
        Department::create_many,
        |(id, name), d| d.id(*id).name(name.clone()),
    )
    .await?;
    batch_insert::<Section, _, _>(db, &sections, Section::create_many, |(id, name), s| {
        s.id(*id).name(name.clone())
    })
    .await?;
    batch_insert::<Office, _, _>(db, &offices, Office::create_many, |(id, name), o| {
        o.id(*id).name(name.clone())
    })
    .await?;
    batch_insert::<PersonInCharge, _, _>(
        db,
        &people_in_charge,
        PersonInCharge::create_many,
        |(id, name), p| p.id(*id).name(name.clone()),
    )
    .await?;

    Ok(intermediate_data)
}

// 1-2_基本情報_事業概要等のcsvを取り込み、中間データを更新
pub async fn import_basicinfo12_overview(
    basic_info_map: &mut HashMap<i64, Project>,
) -> Result<(), Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;
    let basicinfo12_overview_itr = csv_util::read_csv::<BasicInfo12Overview>("1-2", &folder_path)?;
    let overview_rows = basicinfo12_overview_itr.collect::<Result<Vec<_>, _>>()?;

    for overview in overview_rows {
        if let Some(intermediate_data) = basic_info_map.get_mut(&overview.project_id) {
            intermediate_data.project_purpose = overview.project_purpose;
            intermediate_data.project_situation = overview.project_situation;
            intermediate_data.project_overview = overview.project_overview;
            intermediate_data.overview_url = overview.overview_url;
            intermediate_data.start_year = overview.start_year;
            intermediate_data.end_year = overview.end_year;
        }
    }

    Ok(())
}

// 1-3_基本情報_政策・施策、法令等のcsvを取り込み、中間データと法令テーブルを更新
pub async fn import_basicinfo13_policy(
    db: &mut Db,
    basic_info_map: &mut HashMap<i64, Project>,
) -> Result<(), Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;
    let policy_itr = csv_util::read_csv::<BasicInfo13Policy>("1-3", &folder_path)?;
    let policy_rows = policy_itr.collect::<Result<Vec<_>, _>>()?;

    let mut law_id_set = HashSet::<String>::new();
    let mut laws: Vec<(String, String, String)> = Vec::new();
    for row in policy_rows {
        if let Some(intermediate_data) = basic_info_map.get_mut(&row.project_id) {
            intermediate_data.policy = row.policy;
            intermediate_data.policy_measure = row.policy_measure;
            intermediate_data.policy_url = row.policy_url;
            intermediate_data.law_id = row.law_id.clone();
            intermediate_data.law_article = row.law_article;
            intermediate_data.law_paragraph = row.law_paragraph;
            intermediate_data.law_item = row.law_item;
            intermediate_data.plan_notification_name = row.plan_notification_name;
            intermediate_data.plan_notification_url = row.plan_notification_url;
        }

        if let (Some(id), Some(name), Some(number)) = (row.law_id, row.law_name, row.law_number) {
            if !id.is_empty()
                && !name.is_empty()
                && !number.is_empty()
                && law_id_set.insert(id.clone())
            {
                laws.push((id, name, number));
            }
        }
    }

    Query::<List<Law>>::all().delete().exec(db).await?;
    batch_insert::<Law, _, _>(db, &laws, Law::create_many, |(id, name, number), law| {
        law.id(id.clone())
            .name(name.clone())
            .law_number(number.clone())
    })
    .await?;

    Ok(())
}

// 1-4_基本情報_補助率等のcsvを取り込み、中間データを更新
pub async fn import_basicinfo14_subsidy(
    basic_info_map: &mut HashMap<i64, Project>,
) -> Result<(), Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;
    let subsidy_itr = csv_util::read_csv::<BasicInfo14Subsidy>("1-4", &folder_path)?;
    let subsidy_rows = subsidy_itr.collect::<Result<Vec<_>, _>>()?;

    for row in subsidy_rows {
        if let Some(intermediate_data) = basic_info_map.get_mut(&row.project_id) {
            intermediate_data.subsidy_target = row.subsidy_target;
            intermediate_data.subsidy_rate = row.subsidy_rate;
            intermediate_data.subsidy_limit = row.subsidy_limit;
            intermediate_data.subsidy_url = row.subsidy_url;
        }
    }

    Ok(())
}

// 1-5_基本情報_関連事業のcsvを取り込み、関連事業テーブルに登録
pub async fn import_basicinfo15_relation_project(db: &mut Db) -> Result<(), Box<dyn Error>> {
    let folder_path = std::env::var("IMPORT_DATA_FOLDER")?;
    let relation_project_itr =
        csv_util::read_csv::<BasicInfo15RelationProject>("1-5", &folder_path)?;
    let relation_project_rows = relation_project_itr.collect::<Result<Vec<_>, _>>()?;

    let mut relation_project_set = HashSet::<(i64, i64)>::new();
    let mut relation_projects: Vec<(i64, i64, String)> = Vec::new();
    for row in relation_project_rows {
        // 関連プロジェクトが登録されている行のみが登録対象
        if let (Some(relation_project_id), Some(relationship)) =
            (row.relation_project_id, row.relationship)
        {
            if relation_project_set.insert((row.project_id, relation_project_id)) {
                relation_projects.push((row.project_id, relation_project_id, relationship));
            }
        }
    }

    Query::<List<RelationProject>>::all()
        .delete()
        .exec(db)
        .await?;
    batch_insert::<RelationProject, _, _>(
        db,
        &relation_projects,
        RelationProject::create_many,
        |(project_id, relation_project_id, relationship), item| {
            item.project_id(*project_id)
                .relation_project_id(*relation_project_id)
                .relationship(relationship.clone())
        },
    )
    .await?;

    Ok(())
}
