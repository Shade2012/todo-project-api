use std::{collections::HashMap, f32::consts::E, fmt::format, ops::DerefMut, sync::Arc};

use crate::{
    api::router::AppState,
    domain::models::todo::{todo_to_response, Todo},
    schema::{CreateTodoSchema, UpdateTodoSchema},
};
use axum::{
    body,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde_json::json;
use sqlx::MySql;
use tokio::task::Id;
use uuid::Uuid;

pub async fn create_todo_command(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Use the transaction as the executor
    let query_result: Result<sqlx::mysql::MySqlQueryResult, String> =
        sqlx::query(r#"INSERT INTO todos (title, content) VALUES (?, ?)"#)
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
