use crate::api::router::AppState;
use crate::domain::models::todo::{self, todo_to_response, Todo, TodoResponse};
use crate::schema::{FilterOptions, TodoIdQuery};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn todo_list_all_query(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //Param
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    //Query with macro
    let todos = sqlx::query_as!(
        Todo,
        r#"SELECT * FROM todos ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let todo_response = todos
        .iter()
        .map(|todo| todo_to_response(&todo))
        .collect::<Vec<TodoResponse>>();
    let json_response = serde_json::json!({
        "status": "Berhasil",
        "count": todo_response.len(),
        "todo": todo_response
    });
    Ok(Json(json_response))
}
