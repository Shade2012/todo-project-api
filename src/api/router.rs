use std::sync::Arc;

use axum::{
    middleware, routing::{delete, get, post}, Router
};
use sqlx::MySqlPool;

use crate::application::{commands::{todo_commands::{create_todo_command::create_todo_command, delete_todo_command::delete_todo_command, update_todo_command::update_todo_command}, user_commands::{create_user_command::create_user_command, login_user_command::login_user_command}}, middleware::auth, queries::todo_queries::{all_todo_query::todo_list_all_query, detail_todo_query::todo_detail_query}};


pub struct AppState {
    pub db: MySqlPool,
 }

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .nest(
        "/api/todos",
        Router::new()
            .route("/created", post(create_todo_command))
            .route("/all", get(todo_list_all_query))
            .route("/detail", get(todo_detail_query))
            .route("/:id", post(update_todo_command))
            .route("/:id", delete(delete_todo_command))
            .layer(middleware::from_fn(auth::authorization_middleware)),
    )

    //User
    .route("/api/user/register", post(create_user_command))
    .route("/api/user/login", post(login_user_command))
    .with_state(state)
}