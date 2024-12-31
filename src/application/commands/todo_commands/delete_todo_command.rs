use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::api::router::AppState;


pub async fn delete_todo_command (
    Path(id): Path<u64>,
    State(data): State<Arc<AppState>>,
)
-> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>
{
    let query_result = sqlx::query_as!(Todo,r#"DELETE FROM todos WHERE id = ?"#,&id)
    .execute(&data.db)
    .await;
    match query_result{
        Ok(todo) => {
            if todo.rows_affected() > 0 {
                let response = serde_json::json!({
                    "status": "success",
                    "message": format!("Todo with ID: {} successfully deleted", id)
                });
                Ok(Json(response))
            } else {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Todo with ID: {} not found", id)
                });
                Err((StatusCode::NOT_FOUND, Json(error_response)))
            }
        },
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
    }
}