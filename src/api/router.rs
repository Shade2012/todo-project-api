use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;
use crate::application::{commands::create_todo_command::create_todo_command, queries::todo_all_query::{todo_detail_query, todo_list_all_query}};

pub struct AppState {
   pub db: MySqlPool,
}

use super::health_checker_handler;
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/healthchecker",get(health_checker_handler))
    .route("/api/todos/created", post(create_todo_command))
    .route("/api/todos/all", get(todo_list_all_query))
    .route("/api/todos/detail", get(todo_detail_query))
    .with_state(state)
}