use std::error::Error;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims228 {
    iat: u64, // Time issued
    exp: u64, // When expired
    user_uuid: Uuid,
}

pub fn validate_token(token: String, public_key: String) -> Result<Uuid, Box<dyn Error>> {
    println!("token in: {}", token.clone());
    let claims = decode::<Claims228>(
        &token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .unwrap();
    Ok(claims.claims.user_uuid)
}
