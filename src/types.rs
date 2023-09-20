use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Error};
use serde::{Serialize, Deserialize};

use crate::api;
#[derive(Serialize)]
pub struct CVLetter {
    pub date: String,
    pub firstparagraph: String,
    pub experienceparagraphone: String,
    pub experienceparagraphtwo: String,
    pub endingparagraph: String,
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub background: String,
    pub jobposition: String,
    pub projects: Vec<String>,
    pub company: String,
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
    pub async fn generate_paragraph1(userinfo: &UserInfo) -> Result<String, Error>{
        let client = reqwest::Client::new();
        let prompt = format!(
            "Write a first introductory paragraph out of a 4 paragraph cover letter for someone applying to a {} position at {}. Some background information for this person is {}. \
            Write it in first person formally and have some theme with the company's goals",
            userinfo.jobposition,
            userinfo.company,
            userinfo.background
        );
        let request_data = APIData {
            model: "command".to_string(),
            prompt: prompt.to_string(),
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
        Ok(String::from(&response_body[indexone..indextwo]))
    }
    pub async fn generate_experienceparagraph1(userinfo: &UserInfo) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let prompt = format!(
            "Write a project paragraph out of a 4 paragraph cover letter for someone applying to a {} position at {}. The details for this project are here: {} \
            Write it in first person formally and have some theme with the job position and demonstrate the skills used in this project",
            userinfo.jobposition,
            userinfo.company,
            userinfo.projects[0]
        );
        let request_data = APIData {
            model: "command".to_string(),
            prompt: prompt.to_string(),
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
        Ok(String::from(&response_body[indexone..indextwo]))
    }
    pub async fn generate_experienceparagraph2(userinfo: &UserInfo) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let prompt = format!(
            "Write a project paragraph out of a 4 paragraph cover letter for someone applying to a {} position at {}. The details for this project are here: {} \
            Write it in first person formally and have some theme with the job position and demonstrate the skills used in this project",
            userinfo.jobposition,
            userinfo.company,
            userinfo.projects[1]
        );
        let request_data = APIData {
            model: "command".to_string(),
            prompt: prompt.to_string(),
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
        Ok(String::from(&response_body[indexone..indextwo]))
    }
    pub async fn generate_endingparagraph(userinfo: &UserInfo) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let prompt = format!(
            "Write the last paragraph out of a 4 paragraph cover letter for someone applying to a {} position at {}. \
            Write it in first person formally and have some theme with the job position and company.",
            userinfo.jobposition,
            userinfo.company,
        );
        let request_data = APIData {
            model: "command".to_string(),
            prompt: prompt.to_string(),
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
        Ok(String::from(&response_body[indexone..indextwo]))
    }
}