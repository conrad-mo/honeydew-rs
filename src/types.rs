use reqwest::header::HeaderMap;
use serde::Serialize;
use serde::Deserialize;

use crate::api;
#[derive(Debug, Deserialize)]
pub struct APIResponse {
    message: String,
    error: String,
}
#[derive(Serialize)]
pub struct CVLetter {
    pub date: String,
    pub firstparagraph: String,
    pub experienceparagraphone: String,
    pub experienceparagraphtwo: String,
    pub endingparagraph: String,
    pub name: String,
}

impl CVLetter {
    pub async fn generate_paragraph1() -> Result<APIResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert("Authorization", format!("Bearer {}", api::APIKEY).parse().unwrap());
        let resp = client.post("https://api.cohere.ai/v1/generate").headers(headers).send().await;
        let resp_json = resp?.json::<APIResponse>().await;
        println!("{:?}", resp_json);
        resp_json
    }
    async fn generate_experienceparagraph1() -> String {
        String::from("Yes")
    }
    async fn generate_experienceparagraph2() -> String {
        String::from("Yes")
    }
    async fn generate_endingparagraph() -> String {
        String::from("Yes")
    }
}