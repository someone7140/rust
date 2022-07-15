use crate::gather::gather_event_data;
use crate::model::db::event_info_collection::EventUpdateHistoryCollection;
use crate::repository::event_repository;
use crate::repository::event_search_info_repository;
use crate::util::date_util;
use std::error::Error;

pub async fn update_event_execute() -> Result<(), Box<dyn Error>> {
    // DBから更新の管理情報を取得
    let event_search_master_col = event_search_info_repository::get_event_search_master()?;
    let event_update_history_col = event_search_info_repository::get_event_update_history()?;
    // 現在日付（0時0分0秒）
    let now_date = date_util::get_now_jst_date();
    let now_date_time = now_date.timestamp();

    for event_search_master in event_search_master_col.into_iter() {
        let event_search_master_ref = &event_search_master;
        let location_key = event_search_master_ref.clone()._id;
        // 該当の地域キーで更新履歴を絞り込み
        let event_updates = event_update_history_col
            .iter()
            .filter(|history| history.location_key == location_key);
        let event_updates_clone = event_updates.clone().map(|e| e.clone());
        let mut event_updates_vec = event_updates_clone.collect();
        // 更新履歴が登録済みで無い場合は初期値で上書き
        if event_updates.count() == 0 {
            // 翌日から7日分初期設定
            event_updates_vec = event_search_info_repository::set_init_event_update_history(
                location_key.clone(),
                now_date,
                7,
            )?;
        }
        let event_updates_vec_refer = &event_updates_vec;
        // 更新対象
        let update_targets = event_updates_vec_refer
            .into_iter()
            .filter(|history| history.is_update_target(now_date_time));
        // 2つの日付のデータをサイトから更新
        for (i, val) in update_targets.into_iter().enumerate() {
            // 3つ目以上はスキップ
            if i > 1 {
                break;
            } else {
                // サイトからデータ収集
                let gather_events = gather_event_data::get_event_data(
                    event_search_master_ref.clone(),
                    val.event_date.clone(),
                    now_date_time.clone(),
                )
                .await?;
                // event_update_historyに登録
                event_repository::add_events(gather_events)?;
                // event_update_historyのupdate_timeを現時刻で更新
                event_search_info_repository::update_time_event_update_history(
                    location_key.clone(),
                    val.event_date.clone(),
                    now_date_time.clone(),
                )?;
                // 前に追加したeventデータを削除
                event_repository::delete_events(
                    location_key.clone(),
                    val.event_date.clone(),
                    val.update_time.clone(),
                )?;
            }
        }
        // 削除対象
        let delete_targets = event_updates_vec_refer
            .into_iter()
            .filter(|history| history.is_delete_target(now_date_time));
        let delete_targets_clone = delete_targets.clone().map(|e| e.clone());
        let delete_targets_vec: Vec<EventUpdateHistoryCollection> = delete_targets_clone.collect();
        let delete_targets_vec_refer = &delete_targets_vec;
        // 削除処理
        let delete_count = delete_targets_vec_refer.len();
        if delete_count > 0 {
            for (_, delete_target_ref) in delete_targets_vec_refer.into_iter().enumerate() {
                event_repository::delete_events(
                    location_key.clone(),
                    delete_target_ref.event_date.clone(),
                    delete_target_ref.update_time.clone(),
                )?;
            }
            event_search_info_repository::delete_event_update_history(
                location_key.clone(),
                delete_targets_vec_refer
                    .iter()
                    .map(|d| d.event_date.clone())
                    .collect(),
            )?;
        }
        // 日付の追加（登録されているレコードが14日に満たない場合）
        let update_count = event_updates_vec_refer.len();
        if (update_count - delete_count) < 14 {
            // maxの日付
            let mut max_date_time = now_date;
            match event_updates_vec_refer
                .iter()
                .map(
                    |e| match date_util::parse_str_jst_date(e.event_date.clone()) {
                        Ok(d) => d,
                        Err(_e) => now_date,
                    },
                )
                .max()
            {
                Some(r) => max_date_time = r,
                None => {}
            }
            // 2日分を追加
            event_search_info_repository::set_init_event_update_history(
                location_key.clone(),
                max_date_time,
                2,
            )?;
        }
    }

    return Ok(());
}
