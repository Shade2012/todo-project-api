use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::hash;
use serde_json::json;
use crate::domain::models::user::{user_to_response, User};
use crate::{api::router::AppState, schema::CreateUserSchema};

pub async fn create_user_command (
    State(data):State<Arc<AppState>>,
    Json(body):Json<CreateUserSchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>
{
    let hashed_password = hash(&body.password, 8)
        .map_err(|e| format!("Failed to hash password: {}", e));
    let query_result:Result<sqlx::mysql::MySqlQueryResult,String> = 
    sqlx::query(r#"INSERT INTO users (name, password) VALUES (?,?)"#)
    .bind(&body.name)
    //.bind(hash(&body.password, DEFAULT_COST).expect("Error Format"))
    .bind(&hashed_password.expect("error hashMap"))
    .execute(&data.db)
    .await
    .map_err(|e|e.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Todo already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let last_insert_id = query_result.unwrap().last_insert_id();

    let user = sqlx::query_as!(User,"SELECT * FROM users WHERE id = ?", last_insert_id )
    .fetch_one(&data.db)
    .await
    .map_err(|error|{
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "Status" : "Error",
                "message": format!("{:?}", error),
            }))
        )
    })?;

    let user_response = serde_json::json!({
        "Status": "success",
        "data":serde_json::json!({
            "user":user_to_response(&user)
        })
    });
    Ok(Json(user_response))
}