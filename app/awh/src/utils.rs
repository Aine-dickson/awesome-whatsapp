use std::env;

use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, HttpMessage};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub async fn jwt_guard(req: ServiceRequest, next: Next<impl MessageBody>)-> Result<ServiceResponse<impl MessageBody>, actix_web::Error>{
    let auth_cookie = req.cookie("auth");
    if let None = auth_cookie {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }

    match verify_jwt(auth_cookie.unwrap().value(), "secret".to_owned()) {
        Some(claims) => {
            req.extensions_mut().insert(claims);
            return next.call(req).await;
        }
        None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Vec<u8>,
    pub tel_contact: String,
    pub verified: i8
}

pub fn verify_jwt(token:&str, secret: String )->Option<Claims>{
    let claims = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &jsonwebtoken::Validation::default());
    match claims {
        Ok(value) => return Some(value.claims),
        Err(_) => return None,
    }
}

pub fn assign_jwt(claims: Claims)-> String {
    let secret = env::var("AUTH_KEY").unwrap();
    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}