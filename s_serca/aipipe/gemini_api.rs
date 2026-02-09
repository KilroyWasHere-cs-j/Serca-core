use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use reqwest::Client;

#[derive(Debug, Serialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: ContentResponse,
}

#[derive(Debug, Deserialize)]
pub struct ContentResponse {
    pub parts: Vec<PartResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PartResponse {
    pub text: Option<String>,
}

pub async fn gemini_generate_content(
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let body = GenerateContentRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let client = Client::new();

    let resp: GenerateContentResponse = client
        .post(&url)
        .header("x-goog-api-key", api_key)
        .json(&body)
        .send()
        .await
        .context("Failed to send request to Gemini")?
        .error_for_status()
        .context("Gemini returned an error status")?
        .json()
        .await
        .context("Failed to deserialize Gemini response")?;

    // Pull the first text response safely
    let text = resp
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .and_then(|p| p.text.as_ref())
        .cloned()
        .unwrap_or_default();

    Ok(text)
}
