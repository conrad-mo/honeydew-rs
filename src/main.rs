mod types;
mod api;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::types::{CVLetter, UserInfo};
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
async fn coverlettergen(Json(request_data): Json<UserInfo>) -> impl IntoResponse {
    let (paragraph1_result, experience1_result, experience2_result, ending_result) = tokio::join!(
            CVLetter::generate_paragraph1(&request_data),
            CVLetter::generate_experienceparagraph1(&request_data),
            CVLetter::generate_experienceparagraph2(&request_data),
            CVLetter::generate_endingparagraph(&request_data)
        );
    let letter = CVLetter {
        date: String::from(""),
        firstparagraph: paragraph1_result.unwrap(),
        experienceparagraphone: experience1_result.unwrap(),
        experienceparagraphtwo: experience2_result.unwrap(),
        endingparagraph: ending_result.unwrap(),
    };
    (StatusCode::OK, Json(letter))
}
