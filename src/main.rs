use std::{env, f32::consts::E, sync::Arc};
use dotenv::dotenv;
use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use sqlx::{mysql::MySqlPoolOptions, pool, MySqlPool};
use todo_project_api::api::router::{create_router,AppState};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("DB_USERNAME: {:?}", env::var("DB_USERNAME"));
    println!("DATABASE_URL: {:?}", env::var("DATABASE_URL"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
        Ok(pool)=>{
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err)=>{
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
        
    };

    let shared_state = Arc::new(AppState { db: pool.clone() });
     let app = create_router(shared_state);
    println!("✅ Server started successfully at 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();

}
