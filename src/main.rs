use std::sync::Arc;
use dotenv::dotenv;
use todo_project_api::{api::router::{create_router,AppState}, infrastructure::mysql_database::establish_connection, };
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    dotenv().ok();
    // Initialize the database connection
    let pool = establish_connection().await;
    let shared_state = Arc::new(AppState { db: pool });

    // Create the router and attach the state
    let app = create_router(shared_state);
    println!("✅ Server started successfully at 0.0.0.0:8080");

    //Start the server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
