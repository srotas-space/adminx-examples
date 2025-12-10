use validator::ValidationError;
use mongodb::{Database, bson::doc};
use crate::models::user::User;

/*--------------------------------------------------------------------------------------------
// User Model Validations
--------------------------------------------------------------------------------------------*/
pub async fn validate_unique_email(db: &Database, email: &str) -> Result<(), ValidationError> {
    // If email is empty or blank, skip uniqueness check (return success)
    if email.trim().is_empty() {
        return Ok(());
    }

    let collection = db.collection::<User>("users");

    match collection.find_one(doc! { "email": email }, None).await {
        Ok(Some(_)) => {
            let mut err = ValidationError::new("duplicate_email");
            err.message = Some("Email already exists".into());
            Err(err)
        }
        Ok(None) => Ok(()), // email is unique
        Err(_) => {
            let mut err = ValidationError::new("db_error");
            err.message = Some("Database error while validating email".into());
            Err(err)
        }
    }
}
/*--------------------------------------------------------------------------------------------*/
