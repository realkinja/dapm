use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct Ollama {
    // The URL that API requests will get sent to.
    pub api_path: String,
    // The preferred model.
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub model: String,
    pub response: String,
    pub thinking: Option<String>,
    pub done: bool,
    pub done_reason: String,
    pub total_duration: f64,
    pub load_duration: f64,
    pub prompt_eval_count: f64,
    pub prompt_eval_duration: f64,
    pub eval_count: f64,
    pub eval_duration: f64,
}

impl Ollama {
    #[allow(dead_code)]
    pub async fn version(&self, client: &reqwest::Client) -> Result<String, anyhow::Error> {
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

    #[allow(dead_code)]
    pub async fn pull_model(&self, client: &reqwest::Client) -> Result<(), anyhow::Error> {
        let json = json!({
            "model": self.model,
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

    #[allow(dead_code)]
    pub async fn generate(
        &self,
        prompt: Option<&str>,
        system: Option<&str>,
        client: &reqwest::Client,
    ) -> Result<Response, anyhow::Error> {
        let json = json!({
            "model": self.model,
            "prompt": prompt.unwrap_or(" "),
            "system": system.unwrap_or(" "),
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
                    Ok(response) => {
                        return Ok(response);
                    }
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
            model: String::from("gpt-oss:20b"),
        }
    }
}
