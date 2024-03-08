use async_graphql::*;
use url::Url;

use crate::graphql_object::horse_enum::ErrorType;

// urlからhtmlを取得
pub async fn get_html_from_url(url: &String) -> Result<String> {
    match reqwest::get(url).await {
        Ok(r) => match r.text().await {
            Ok(text) => Ok(text),
            Err(error) => {
                return Err(Error::new(error.to_string())
                    .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
            }
        },
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    }
}

// urlからページを取得
pub fn get_page_from_url(url: &String) -> Result<String> {
    match Url::parse(&url) {
        Ok(u) => match u.path().split("/").last() {
            Some(p) => Ok(p.to_string()),
            None => Err(Error::new("Can not get page")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest))),
        },
        Err(error) => {
            Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    }
}
