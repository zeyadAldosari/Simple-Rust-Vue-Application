use axum::{
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use tracing::info;
use validator::Validate;

use crate::db::establish_connection;
use crate::models::{LoginRequest, NewUser, RegisterRequest, User};
use crate::schema::users;
use crate::utils::{hash_password, verify_password};

pub async fn register_user(
    Json(payload): Json<RegisterRequest>,
) -> Result<&'static str, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        info!("Registration validation failed: {:?}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Invalid input: {}", e)));
    }

    let hashed_password = match hash_password(payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Failed to hash password: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process password".to_string(),
            ));
        }
    };

    let mut conn = establish_connection();

    let username_exists = users::table
        .filter(users::username.eq(&payload.username))
        .select(users::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database query error".into())
        })?
        .is_some();

    if username_exists {
        return Err((StatusCode::CONFLICT, "Username already exists".into()));
    }    

    let new_user = NewUser {
        username: &payload.username,
        password: &hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| {
            eprintln!("Failed to insert user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create account".to_string(),
            )
        })?;

    info!("User '{}' registered successfully", payload.username);
    Ok("Account created successfully")
}

pub async fn login_user(
    Json(payload): Json<LoginRequest>,
) -> Result<&'static str, (StatusCode, String)> {
    let mut conn = establish_connection();

    let user = users::table
        .filter(users::username.eq(&payload.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| {
            eprintln!("Failed to query user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database query error".to_string(),
            )
        })?;

    match user {
        Some(user) => {
            match verify_password(payload.password, &user.password) {
                Ok(valid) => {
                    if valid {
                        info!("User '{}' logged in successfully", payload.username);
                        Ok("Logged in successfully")
                    } else {
                        info!("Login failed for '{}': Invalid password", payload.username);
                        Err((
                            StatusCode::UNAUTHORIZED,
                            "Invalid username or password".to_string(),
                        ))
                    }
                }
                Err(e) => {
                    eprintln!("Password verification error: {:?}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Password verification failed".to_string(),
                    ))
                }
            }
        }
        None => {
            info!("Login failed: Username '{}' not found", payload.username);
            Err((StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))
        }
    }
}
