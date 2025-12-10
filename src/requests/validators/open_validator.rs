use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use actix_web::{Error};
use regex::Regex;
use crate::handle_custom_error;
use crate::requests::{
    regexes::open_regex::{
        ASCII_SPECIAL_CHARS,
        TEXTUAL_NAME_REGEX,
        TEXTTUAL_ADDRESS_REGEX,
        TEXTTUAL_USERNAME_REGEX,
        TEXTUAL_OTP_REGEX,
        TEXTUAL_MONGO_OBJECT_REGEX,
    },
    messages::open_message::{
        BACKTICK_NOT_ALLOWED_MESSAGE,
        EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE,
        MAXIMUM_ALLOWED_CHARS_MESSAGE,
        TEXTUAL_NAME_REGEX_MESSAGE,
        TEXTTUAL_ADDRESS_REGEX_MESSAGE,
        TEXTTUAL_USERNAME_REGEX_MESSAGE,
        TEXTUAL_OTP_REGEX_MESSAGE,
        TEXTUAL_MONGO_OBJECT_REGEX_MESSAGE,
    },
    constants::open_constant::{
        PROMPT_MAX_CHARS,
        MAX_32_CHARS,
        MAX_50_CHARS,
        MAX_15_CHARS,
        MAX_100_CHARS,
        MAX_500_CHARS,
        MAX_1000_CHARS,
    }
};


/*---------------------------------------------------------------------
No special chars
----------------------------------------------------------------------*/
pub fn string_otp_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_OTP_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_OTP_REGEX_MESSAGE));
    }
    
    Ok(())
}

pub fn str_otp_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_OTP_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_OTP_REGEX_MESSAGE));
    }
    
    Ok(())
}


pub fn string_mobgo_object_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_MONGO_OBJECT_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_MONGO_OBJECT_REGEX_MESSAGE));
    }
    
    Ok(())
}

pub fn str_mobgo_object_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_MONGO_OBJECT_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_MONGO_OBJECT_REGEX_MESSAGE));
    }
    
    Ok(())
}


pub fn string_name_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_NAME_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_NAME_REGEX_MESSAGE));
    }
    
    Ok(())
}

pub fn str_name_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTUAL_NAME_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTUAL_NAME_REGEX_MESSAGE));
    }
    
    Ok(())
}


pub fn string_address_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTTUAL_ADDRESS_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTTUAL_ADDRESS_REGEX_MESSAGE));
    }
    
    Ok(())
}

pub fn str_address_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTTUAL_ADDRESS_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTTUAL_ADDRESS_REGEX_MESSAGE));
    }
    
    Ok(())
}


pub fn string_username_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTTUAL_USERNAME_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTTUAL_USERNAME_REGEX_MESSAGE));
    }
    
    Ok(())
}

pub fn str_username_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !TEXTTUAL_USERNAME_REGEX.is_match(value) {
        return Err(ValidationError::new(TEXTTUAL_USERNAME_REGEX_MESSAGE));
    }
    
    Ok(())
}



// Regex pattern allowing:
    // - Letters (a-z, A-Z)
    // - Numbers (0-9)
    // - Dot, underscore, hyphen, space
    // - Special symbols: !@#$%^&*()
    // - Explicitly excludes backtick (`) and all other characters
pub fn string_no_special_chars(value: &String) -> Result<(), ValidationError> {
    if (value.contains('`')) || !ASCII_SPECIAL_CHARS.is_match(value) {
        return Err(ValidationError::new(EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE));
    }
    
    Ok(())
}


pub fn str_no_special_chars(value: &str) -> Result<(), ValidationError> {
    if (value.contains('`')) || !ASCII_SPECIAL_CHARS.is_match(value) {
        return Err(ValidationError::new(EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE));
    }
    
    Ok(())
}


pub fn option_string_no_special_chars(value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(v) = value {
        if (v.contains('`')) || !ASCII_SPECIAL_CHARS.is_match(v) {
            return Err(ValidationError::new(EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE));
        }
    }
    Ok(())
}


pub fn vec_option_string_no_special_chars(value: &Vec<String>) -> Result<(), ValidationError> {
    for v in value {
        if (v.contains('`')) || !ASCII_SPECIAL_CHARS.is_match(v) {
            return Err(ValidationError::new(EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE));
        }
    }
    Ok(())
}

/*---------------------------------------------------------------------
No special chars
----------------------------------------------------------------------*/



/*---------------------------------------------------------------------
No backstick
----------------------------------------------------------------------*/
pub fn str_no_backtick(value: &str) -> Result<(), ValidationError> {
    if value.contains('`') {
        return Err(ValidationError::new(BACKTICK_NOT_ALLOWED_MESSAGE));
    }
    Ok(())
}

pub fn string_no_backtick(value: &String) -> Result<(), ValidationError> {
    if value.contains('`') {
        return Err(ValidationError::new(BACKTICK_NOT_ALLOWED_MESSAGE));
    }
    Ok(())
}

pub fn option_string_no_backtick(value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(v) = value {
        if v.contains('`') {
            return Err(ValidationError::new(BACKTICK_NOT_ALLOWED_MESSAGE));
        }
    }
    Ok(())
}



pub fn vec_option_string_no_backtick(value: &Vec<String>) -> Result<(), ValidationError> {
    for v in value {
        if v.contains('`') {
            return Err(ValidationError::new(BACKTICK_NOT_ALLOWED_MESSAGE));
        }
    }
    Ok(())
}
/*---------------------------------------------------------------------
No backstick
----------------------------------------------------------------------*/



/*---------------------------------------------------------------------
START Max Chars
----------------------------------------------------------------------*/
// String specific
pub fn string_prompt_max_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > PROMPT_MAX_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, PROMPT_MAX_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn string_max_15_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_15_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_15_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn string_max_32_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_32_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_32_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn string_max_50_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_100_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_100_CHARS).into());
        return Err(err);
    }
    Ok(())
}

pub fn string_max_100_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_100_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_100_CHARS).into());
        return Err(err);
    }
    Ok(())
}

pub fn string_max_500_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_500_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_500_CHARS).into());
        return Err(err);
    }
    Ok(())
}

pub fn string_max_1000_chars(value: &String) -> Result<(), ValidationError> {
    if value.len() > MAX_1000_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_1000_CHARS).into());
        return Err(err);
    }
    Ok(())
}


// Vec specific
pub fn vec_string_max_1000_chars(value: &Vec<String>) -> Result<(), ValidationError> {
    for v in value {
        if v.len() > MAX_1000_CHARS {
            let mut err = ValidationError::new("length");
            err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_100_CHARS).into());
            return Err(err);
        }
    }
    Ok(())
}


// Str specific
pub fn str_prompt_max_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > PROMPT_MAX_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, PROMPT_MAX_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn str_max_15_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_15_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_15_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn str_max_32_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_32_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_32_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn str_max_50_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_100_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_100_CHARS).into());
        return Err(err);
    }
    Ok(())
}


pub fn str_max_100_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_100_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_100_CHARS).into());
        return Err(err);
    }
    Ok(())
}

pub fn str_max_500_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_500_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_500_CHARS).into());
        return Err(err);
    }
    Ok(())
}

pub fn str_max_1000_chars(value: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_1000_CHARS {
        let mut err = ValidationError::new("length");
        err.message = Some(format!("{}: {}", MAXIMUM_ALLOWED_CHARS_MESSAGE, MAX_1000_CHARS).into());
        return Err(err);
    }
    Ok(())
}



/*---------------------------------------------------------------------
END Max Chars
----------------------------------------------------------------------*/

/// Validate event date is in the future
pub fn validate_future_date(date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    
    if *date <= now {
        return Err(ValidationError::new("event_date_not_future"));
    }
    
    // Check if date is not too far in the future (e.g., 5 years)
    let max_future_date = now + chrono::Duration::days(365 * 5);
    if *date > max_future_date {
        return Err(ValidationError::new("event_date_too_far_future"));
    }
    
    Ok(())
}


/// Validate event end date is after start date
pub fn validate_event_dates(start_date: &chrono::DateTime<chrono::Utc>, end_date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    if end_date <= start_date {
        return Err(ValidationError::new("end_date_before_start_date"));
    }
    
    // Check if event duration is not too long (e.g., 30 days)
    let duration = *end_date - *start_date;
    if duration.num_days() > 30 {
        return Err(ValidationError::new("event_duration_too_long"));
    }
    
    Ok(())
}



pub fn option_string_max_5000_chars(value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(v) = value {
        if v.len() > PROMPT_MAX_CHARS {
            let message = format!("MAXIMUM_ALLOWED_CHARS: {:?}", PROMPT_MAX_CHARS);
            return Err(ValidationError::new("MAXIMUM_ALLOWED_CHARS excess"));
        }
    }
    Ok(())
}



pub async fn validate_params<T: Validate>(payload: &T) -> Result<(), Error> {
    if let Err(e) = payload.validate() {
        return Err(handle_custom_error!(bad_request, 400, e.to_string()));
    }
    Ok(())
}