use validator::ValidationError;
use regex::Regex;
use mongodb::{bson::{oid::ObjectId}};

use crate::requests::{
    regexes::open_regex::{
        EMAIL_REGEX,
        PHONE_REGEX,
        USERNAME_REGEX,
        DIGIT_ONLY_REGEX,
        ALPHA_ONLY_REGEX,
        ALPHANUMERIC_REGEX,
        TEXTTUAL_REGEX,
    },
    messages::open_message::{
        INVALID_EMAIL_MESSAGE,
        INVALID_PHONE_MESSAGE,
        INVALID_USERNAME_MESSAGE,
        INVALID_DIGITS_ONLY_MESSAGE,
        INVALID_ALPHA_ONLY_MESSAGE,
        INVALID_ALPHANUMERIC_MESSAGE,
        INVALID_TEXTUAL_CHARS_MESSAGE,
    },
    constants::open_constant::{
        PROMPT_MAX_CHARS,
        MAX_15_CHARS,
        MAX_32_CHARS,
        MAX_50_CHARS,
        MAX_100_CHARS,
        MAX_500_CHARS,
        MAX_1000_CHARS
    }
};


pub fn validate_email(val: &str) -> Result<(), ValidationError> {
    if EMAIL_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_email");
        err.message = Some(INVALID_EMAIL_MESSAGE.into());
        Err(err)
    }
}

pub fn validate_phone(val: &str) -> Result<(), ValidationError> {
    if PHONE_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_phone");
        err.message = Some(INVALID_PHONE_MESSAGE.into());
        Err(err)
    }
}


pub fn validate_username(val: &str) -> Result<(), ValidationError> {
    if USERNAME_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_message_format");
        err.message = Some(INVALID_USERNAME_MESSAGE.into());
        Err(err)
    }
}


pub fn validate_digits_only(val: &str) -> Result<(), ValidationError> {
    if DIGIT_ONLY_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_digits_only");
        err.message = Some(INVALID_DIGITS_ONLY_MESSAGE.into());
        Err(err)
    }
}

pub fn validate_alpha_only(val: &str) -> Result<(), ValidationError> {
    if ALPHA_ONLY_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_alpha_only");
        err.message = Some(INVALID_ALPHA_ONLY_MESSAGE.into());
        Err(err)
    }
}

pub fn validate_alphanumeric(val: &str) -> Result<(), ValidationError> {
    if ALPHANUMERIC_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_alphanumeric");
        err.message = Some(INVALID_ALPHANUMERIC_MESSAGE.into());
        Err(err)
    }
}


pub fn validate_textual(val: &str) -> Result<(), ValidationError> {
    if TEXTTUAL_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_message_format");
        err.message = Some(INVALID_TEXTUAL_CHARS_MESSAGE.into());
        Err(err)
    }
}

