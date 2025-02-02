use std::time::Duration;

use actix_web::error::{ErrorConflict, ErrorInternalServerError};
use resend_rs::{types::CreateEmailBaseOptions, Resend};
use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use entity::{prelude::*, user, verification};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::{thread_rng, Rng};

use crate::utils::{assign_jwt, Claims};

use super::user_dto::UserDto;

pub async fn create(user_body: UserDto, db: DatabaseConnection)-> Result<String, actix_web::Error> {
    let item = User::find().filter(
        user::Column::TelContact.eq(&user_body.tel_contact)
    ).one(&db).await;

    if let Err(_) = item {
        return Err(ErrorInternalServerError("Failed to save user"));
    }
    
    if let Some(_) = item.unwrap() {
        return Err(ErrorConflict("Tel contact already exist"));   
    }

    let new_user = ActiveUser {
        id: Set(Uuid::new_v4().into()),
        username: Set(user_body.username),
        email_address: Set(user_body.email_address),
        tel_contact: Set(user_body.tel_contact),
        verified: Set(0),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now())
    };

    let created_user = new_user.insert(&db).await;
    if let Err(_) = created_user {
        return Err(ErrorInternalServerError("Failed to save user"));
    }

    let user = created_user.unwrap();
    let claims = Claims {
        id: user.id,
        tel_contact: user.tel_contact,
        verified: user.verified
    };

    let mut rng = thread_rng();
    let code: u32 = rng.gen_range(100_000..1_000_000);

    let resend = Resend::new("re_ViTTdWCp_Lu7Q5rZypkMw8ANDyaiC3CuG");
    let from = "ainedixon01@gmail.com";
    let to = [user.email_address.clone()];
    let subject = "Verify Account";
    let html = format!("Subject: Verify Your Email Address

Hi [User],

Enter this 6 digit code in the verification page to verify your email address: 
<span style=\"display: block\">{}</span>

If you didn’t create an account, please ignore this email.

Thanks,
Wassap Team
", code);

    let email = CreateEmailBaseOptions::new(from, to, subject)
        .with_html(&html);

    let result = resend.emails.send(email).await;

    if let Err(err) = result {
        let error = format!("Error while sending verification email {}", err);
        return Err(ErrorInternalServerError(error));
    }

    let new_verification = ActiveVerification {
        id: Set(Uuid::new_v4().into()),
        email_address: Set(user.email_address),
        token: Set(code as i64),
        expiry: Set((chrono::Utc::now() + Duration::from_secs(3600)).naive_utc()),
    };

    let verification = new_verification.insert(&db).await;
    if let Err(_) = verification {
        return Err(ErrorInternalServerError("Failed to save verification"));
    }


    Ok(assign_jwt(claims))
}

pub async fn verify_account(token: String, db: DatabaseConnection) -> Result<LoggedUser, actix_web::Error> {
    let verification = Verification::find().filter(
        verification::Column::Token.eq(&token)
    ).one(&db).await;

    if let Err(_) = verification {
        return Err(ErrorInternalServerError("Failed to verify account"));
    }

    if let None = verification.as_ref().unwrap() {
        return Err(ErrorConflict("Invalid verification token"));
    }

    let now = chrono::Utc::now().naive_utc();

    if let Some(verification) = verification.as_ref().unwrap() {
        if verification.expiry < now {
            return Err(ErrorConflict("Verification token expired"));
        }
    }

    let verification = verification.as_ref().unwrap().as_ref().unwrap();
    let user = User::find().filter(
        user::Column::EmailAddress.eq(&verification.email_address)
    ).one(&db).await;

    if let Err(_) = user {
        return Err(ErrorInternalServerError("Failed to verify account"));
    }

    if let None = user.as_ref().unwrap() {
        return Err(ErrorConflict("Invalid verification token"));
    }

    let mut user: ActiveUser = user.unwrap().unwrap().into();
    user.verified = Set(1);
    let user = user.update(&db).await;

    if let Err(_) = user {
        return Err(ErrorInternalServerError("Failed to verify account"));
    }

    let user = user.unwrap();
    let user = LoggedUser {
        username: user.username,
        tel_contact: user.tel_contact,
        id: Uuid::from_bytes(user.id.try_into().expect("Expected a Vec of length 16")),
        verified: user.verified,
        email_address: user.email_address,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    };

    Ok(user)

}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub id: Uuid,
    pub tel_contact: String,
    pub verified: i8,
    pub username: String,
    pub email_address: String,
    pub created_at: String,
    pub updated_at: String,
}