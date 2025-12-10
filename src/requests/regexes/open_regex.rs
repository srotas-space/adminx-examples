use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MESSAGE_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_\-=, ]*$"
    ).unwrap();

    pub static ref TEXTTUAL_REGEX: Regex = Regex::new(
        r"^[\x00-\x7F]*$"
    ).unwrap();

    pub static ref TEXTUAL_NAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9\s]*$"
    ).unwrap();

    pub static ref TEXTTUAL_ADDRESS_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_\-=,;\s]*$"
    ).unwrap();

    pub static ref TEXTTUAL_USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_\-.]*$"
    ).unwrap();

   pub static ref TEXTUAL_OTP_REGEX: Regex = Regex::new(
        r"^[0-9]{6}$"
    ).unwrap();

    pub static ref TEXTUAL_MONGO_OBJECT_REGEX: Regex = Regex::new(
        r"^[a-fA-F0-9]{24}$"
    ).unwrap();

    pub static ref IDENTIFIER_REGEX: Regex = Regex::new(
        r"^(?:\+?\d{10,15}|[\w\.-]+@[\w\.-]+\.\w{2,})$"
    ).unwrap();

    // ✅ Validates standard email addresses
    pub static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[\w\.-]+@[\w\.-]+\.\w{2,}$"
    ).expect("Invalid EMAIL_REGEX");

    // ✅ Validates phone numbers (10 to 15 digits, optional leading '+')
    pub static ref PHONE_REGEX: Regex = Regex::new(
        r"^\+?\d{10,15}$"
    ).expect("Invalid PHONE_REGEX");

    // ✅ Only digits (0–9)
    pub static ref DIGIT_ONLY_REGEX: Regex = Regex::new(
        r"^\d+$"
    ).expect("Invalid DIGIT_ONLY_REGEX");

    // ✅ Only alphabets (a–z, A–Z)
    pub static ref ALPHA_ONLY_REGEX: Regex = Regex::new(
        r"^[a-zA-Z ]+$"
    ).expect("Invalid ALPHA_ONLY_REGEX");

    pub static ref ALPHANUMERIC_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9 ]+$"
    ).expect("Invalid ALPHANUMERIC_REGEX");

    pub static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_\-.]*$"
    ).expect("Invalid USERNAME_REGEX");

    pub static ref NON_EMPTY_REGEX: Regex = Regex::new(
        r"^(?!\s*$).+"
    ).unwrap();

    pub static ref COMPACT_TEXT_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_\-.]*$"
    ).unwrap();

    pub static ref ASCII_SPECIAL_CHARS: Regex = Regex::new(
        r"^[\x00-\x7F]*$"
    ).unwrap();
}
