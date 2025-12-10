use validator::ValidationError;
use regex::Regex;
use mongodb::{bson::{oid::ObjectId}};

use crate::requests::{
    regexes::open_regex::{
        EMAIL_REGEX
    }
};


pub fn validate_email(val: &str) -> bool {
    EMAIL_REGEX.is_match(val)
}