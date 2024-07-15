use actix_web::HttpRequest;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::entity::user::UserId;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i32,
}

pub fn get_user_id_from_jwt(token: &str, secret: &[u8]) -> Result<String, anyhow::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims.sub)
}

pub fn get_user_id_from_req(req: HttpRequest) -> Result<UserId, anyhow::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let token = auth_header.trim_start_matches("Bearer ");

    println!("{}", token);

    // JWTトークンからユーザーIDを抽出
    let user_id = get_user_id_from_jwt(token, b"your_secret_key").unwrap();
    Ok(UserId::new(user_id).unwrap())
}
