// /Users/xsm/Documents/workspace/XARD/xard-be/src/requests/enums/event_enums.rs


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventStatusEnum {
    Draft,
    Published,
    Cancelled,
    Completed,
    Initial,
}

impl Default for EventStatusEnum {
    fn default() -> Self {
        EventStatusEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ApprovalStatusEnum {
    Pending,
    Approved,
    Rejected,
    Initial,
}

impl Default for ApprovalStatusEnum {
    fn default() -> Self {
        ApprovalStatusEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventCategoryEnum {
    Business,
    Technology,
    Education,
    Entertainment,
    Sports,
    Health,
    FoodAndDrink,
    ArtAndCulture,
    Networking,
    Conference,
    Workshop,
    Meetup,
    Webinar,
    Seminar,
    Other,
    Initial,
}

impl Default for EventCategoryEnum {
    fn default() -> Self {
        EventCategoryEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventTypeEnum {
    Physical,
    Virtual,
    Hybrid,
    Initial,
}

impl Default for EventTypeEnum {
    fn default() -> Self {
        EventTypeEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RegistrationTypeEnum {
    Required,
    Optional,
    NotRequired,
    Initial,
    Open,
    Invited,
    InviteOnly,
    Paid,
    Waitlist,
}

impl Default for RegistrationTypeEnum {
    fn default() -> Self {
        RegistrationTypeEnum::Initial
    }
}

impl RegistrationTypeEnum {
    pub fn lowercase(&self) -> String {
        match self {
            RegistrationTypeEnum::Initial => "initial",
            RegistrationTypeEnum::Open => "open",
            RegistrationTypeEnum::Invited => "invited",
            RegistrationTypeEnum::InviteOnly => "invite_only",
            RegistrationTypeEnum::Paid => "paid",
            RegistrationTypeEnum::Waitlist => "waitlist",
            RegistrationTypeEnum::Required => "required",
            RegistrationTypeEnum::Optional => "optional",
            RegistrationTypeEnum::NotRequired => "notrequired",
        }
        .to_string()
    }
} 

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PaymentTypeEnum {
    Free,
    Paid,
    Donation,
    Initial,
}

impl Default for PaymentTypeEnum {
    fn default() -> Self {
        PaymentTypeEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AgeRestrictionEnum {
    AllAges,
    AdultsOnly,
    ChildrenOnly,
    Teenagers,
    Seniors,
    Custom(String),
    Initial,
}

impl Default for AgeRestrictionEnum {
    fn default() -> Self {
        AgeRestrictionEnum::Initial
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DressCodeEnum {
    Casual,
    BusinessCasual,
    Business,
    Formal,
    BlackTie,
    Costume,
    Sportswear,
    Custom(String),
    Initial,
}

impl Default for DressCodeEnum {
    fn default() -> Self {
        DressCodeEnum::Initial
    }
}
