use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Ollama {
    // The URL that API requests will get sent to.
    api_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    model: String,
    pub response: String,
    thinking: String,
    done: bool,
    done_reason: String,
    total_duration: f64,
    load_duration: f64,
    prompt_eval_count: f64,
    prompt_eval_duration: f64,
    eval_count: f64,
    eval_duration: f64,
}

impl Ollama {
    pub async fn version(&self) -> Result<String, anyhow::Error> {
        let client = reqwest::Client::new();
        let request = client
            .get(format!("{}/api/version", self.api_path))
            .send()
            .await;

        match request {
            Ok(response) => {
                let json: Result<serde_json::Value, reqwest::Error> = response.json().await;
                match json {
                    Ok(json) => return Ok(json["version"].as_str().unwrap().to_string()),
                    Err(err) => {
                        bail!("{}", err)
                    }
                }
            }
            Err(err) => bail!("{}", err),
        }
    }

    pub async fn pull_model(&self, model: &str) -> Result<(), anyhow::Error> {
        let client = reqwest::Client::new();
        let json = json!({
            "model": model,
            "stream": false
        });

        let request = client
            .post(format!("{}/api/pull", self.api_path))
            .json(&json)
            .send()
            .await;

        match request {
            Ok(response) => {
                let json: Result<serde_json::Value, reqwest::Error> = response.json().await;
                match json {
                    Ok(json) => {
                        if json["status"] == "success" {
                            return Ok(());
                        } else if json["error"].is_string() {
                            bail!("{}", json["error"]);
                        } else {
                            bail!("Something went wrong after decoding the response body!");
                        }
                    }
                    Err(err) => bail!("Could not receive the JSON! {}", err),
                }
            }
            Err(err) => bail!("Failed to send response! {}", err),
        }
    }

    pub async fn generate(
        &self,
        model: &str,
        prompt: Option<&str>,
        system_prompt: Option<&str>,
    ) -> Result<Response, anyhow::Error> {
        let client = reqwest::Client::new();
        let json = json!({
            "model": model,
            "prompt": prompt.unwrap_or(""),
            "system": prompt.unwrap_or(""),
            "stream": false
        });

        let request = client
            .post(format!("{}/api/generate", self.api_path))
            .json(&json)
            .send()
            .await;

        match request {
            Ok(response) => {
                let json: Result<Response, reqwest::Error> = response.json().await;
                match json {
                    Ok(response) => return Ok(response),
                    Err(err) => bail!("{}", err),
                }
            }
            Err(err) => bail!("{}", err),
        }
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Ollama {
            api_path: String::from("http://localhost:11434"),
        }
    }
}
