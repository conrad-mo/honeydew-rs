mod types;
mod api;

use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router,};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::types::{CVLetter, UserInfo};
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/generate", post(coverlettergen))
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
    let field1 = &request_data.name;
    let field3 = &request_data.projects;
    println!("{}", field1);
    println!("Projects: \n{:?}", field3);
    let mut letter = CVLetter {
        date: String::from(""),
        firstparagraph: String::from(""),
        experienceparagraphone: String::from(""),
        experienceparagraphtwo: String::from(""),
        endingparagraph: String::from(""),
    };
    //println!("{}", payload.name);
    let _ = &letter.generate_paragraph1(&request_data).await;
    let _ = &letter.generate_experienceparagraph1(&request_data).await;
    let _ = &letter.generate_experienceparagraph2(&request_data).await;
    // let _ = &letter.generate_endingparagraph().await;
    (StatusCode::OK, Json(letter))
}
