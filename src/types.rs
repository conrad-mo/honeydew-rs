use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Error, Response};
use serde::{Serialize, Deserialize};

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
    pub async fn generate_paragraph1() -> Result<(), Error>{
        let url = "https://api.cohere.ai/v1/generate";
        let client = reqwest::Client::new();
        let request_data = APIData {
            model: "command".to_string(),
            prompt: "Write a body paragraph about \"Shopify is a great case study\" in a blog post titled \"Tips from the most successful companies\"".to_string(),
            max_tokens: 300,
            temperature: 0.9,
            k: 0,
            stop_sequences: Vec::new(),
            return_likelihoods: "NONE".to_string(),
        };
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("BEARER {}", api::APIKEY)).unwrap());
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let response = client.post("https://api.cohere.ai/v1/generate").headers(headers).json(&request_data).send().await?;
        println!("Status: {}", response.status());
        let response_body = response.text().await?;
        println!("Response body:\n{}", response_body);
        Ok(())
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