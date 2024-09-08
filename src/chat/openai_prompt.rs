use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env , fmt};

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    message: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
    index: usize,
    finish_reason: String,
}

pub struct OpenAI {
    client: Client,
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

enum CustomError {
    Request(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Request(e) => write!(f, "Request error: {}", e),
            CustomError::Serde(e) => write!(f, "Serialization error: {}", e),
            CustomError::Io(e) => write!(f, "I/O error: {}", e),
            CustomError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

// Implement conversion from `reqwest::Error` to `CustomError`
impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Request(err)
    }
}

// Implement conversion from `serde_json::Error` to `CustomError`
impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::Serde(err)
    }
}

// Implement conversion from `std::io::Error` to `CustomError`
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::Io(err)
    }
}


impl OpenAI {
    pub fn new() -> Self {
        dotenv().ok();
        let api_key = env::var("OPEN_AI").expect("OpenAI key must be set!");
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn generate_text(&self, prompt: &str) -> Result<String, CustomError> {
        let request_body = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            message: vec![Message {
                role: "system".to_string(),
                content: "you are a helpful assistant.".to_string(),
            },
            Message{
                role: "user".to_string(),
                content: prompt.to_string()
            }],
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

    
        if !status.is_success() {
            return Err(CustomError::Other(format!(
                "Request failed with status {}: {}",
                status, text
            )));
        }
        let response_json: OpenAIResponse = serde_json::from_str(&text)?;

        let default_reply = "no reply".to_string();
        let reply = response_json
        .choices
        .first()
        .map(|choice| &choice.message.content)
        .unwrap_or(&default_reply);

        Ok(reply.clone())
    }
}
