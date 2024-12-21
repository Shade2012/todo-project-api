use std::sync::Arc;

use crate::{
    api::router::AppState,
    domain::models::todo::{todo_to_response, Todo},
    schema::UpdateTodoSchema,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn update_todo_command(
    Path(id): Path<u64>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(Todo, r#"SELECT * FROM todos WHERE id = ?"#, &id)
        .fetch_one(&data.db)
        .await;
        match query_result {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status":"error not found",
                "message":format!("Todo with ID : {} Not Found",&id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status":"error",
                    "message":format!("{:?}",e)
                })),
            ));
        }
    };

    let mut update = String::new();
    if let Some(title) = &body.title {
        update.push_str(&format!("title = '{}', ", title));
    }
    if let Some(content) = &body.content {
        update.push_str(&format!("content = '{}', ", content));
    }
    if let Some(complete) = body.complete {
        update.push_str(&format!("complete = {}", complete));
    }
    if update.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": "Update tidak boleh kosong"})),
        ));
    }
    let query = format!("UPDATE todos SET {} WHERE id = {}", update, &id);
    sqlx::query(&query).execute(&data.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error query update",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let updated_todo = sqlx::query_as!(Todo, r#"SELECT * FROM todos WHERE id = ?"#, &id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error get updated","message": format!("{:?}", e)})),
            )
        })?;
    let todo_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "todo": todo_to_response(&updated_todo)
        })
    });
    Ok(Json(todo_response))
}
