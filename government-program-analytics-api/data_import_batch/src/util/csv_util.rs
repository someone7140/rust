use serde::de::DeserializeOwned;
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

pub fn read_csv<T>(
    file_item_no: &str,
    csv_dir: &str,
) -> Result<csv::DeserializeRecordsIntoIter<File, T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    // 指定した項番のファイルのフルパスを取得
    let mut matches: Vec<PathBuf> = fs::read_dir(csv_dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| {
            path.is_file()
                && path.extension().is_some_and(|ext| ext == "csv")
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&format!("{file_item_no}_")))
        })
        .collect();
    if matches.len() != 1 {
        return Err(format!(
            "CSVファイルが一意に特定できません: type={file_item_no}, matches={}",
            matches.len()
        )
        .into());
    }

    // CSVから指定した型のIteratorに変換して返す
    let reader = csv::Reader::from_path(matches.remove(0))?;
    Ok(reader.into_deserialize::<T>())
}
