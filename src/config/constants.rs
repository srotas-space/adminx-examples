// src/config/constants.rs

pub const PRODUCT_NAME: &str = "Srotas Space";
pub const WEBSITE_URI: &str = "https://srotas.space";
pub const SUPPORT_URI: &str = "https://srotas.space/support";

pub const DEFAULT_PAGE: i64 = 1;
pub const DEFAULT_LIMIT: i64 = 10;
pub const MAX_LIMIT: i64 = 100;


pub const EMAIL_GENERAL_LABEL: &str = "Srotas Space";


pub static OTP_EXPIRY: usize = 300; // 5 minutes
pub static JWT_EXPIRY_SECONDS: i64 = 2628000; // 1 Month
