mod types;
mod api;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::types::CVLetter;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/generate", get(coverlettergen))
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
async fn coverlettergen() -> impl IntoResponse {
    let mut letter = CVLetter {
        date: String::from(""),
        firstparagraph: String::from(""),
        experienceparagraphone: String::from(""),
        experienceparagraphtwo: String::from(""),
        endingparagraph: String::from(""),
        name: String::from(""),
    };
    let _ = &letter.generate_paragraph1().await;
    let _ = &letter.generate_experienceparagraph1().await;
    let _ = &letter.generate_experienceparagraph2().await;
    let _ = &letter.generate_endingparagraph().await;
    (StatusCode::OK, Json(letter))
}