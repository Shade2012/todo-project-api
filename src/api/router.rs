use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::application::{commands::todo_commands::{create_todo_command::create_todo_command, delete_todo_command::delete_todo_command, update_todo_command::update_todo_command}
, queries::todo_queries::{all_todo_query::todo_list_all_query, detail_todo_query::todo_detail_query}};

use super::health_checker_handler;

pub struct AppState {
    pub db: MySqlPool,
 }

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/healthchecker",get(health_checker_handler))
    .route("/api/todos/created", post(create_todo_command))
    .route("/api/todos/all", get(todo_list_all_query))
    .route("/api/todos/detail", get(todo_detail_query))
    .route("/api/todos/:id", post(update_todo_command))
    .route("/api/todos/:id", delete(delete_todo_command))
    .with_state(state)
}