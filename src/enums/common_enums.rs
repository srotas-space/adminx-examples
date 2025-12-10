// /Users/xsm/Documents/workspace/XARD/xard-be/src/enums/common_enums.rs


use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;
use serde_json::Value;
use serde_with::DisplayFromStr;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum GenderEnum {
    Initial,
    Male,
    Female,
    Other,
}


impl GenderEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            GenderEnum::Male => "Male",
            GenderEnum::Female => "Female",
            GenderEnum::Other => "Other",
            GenderEnum::Initial => "",
        }
    }
}

pub fn deserialize_gender<'de, D>(deserializer: D) -> Result<Option<GenderEnum>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s.map(|s| s.to_lowercase()) {
        Some(ref val) if val == "male" => Ok(Some(GenderEnum::Male)),
        Some(ref val) if val == "female" => Ok(Some(GenderEnum::Female)),
        Some(ref val) if val == "other" => Ok(Some(GenderEnum::Other)),
        Some(other) => Err(serde::de::Error::custom(format!("Invalid gender: {}", other))),
        None => Ok(None),
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum StatusEnum {
    Active,
    Inactive,
    Initial,
    Pending,
    Finished,
    Accepted,
    Declined,
    Expired,
    Blocked,
    Archived

}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum OnboardEnum {
    Completed,
    Initial,
}


impl Default for GenderEnum {
    fn default() -> Self {
        GenderEnum::Initial
    }
}

impl Default for StatusEnum {
    fn default() -> Self {
        StatusEnum::Initial
    }
}


impl StatusEnum {
    pub fn lowercase(&self) -> String {
        match self {
            StatusEnum::Initial => "initial",
            StatusEnum::Active => "active",
            StatusEnum::Inactive => "inactive",
            StatusEnum::Blocked => "blocked",
            StatusEnum::Archived => "archived",
            StatusEnum::Pending => "pending",
            StatusEnum::Finished => "finished",
            StatusEnum::Accepted => "accepted",
            StatusEnum::Declined => "declined",
            StatusEnum::Expired => "expired",
        }
        .to_string()
    }
} 


#[derive(Debug, serde::Deserialize)]
pub struct FetchContactRequest {
    pub query: Option<String>,
    pub favorite: Option<bool>,
}


#[derive(Debug, serde::Deserialize)]
pub struct FetchNotificationRequest {
    pub query: Option<String>,
    pub deleted: Option<bool>,
}


#[derive(Deserialize)]
pub struct QRRequest {
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct InviteRequest {
    pub email: String,
    pub message: Option<String>,
}


#[derive(Deserialize)]
pub struct SupportRequest {
    pub subject: String,
    pub description: String,
}


#[derive(Debug, serde::Deserialize)]
pub struct LinkedInCallback {
    pub code: Option<String>,
    pub error: Option<String>,
    pub state: Option<String>,
}


#[derive(Debug, serde::Deserialize)]
pub struct ScanRQCodeStruct {
    pub user_id: Option<String>,
    pub event_id: Option<String>,
}
