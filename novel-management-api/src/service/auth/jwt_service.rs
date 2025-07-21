use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub const TEMP_TOKEN_EXP_HOURS: u64 = 3;
pub const STORE_TOKEN_EXP_HOURS: u64 = 4320;

pub const JWT_GMAIL_KEY: &str = "gmail";
pub const JWT_IMAGE_URL_KEY: &str = "image_url";
pub const JWT_USER_ACCOUNT_ID_KEY: &str = "user_account_id";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub contents: HashMap<String, String>,
    iat: i64,
    exp: i64,
}

pub fn make_jwt(secret: &String, contents: HashMap<String, String>, exp_hours: u64) -> String {
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::from_secs(exp_hours * 60 * 60)).timestamp();
    let my_claims = Claims {
        contents: contents.clone(),
        iat,
        exp,
    };

    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_jwt(
    jwt: &str,
    secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
