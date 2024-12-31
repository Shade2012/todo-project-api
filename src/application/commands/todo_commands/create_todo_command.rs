use std::sync::Arc;

use crate::{
    api::router::AppState,
    domain::models::todo::{todo_to_response, Todo},
    schema::CreateTodoSchema,
};
use axum::{
    extract::State, http::{header::AUTHORIZATION, StatusCode}, response::IntoResponse, Extension, Json
};
use serde_json::json;
pub async fn create_todo_command(
    State(data): State<Arc<AppState>>,
    Extension(user_id): Extension<u32>,
    Json(body): Json<CreateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Use the transaction as the executor
    let query_result: Result<sqlx::mysql::MySqlQueryResult, String> =
        sqlx::query(r#"INSERT INTO todos (user_id, title, content) VALUES (?, ?, ?)"#)
            .bind(&user_id)
            .bind(&body.title)
            .bind(&body.content)
            .execute(&data.db) // Pass `&mut tx` here
            .await
            .map_err(|err| err.to_string());

    //Duplicate Err Check
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

    //Get Created todo by id
    let todo = sqlx::query_as!(Todo, r#"SELECT * FROM todos WHERE id = ?"#, last_insert_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let todo_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "todo": todo_to_response(&todo)
        })
    });
    Ok(Json(todo_response))
}

