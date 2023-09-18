mod types;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::types::CVLetter;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/generate", get(newcv))
        .layer(cors);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "404"
}
async fn newcv() -> impl IntoResponse {
    let letter = CVLetter{
        date: String::from("Sample"),
        firstparagraph: String::from("Sample"),
        experienceparagraphone: String::from("Sample"),
        experienceparagraphtwo: String::from("Sample"),
        endingparagraph: String::from("Sample"),
        name: String::from("Sample"),
    };
    (StatusCode::OK, Json(letter))
}