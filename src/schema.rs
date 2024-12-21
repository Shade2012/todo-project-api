use serde::{Deserialize,Serialize};

//list
#[derive(Deserialize,Debug,Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

// Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub complete: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TodoIdQuery {
    pub id: Option<u64>,
}