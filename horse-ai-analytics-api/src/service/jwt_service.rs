use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    contents: String,
    iat: i64,
    exp: i64,
}

pub fn make_jwt(secret: &String, contents: &String, exp_hours: u64) -> String {
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

fn decode_jwt(jwt: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
