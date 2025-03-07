use std::error::Error;

use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::config::UserServiceConfig;

#[derive(Debug, Serialize, Deserialize)]
struct Claims228 {
    iat: u64, // Time issued
    exp: u64, // When expired
    user_uuid: Uuid,
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
    let jwt = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
    )?;
    return Ok(jwt);
}

pub fn validate_token(token: String, public_key: String) -> Result<Uuid, Box<dyn Error>> {
    let claims = decode::<Claims228>(
        &token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .unwrap();
    Ok(claims.claims.user_uuid)
}

#[cfg(test)]
mod tests {
    use deadpool_postgres::Config;

    use super::*;

    #[test]
    fn check_token_validation() {
        let public_key = "-----BEGIN PUBLIC KEY-----
            MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAkH21eCClmEJ+avRrUclG
            ijv9/L13521NTc+cP1kzKXcQaCCHjw7dgcqKFUEspxIOooqObGlS3cy6HG/SyTT1
            E6K+MZrnk7QrL0Im180WTcADODqu1Kbn8Tym9sgKP8ic0kvYloE7+fLPqD+7oylf
            csaCpWDt22PXPRDsQzB/w6RqPEEjiCrCRR0VOP+TE5MWtdCQrSK6hE8JFSl7wPHy
            QPzKHLWg1tIt3+/SNA+Jj3w0kUZ3abCxHBJaz+oGgJZZ19cs46P8FOzvLeaSIUDS
            W1zuHCl5siEjML5FGBurObmCk06tPTi5v2Dzt+5SXUcjvuyE5avNc5TfB6804q7K
            GwIDAQAB
            -----END PUBLIC KEY-----
            "
        .to_string();
        let private_key = "-----BEGIN PRIVATE KEY-----
            MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCQfbV4IKWYQn5q
            9GtRyUaKO/38vXfnbU1Nz5w/WTMpdxBoIIePDt2ByooVQSynEg6iio5saVLdzLoc
            b9LJNPUTor4xmueTtCsvQibXzRZNwAM4Oq7UpufxPKb2yAo/yJzSS9iWgTv58s+o
            P7ujKV9yxoKlYO3bY9c9EOxDMH/DpGo8QSOIKsJFHRU4/5MTkxa10JCtIrqETwkV
            KXvA8fJA/MoctaDW0i3f79I0D4mPfDSRRndpsLEcElrP6gaAllnX1yzjo/wU7O8t
            5pIhQNJbXO4cKXmyISMwvkUYG6s5uYKTTq09OLm/YPO37lJdRyO+7ITlq81zlN8H
            rzTirsobAgMBAAECggEAOcfDa0vm3InuotemGPkbt8jAyzJ7QfUgHjcg/OYvQuwl
            h/ZGL0ubAb2cVV6QPClI5+/vf2Po6uBlJl0DVknSnW0NkKmo5JR+VnImNVMCkcnw
            n6o74rbx9rKx8+rPymzvZVtGO/1nHg9ewiHmZ5tyW2QNt5OY2oxE9NsRF9dLqFC7
            6mUfp+QFO0Noa/2sLdbSUa39MgvzJkDV8S6oD+omxdUI7D+1czePyxzBVGn71F4H
            QGLY2lBzp6FhzaULeoc0+CV9yPd8vBe1OEYiJFgZT1BpsVX/c6oU509LpXQesLDY
            9bpTD4l/cQ3+bGfY1bcJ4/L2F7RCt0zapnRWC0xWTQKBgQDHwbpqEJdJqHgep/Qx
            J/gAw8zgO0VFydmose5HcYNvfE6iAK5aMHhtY6NQnzdreUY0Ueqh/93wQl3O+6MY
            G9Yg6VtnNPMeKmXpbo0KoXCnZuFzNtwQvJIQSA4NBvjapvntJV+5hwrAdpZMmOFw
            ci6KAQw9VdLNPWli2VzhBHtynQKBgQC5LHueF3INjfigPbsIVMor+P7jrvp18Uhb
            nZ1kUL0Ql8ccp0wUye0eo7VKSXIA2Nz485S7JmZy4T72UmLO9rNX0e5yVK3e9eXu
            nJYNyasK0YjRhwSUlRe9crOFZjlh/AUVPFtmjEOAycqBqocazYYsnI2no+h4NBpr
            p6/KK1YWFwKBgHBaMTxI2p7OR8mOU9V2EpJzjn8sNxk6n9a3RTsfyHpjI2MRGP+X
            NZTpLslA1A355xe0X1EAtdbVrSem9yOvPG/EcOmKqZd05181E9U/2jn9rp5746jy
            NwE7VdPT8RIiSn4swDt8jfqUeJLahioVvsFWN4kkBW30tZqogQn62GgtAoGBAKo5
            YDJGb0xSlJoJ5WOVPn0J2pbHheriMZg23jcXmZYlUJ7glCoQLnCaIc/2kFyC/y/E
            C5e3hzpYIh8iQT7svpUeoUps2aE6/3JIdBCcsSWy1Ul4Cxfjrv0y8iDRhjKzEKEi
            6QruKie8WPK2JpjXEl70U6wmG1BU5WzNq+X3zJ1lAoGACnFo7Rw3cj8WDXGk7OqG
            SUCmlEmit2hzeLdpiGTr9ZEaTOBbi9OlcokKbryAEeKCpoB+0WvJsgOkAk76lktD
            Y3+fFNq+qrwiXQk7Hp8FqQB4HhmJeRF7+OV2B8glbz275c5FrTkfjb1c4/cBN0vy
            7JQbBWjFyXzQWtFqXc4e/8Y=
            -----END PRIVATE KEY-----
            "
        .to_string();
        let config = UserServiceConfig {
            postgres_config: Config::new(),
            public_key: public_key.clone(),
            private_key,
            token_duration: 123,
        };
        let uuid = Uuid::new_v4();
        let token_res = create_token(uuid, &config);
        assert!(token_res.is_ok());
        let token = token_res.unwrap();
        let uuid_res = validate_token(token, public_key);
        assert!(uuid_res.is_ok());
        assert_eq!(uuid, uuid_res.unwrap());
    }
}
