use std::collections::BTreeMap;
use std::env;
use std::time::{Duration, SystemTime};

use base64ct::{Base64, Encoding};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::serde::json::serde_json;
use sha2::Digest;
use sha2::Sha256;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();

    let salt = env::var("SECURITY_SALT").unwrap_or("default_salt".to_string());
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());

    Base64::encode_string(&hasher.finalize())
}


pub fn create_jwt(email: &str) -> String {
    let salt = env::var("JWT_SALT").unwrap_or("default_salt".to_string());

    let hours_to_expire = 12;
    let expire_time = Duration::from_secs(hours_to_expire * 60 * 60);
    let now = SystemTime::now();

    let key: Hmac<Sha256> = Hmac::new_from_slice(salt.as_bytes()).expect("Hmac issue");
    let mut claims: BTreeMap<&str, String> = BTreeMap::new();
    claims.insert("sub", email.to_string());
    claims.insert("expire", serde_json::to_string(&(now + expire_time)).expect("Serde issue"));
    let token_str = claims.sign_with_key(&key).expect("Sign issue");

    return token_str;
}

pub fn get_email_from_token(token: &str) -> Result<String, String> {
    let salt = env::var("JWT_SALT").unwrap_or("default_salt".to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(salt.as_bytes()).expect("Hmac issue");

    let claims: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key);

    return match claims {
        Ok(claims) => {
            let token_expire_time: SystemTime = match serde_json::from_str::<SystemTime>(&claims["expire"]) {
                Ok(token_expire_time) => {
                    token_expire_time
                }
                Err(_) => {
                    return Err("Ошибка при получении срока действия токена".to_string());
                }
            };

            let now = SystemTime::now();

            if token_expire_time < now {
                return Err("Токен устарел".to_string());
            }

            Ok(claims["sub"].clone())
        }
        Err(..) => {
            Err("Невалидный токен".to_string())
        }
    }
}