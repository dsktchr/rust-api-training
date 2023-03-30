use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router, http::HeaderValue, Json,
};
use sea_orm::{DatabaseConnection, Database};
use serde::{Serialize, Deserialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

async fn create_user() -> Json<User> {
    let user = User {
        name: String::from("範馬勇次郎"),
        age: 40,
    };

    return Json(user);
}


#[tokio::main]
async fn main() {

    // Database接続
    let db: DatabaseConnection = Database::connect("sqlite://admin:pass@localhost/database").await.expect("error");
    
    // とりあえずトレースを初期化するらしい
    let app: Router = Router::new()
    .route("/hello-world", get(create_user))
    .layer(CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}