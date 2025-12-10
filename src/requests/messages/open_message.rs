// Custom validation error messages
pub const INVALID_EMAIL_MESSAGE: &str = "Invalid email format";
pub const INVALID_PHONE_MESSAGE: &str = "Phone number must be 10 to 15 digits";
pub const INVALID_USERNAME_MESSAGE: &str = "Username can have letters, numbers, ., _ and - only";
pub const INVALID_DIGITS_ONLY_MESSAGE: &str = "Only numeric digits are allowed";
pub const INVALID_ALPHA_ONLY_MESSAGE: &str = "Only alphabetic characters are allowed";
pub const INVALID_ALPHANUMERIC_MESSAGE: &str = "Only alpha-numeric characters are allowed";
pub const BACKTICK_NOT_ALLOWED_MESSAGE: &str = "backtick not allowed";
pub const INVALID_TEXTUAL_CHARS_MESSAGE: &str = "Message can only contain letters, numbers, spaces, newlines, and - _ = ,";
pub const EXCEPT_ASCII_CHARS_NOT_ALLOWED_MESSAGE: &str = "It contains invalid characters. Only letters, numbers, spaces, and the following characters are allowed: . _ - ! @ # $ % ^ & * ( )";
pub const MAXIMUM_ALLOWED_CHARS_MESSAGE: &str = "Maximum allowed characters";
pub const TEXTUAL_NAME_REGEX_MESSAGE: &str = "Only alphanumeric characters and spaces are permitted";
pub const TEXTTUAL_ADDRESS_REGEX_MESSAGE: &str = "Only alphanumeric characters, spaces, and symbols (_ - = , ;) are permitted";
pub const TEXTTUAL_USERNAME_REGEX_MESSAGE: &str = "Only alphanumeric characters, underscore, hyphen, and period";
pub const TEXTUAL_OTP_REGEX_MESSAGE: &str = "OTP must be 6 digits only";
pub const TEXTUAL_MONGO_OBJECT_REGEX_MESSAGE: &str = "Only 24 hexadecimal characters";