use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static::lazy_static! {
    static ref PHONE_REGEX: Regex = Regex::new(r"^\+[1-9]\d{7,14}$").unwrap();
}

lazy_static::lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserDto {
    #[validate(length(min="5", max="15"), regex(path= *STRING_REGEX))]
    pub username: String,

    #[validate(email)]
    pub email_address: String,

    #[validate(regex(path= *PHONE_REGEX), length(min="3", max="15"))]
    pub tel_contact: String
}