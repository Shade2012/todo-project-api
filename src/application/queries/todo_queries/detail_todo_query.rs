use crate::api::router::AppState;
use crate::domain::models::todo::{self, todo_to_response, Todo, TodoResponse};
use crate::schema::{FilterOptions, TodoIdQuery};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn todo_detail_query(
    opts: Option<Query<TodoIdQuery>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let Query(opts) = opts.unwrap_or_default();
    let id = opts.id.unwrap_or(0);
    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT * FROM todos WHERE id = ?"#,
        id
    )
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