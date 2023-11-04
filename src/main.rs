extern crate dotenv;

use salvo::prelude::*;
use serde::Serialize;

// SQLX TYPES
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;

// ENV
use dotenv::dotenv;

mod db;

mod functions;
use functions::users::get_users;


#[derive(Serialize, Debug)]
pub struct User {
    name: String,
    id: Uuid,
    created_at: DateTime<Utc>,
    age: i32,
}

#[handler]
async fn hello(res: &mut Response) {
    let users = get_users().await.unwrap();

    res.render(Json(users));
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
