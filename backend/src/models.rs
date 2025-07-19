use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}


fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("too_short"));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("missing_uppercase"));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("missing_lowercase"));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("missing_digit"));
    }
    
    let special_chars = "!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?";
    if !password.chars().any(|c| special_chars.contains(c)) {
        return Err(ValidationError::new("missing_special"));
    }

    Ok(())
}


#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub username: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_request_validation() {
        let req = RegisterRequest {
            username: "test@example.com".to_string(),
            password: "StrongPassword123!".to_string(),
        };
        assert!(req.validate().is_ok());

        let req = RegisterRequest {
            username: "test@example.com".to_string(),
            password: "short".to_string(),
        };
        let err = req.validate().unwrap_err();
        assert!(err.field_errors().contains_key("password"));
        assert_eq!(err.field_errors()["password"][0].code, "too_short");

        let req = RegisterRequest {
            username: "test@example.com".to_string(),
            password: "strongpassword123!".to_string(),
        };
        let err = req.validate().unwrap_err();
        assert!(err.field_errors()["password"][0].code == "missing_uppercase");

    }
}
