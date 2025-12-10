#[macro_export]
macro_rules! required_field {
    ($obj:expr, $field:ident) => {
        if $obj.$field.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
            return Err(actix_web::error::ErrorBadRequest(serde_json::json!({
                "status": 400,
                "code": 400,
                "message": format!("{} is required", stringify!($field)),
                "data": stringify!($field),
            })));
        }
    };
}
