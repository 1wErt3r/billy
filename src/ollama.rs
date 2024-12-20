use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool
}

pub enum QueryMode {
    Command,
    Summarize
}

pub async fn query_ollama(
    prompt: &str, 
    server_url: &str, 
    model: &str,
    mode: QueryMode
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let prompt_template = match mode {
        QueryMode::Command => format!(
            "You are a Bitcoin node assistant. Convert the following question into appropriate Bitcoin RPC commands. \
            Only return the RPC command name without explanation. Question: {}", 
            prompt
        ),
        QueryMode::Summarize => format!(
            "You are a Bitcoin node assistant. Summarize the following JSON response from a Bitcoin node in a clear, \
            concise way that a user can understand. Response: {}", 
            prompt
        ),
    };

    let request = OllamaRequest {
        model: model.to_string(),
        prompt: prompt_template,
        stream: false
    };

    let response = client
        .post(format!("{}/api/generate", server_url))
        .json(&request)
        .send()
        .await?;

    let text = response.text().await?;
    let ollama_response: OllamaResponse = serde_json::from_str(&text)?;
    
    Ok(ollama_response.response.trim().to_string())
} 