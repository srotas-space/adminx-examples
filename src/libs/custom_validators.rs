use validator::ValidationError;
use regex::Regex;
use mongodb::{bson::{oid::ObjectId}};

use crate::libs::custom_message::{
    INVALID_EMAIL, INVALID_PHONE, INVALID_IDENTIFIER, INVALID_MESSAGE_CHARS,
    INVALID_DIGITS_ONLY, INVALID_ALPHA_ONLY,
    INVALID_ALPHANUMERIC, INVALID_USERNAME, INVALID_WEBSITE,
};

use crate::libs::custom_regex::{
    EMAIL_REGEX, DIGIT_ONLY_REGEX, ALPHA_ONLY_REGEX, ALPHANUMERIC_REGEX,
    IDENTIFIER_REGEX, MESSAGE_REGEX, USERNAME_REGEX, WEBSITE_REGEX
};


pub fn ignore_none(val: &Option<&str>) -> bool {
    match val {
        None => true,
        Some(v) => v.trim().is_empty(),
    }
}


// ✅ Email Validator
pub fn validate_email(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if EMAIL_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid email");
        err.message = Some(INVALID_EMAIL.into());
        Err(err)
    }
}

// ✅ Phone Validator
pub fn validate_phone(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if DIGIT_ONLY_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid phone");
        err.message = Some(INVALID_PHONE.into());
        Err(err)
    }
}

// Validate username
pub fn validate_username(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if USERNAME_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid username");
        err.message = Some(INVALID_USERNAME.into());
        Err(err)
    }
}


// Validate website
pub fn validate_website(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if WEBSITE_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid website");
        err.message = Some(INVALID_WEBSITE.into());
        Err(err)
    }
}


// ✅ Identifier (Email or Phone) Validator
pub fn validate_identifier(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if IDENTIFIER_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid identifier");
        err.message = Some(INVALID_IDENTIFIER.into());
        Err(err)
    }
}

// ✅ Message Validator
pub fn validate_message(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if MESSAGE_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid message_format");
        err.message = Some(INVALID_MESSAGE_CHARS.into());
        Err(err)
    }
}

// ✅ Digits Only
pub fn validate_digits_only(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if DIGIT_ONLY_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid digits only");
        err.message = Some(INVALID_DIGITS_ONLY.into());
        Err(err)
    }
}

// ✅ Alpha Only
pub fn validate_alpha_only(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if ALPHA_ONLY_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid alpha only");
        err.message = Some(INVALID_ALPHA_ONLY.into());
        Err(err)
    }
}

// ✅ Alphanumeric
pub fn validate_alphanumeric(val: &str) -> Result<(), ValidationError> {
    let input = Some(val);
    if ignore_none(&input) {
        return Ok(());
    }

    if ALPHANUMERIC_REGEX.is_match(val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid alphanumeric");
        err.message = Some(INVALID_ALPHANUMERIC.into());
        Err(err)
    }
}


pub fn option_validate_alphanumeric(val: String) -> Result<(), ValidationError> {
    let input = Some(val.clone());
    if ignore_none(&input.as_deref()) {
        return Ok(());
    }

    if ALPHANUMERIC_REGEX.is_match(&val) {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid alphanumeric");
        err.message = Some(INVALID_ALPHANUMERIC.into());
        Err(err)
    }
}


pub fn validate_object_required(val: &ObjectId) -> Result<(), ValidationError> {
    if val == &ObjectId::default() {
        let mut err = ValidationError::new("required");
        err.message = Some("ObjectId is required".into());
        Err(err)
    } else {
        Ok(())
    }
}


