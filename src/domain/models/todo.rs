use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// for sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Todo{
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub title: String,
    pub content: String,
    pub complete: Option<i8>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

//for json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]

pub struct TodoResponse {
    #[serde(rename = "id")]
    pub id: u32,
    pub user_id: u32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "complete")]
    pub complete: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub fn todo_to_response(todo:&Todo) -> TodoResponse {
    TodoResponse {
        id: todo.id.as_ref().unwrap().to_owned(),
        user_id: todo.user_id.as_ref().unwrap().to_owned(),
        title: todo.title.to_owned(),
        content: todo.content.to_owned(),
        complete: match todo.complete {
            Some(1) => true,  // If the value is 1, return true
            Some(0) => false, // If the value is 0, return false
            _ => false,       // If no value, default to false
        },
        created_at: todo.created_at.unwrap().to_owned(),
        updated_at: todo.updated_at.unwrap().to_owned(),
    }
}
