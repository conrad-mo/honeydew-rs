use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Error, Response};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeJsonResult;

use crate::api;
#[derive(Serialize)]
pub struct CVLetter {
    pub date: String,
    pub firstparagraph: String,
    pub experienceparagraphone: String,
    pub experienceparagraphtwo: String,
    pub endingparagraph: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct APIData {
    model: String,
    prompt: String,
    max_tokens: i32,
    temperature: f64,
    k: i32,
    stop_sequences: Vec<String>,
    return_likelihoods: String,
}

#[derive(Debug, Deserialize)]
struct APIResponse {
    message: String,
    error: String,
}

impl CVLetter {
    pub async fn generate_paragraph1(){
        let client = reqwest::Client::new();
        let request_data = APIData {
            model: "command".to_string(),
            prompt: "Write a body paragraph about why someone should apply to RBC".to_string(),
            max_tokens: 300,
            temperature: 0.9,
            k: 0,
            stop_sequences: Vec::new(),
            return_likelihoods: "NONE".to_string(),
        };
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("BEARER {}", api::APIKEY)).unwrap());
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let response = client.post("https://api.cohere.ai/v1/generate").headers(headers).json(&request_data).send().await;
        let pog = response.unwrap();
        println!("{:?}", pog);
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