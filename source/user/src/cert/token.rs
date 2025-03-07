use std::{error::Error};

use jsonwebtoken::{decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::config::UserServiceConfig;


#[derive(Debug, Serialize, Deserialize)]
struct Claims228 {
    iat: u64, // Time issued
    exp: u64, // When expired
    user_uuid: Uuid
}

pub fn create_token(user_uuid: Uuid, config: &UserServiceConfig) -> Result<String, Box<dyn Error>> {
    let mut header = Header::new(Algorithm::RS256);
    header.typ = Some("JWT".to_string());
    let iat = get_current_timestamp();
    let exp = iat + config.token_duration;
    let claims = Claims228 {
        iat,
        exp,
        user_uuid,
    };
    let private_key = config.private_key.as_str();
    let jwt = encode(&header, &claims, &EncodingKey::from_rsa_pem( private_key.as_bytes())?)?;
    return Ok(jwt);
}

pub fn validate_token(token: String, public_key: String) -> Result<Uuid, Box<dyn Error>> {
    let claims = decode::<Claims228>(&token, &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(), &Validation::new(Algorithm::RS256)).unwrap();
    Ok(claims.claims.user_uuid)
}
