use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::requests::{
    validators::open_validator::{
        str_no_backtick,
        string_no_backtick,
        str_no_special_chars,
        string_no_special_chars,
        vec_option_string_no_special_chars,
        string_max_500_chars,
        string_max_50_chars,
        string_max_32_chars,
        string_max_15_chars,
        str_max_500_chars,
        str_max_50_chars,
        str_max_32_chars,
        str_max_15_chars,
        str_max_1000_chars,
        string_max_100_chars,
        string_max_1000_chars,
        vec_string_max_1000_chars,
        string_prompt_max_chars,

        string_name_special_chars,
        str_name_special_chars,
        string_address_special_chars,
        str_address_special_chars,
        string_username_special_chars,
        str_username_special_chars,
        string_otp_special_chars,
        str_otp_special_chars,
        string_mobgo_object_special_chars,
        str_mobgo_object_special_chars,
    }
};





#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PromptQueryRequest {
    #[validate(custom = "str_no_backtick")]
    #[validate(custom = "str_max_500_chars")]
    pub prompt: String,

    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub user_session_id: Option<String>,

    #[serde(default)]
    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub previous_prompt_id: Option<String>,

    #[serde(default)]
    pub update_data: Option<bool>,

    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub widget_id: Option<String>,

    pub alternate_visual: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PromptAttachmentFormFields {
    #[validate(custom = "str_no_backtick")]
    #[validate(custom = "str_max_500_chars")]
    pub prompt: String,

    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub user_session_id: Option<String>,

    #[serde(default)]
    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub previous_prompt_id: Option<String>,

    #[serde(default)]
    pub update_data: Option<bool>,

    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub widget_id: Option<String>,

    #[serde(default)]

    pub alternate_visual: Option<bool>,
}



#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PromptNotifyRequest {
    #[validate(custom = "string_no_backtick")]
    #[validate(custom = "string_max_50_chars")]
    pub prompt: String,


    #[validate(custom = "string_mobgo_object_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub user_session_id: Option<String>,
}




#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DashboardWidgetQuery {
    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_500_chars")]
    pub asset_codes: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_15_chars")]
    pub popular: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_15_chars")]
    pub created_by_me: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_15_chars")]
    pub following: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_15_chars")]
    pub private: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ClearWidgetQuery {
    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_1000_chars")]
    pub widget_ids: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ReadNotificationsBody {
    #[validate(custom = "vec_option_string_no_special_chars")]
    #[validate(custom = "vec_string_max_1000_chars")]
    pub notification_ids: Option<Vec<String>>,
}



#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UserProfileRequestStruct {
    // Basic info
    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub full_name: Option<String>,

    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_15_chars")]
    pub ccode: Option<String>, // country code

    #[validate(custom = "string_address_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub notes: Option<String>,

    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub language: Option<String>,

    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub cccode: Option<String>,

    // SEBI info
    #[validate(custom = "string_username_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub username: Option<String>,

    #[validate(custom = "string_address_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub bio: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    pub picture: Option<String>,

    #[validate(custom = "vec_option_string_no_special_chars")]
    #[validate(custom = "vec_string_max_1000_chars")]
    pub asset_classes: Option<Vec<String>>,

    #[validate(custom = "vec_option_string_no_special_chars")]
    #[validate(custom = "vec_string_max_1000_chars")]
    pub investment_goals: Option<Vec<String>>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub experience_level: Option<String>,
}



#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UserOnboardRequestStruct {
    // Basic info
    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub full_name: Option<String>,

    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub ccode: Option<String>, // country code

    #[validate(custom = "str_address_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub notes: Option<String>,

    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub language: Option<String>,

    // SEBI info
    #[validate(custom = "string_username_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub username: Option<String>,

    #[validate(custom = "str_address_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub bio: Option<String>,

    #[validate(custom = "string_no_special_chars")]
    pub picture: Option<String>,

    #[validate(custom = "vec_option_string_no_special_chars")]
    #[validate(custom = "vec_string_max_1000_chars")]
    pub asset_classes: Option<Vec<String>>,

    #[validate(custom = "vec_option_string_no_special_chars")]
    #[validate(custom = "vec_string_max_1000_chars")]
    pub investment_goals: Option<Vec<String>>,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub experience_level: Option<String>,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct InviteRequestStruct {
    #[validate(custom = "str_no_special_chars")]
    #[validate(custom = "str_max_32_chars")]
    pub email: String,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_100_chars")]
    pub message: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UsernameExistsStruct {
    #[validate(custom = "string_username_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub username: Option<String>,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct OAuthCustomRequestStruct {
    #[validate(custom = "str_no_special_chars")]
    #[validate(custom = "str_max_1000_chars")]
    pub access_token: String,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_1000_chars")]
    pub firebase_token: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct OTPRequest {
    #[validate(custom = "str_no_special_chars")]
    #[validate(custom = "str_max_32_chars")]
    pub identifier: String,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_32_chars")]
    pub ccode: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct VerifyOTP {
    #[validate(custom = "str_no_special_chars")]
    #[validate(custom = "str_max_32_chars")]
    pub identifier: String,

    #[validate(custom = "str_otp_special_chars")]
    #[validate(custom = "str_max_32_chars")]
    pub otp: String,

    #[validate(custom = "string_no_special_chars")]
    #[validate(custom = "string_max_1000_chars")]
    pub firebase_token: Option<String>,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UserPath {
    #[validate(custom = "str_mobgo_object_special_chars")]
    #[validate(custom = "str_max_50_chars")]
    pub user_id: String,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ReportStruct {
    #[validate(custom = "str_address_special_chars")]
    #[validate(custom = "str_max_50_chars")]
    pub reason: String,
}



#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SessionQueryStruct {
    #[validate(custom = "string_name_special_chars")]
    #[validate(custom = "string_max_50_chars")]
    pub title: Option<String>
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SessionWidgetPath {
    #[validate(custom = "str_mobgo_object_special_chars")]
    #[validate(custom = "str_max_50_chars")]
    pub session_id: String,

    #[validate(custom = "str_mobgo_object_special_chars")]
    #[validate(custom = "str_max_50_chars")]
    pub prompt_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SessionPromptPath {
    #[validate(custom = "str_mobgo_object_special_chars")]
    #[validate(custom = "str_max_50_chars")]
    pub session_id: String,
}


