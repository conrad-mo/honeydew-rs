use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Error};
use serde::{Serialize};

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

impl CVLetter {
    pub async fn generate_paragraph1(&mut self) -> Result<(), Error>{
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
        let response = client.post("https://api.cohere.ai/v1/generate").headers(headers).json(&request_data).send().await?;
        let response_body = response.text().await?;
        let index1 = response_body.find(",\"text\":\" ");
        let indexone =  index1.unwrap() + 10;
        let index2 = response_body.find("\"}],\"prompt\":");
        let indextwo = index2.unwrap();
        self.firstparagraph = String::from(&response_body[indexone..indextwo]);
        Ok(())
    }
    pub async fn generate_experienceparagraph1(&mut self) -> String {
        self.experienceparagraphone = String::from("Yes");
        String::from("Yes")
    }
    pub async fn generate_experienceparagraph2(&mut self) -> String {
        self.experienceparagraphtwo = String::from("Yes");
        String::from("Yes")
    }
    pub async fn generate_endingparagraph(&mut self) -> String {
        self.endingparagraph = String::from("Yes");
        String::from("Yes")
    }
}