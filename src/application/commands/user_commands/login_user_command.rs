use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::verify;
use serde_json::json;

use crate::{api::router::AppState, application::middleware::auth::encode_jwt, domain::models::user::{user_to_response, User}, schema::LoginSchema};


pub async fn login_user_command (
    State(data):State<Arc<AppState>>,
    Json(body):Json<LoginSchema>
)
-> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    let user = sqlx::query_as!(User,r#"SELECT * FROM users WHERE name = ?"#,&body.name)
    .fetch_one(&data.db)
    .await
    .map_err(|_|{
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "Status" : "Error",
               "message": "Invalid username or password",
            })),
        )
    })?;
     let password_matched = verify(&body.password, &user.password).map_err(|_|{
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "Status" : "Error",
                "message": "An error occurred during password verification",
            }))
        )
     })?;

     if !password_matched {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Invalid username or password",
            })),
        ));
    }
    let token = encode_jwt(user.id.unwrap());
    let user_response = json!({
        "data": {
            "user": user_to_response(&user),
        },
        "status": "success",
        "token": token.unwrap(),
    });
    Ok(Json(user_response))
}


