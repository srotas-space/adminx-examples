use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use validator::{Validate, ValidationError};
use crate::requests::validators::open_validator::{string_no_backtick, str_no_backtick};
use crate::enums::{
    common_enums::{
        GenderEnum,
        StatusEnum,
        deserialize_gender,
    }
};
use crate::libs::custom_validators::{
    validate_alpha_only,
    validate_email,
    validate_phone,
    validate_alphanumeric,
    validate_object_required,
    validate_message,
    validate_username,
    validate_website,
};

#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct UserProfileRequest {
    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_alpha_only")]
    pub first_name: Option<String>,

    #[validate(custom = "string_no_backtick")]
    pub last_name: Option<String>,

    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,


    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_username")]
    pub username: Option<String>,


    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_email")]
    pub official_email: Option<String>,


    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_phone")]
    pub phone_number: Option<String>,

    #[validate(custom = "validate_alphanumeric")]
    #[validate(custom = "string_no_backtick")]
    pub ccode: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub age: Option<u8>,

    #[serde(default, deserialize_with = "deserialize_gender")]
    pub gender: Option<GenderEnum>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub designation: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub description: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub firebase_token: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub linkedin: Option<String>,

    pub linkedin_datum: Option<Value>,

    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_message")]
    pub company_name: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub company_address: Option<String>,

    #[validate(custom = "validate_website")]
    #[validate(custom = "string_no_backtick")]
    pub website: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub notes: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub protection_secret: Option<String>,
    pub x_platform: Option<String>,
}


#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct ContactUpdateRequest {
    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_alpha_only")]
    pub first_name: Option<String>,


    #[validate(custom = "string_no_backtick")]
    pub last_name: Option<String>,


    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_email")]
    pub email: Option<String>,


    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_email")]
    pub official_email: Option<String>,


    #[validate(required)]
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "validate_phone")]
    pub phone_number: Option<String>,


    #[serde_as(as = "Option<DisplayFromStr>")]
    pub age: Option<u8>,
    #[serde(default, deserialize_with = "deserialize_gender")]
    pub gender: Option<GenderEnum>,

    #[validate(custom = "validate_username")]
    #[validate(custom = "string_no_backtick")]
    pub username: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub designation: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub linkedin: Option<String>,

    pub linkedin_datum: Option<Value>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub description: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub notes: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub company_name: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub company_address: Option<String>,

    #[validate(custom = "validate_website")]
    #[validate(custom = "string_no_backtick")]
    pub website: Option<String>,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub x_platform: Option<String>,

    pub favorite: Option<bool>,
}



#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct OAuthRequest {
    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub code: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct OAuthCustomRequestStruct {
    #[validate(custom = "str_no_backtick")]
    pub access_token: String,
    #[validate(custom = "string_no_backtick")]
    pub firebase_token: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct OTPRequest {
    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub identifier: String,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub ccode: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub struct VerifyOTP {
    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub identifier: String,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub otp: String,

    #[validate(custom = "validate_message")]
    #[validate(custom = "string_no_backtick")]
    pub firebase_token: Option<String>,
}

