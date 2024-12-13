use chrono::{DateTime, Local, Utc};

pub struct Todo{
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub complete: Option<bool>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>
}