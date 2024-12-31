use serde::{Deserialize, Serialize};

// for sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct User{
    pub id: Option<u32>,
    pub name: String,
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

//for json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]

pub struct UserResponse {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn user_to_response(user:&User) -> UserResponse {
    UserResponse {
        id: user.id.as_ref().unwrap().to_owned(),
        name: user.name.to_owned(),
        password: user.password.to_owned(),
        created_at: user.created_at.unwrap().to_owned(),
        updated_at: user.updated_at.unwrap().to_owned(),
    }
}
