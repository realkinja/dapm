mod dialog;
mod ollama;
use crate::dialog::Dialog;

#[tokio::main]
async fn main() {
    let ollama = ollama::Ollama::default();
    let model = "gpt-oss:20b";
    let system_prompt = include_str!("../master-prompt.md");
    match ollama.version().await {
        Ok(version) => {
            println!("[INFO] Running ollama v{}", version);
            match ollama.pull_model(model).await {
                Ok(_) => println!("[OK] Pulled {} successfully!", model),
                Err(err) => eprintln!("[ERR] Could not pull model! {}", err),
            }

            match ollama.generate(model, None, Some(system_prompt)).await {
                Ok(response) => {
                    println!("=== Statistics ===");
                    println!("Generation duration: {}", response.total_duration);
                    println!("Load duration: {}", response.load_duration);
                    println!("Input tokens: {}", response.prompt_eval_count);
                    println!("Time evaluating prompt: {}", response.prompt_eval_duration);
                    println!("Output tokens: {}", response.eval_count);
                    println!("Time evaluating prompt: {}", response.eval_duration);

                    let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                    match dialog {
                        Ok(dialog) => {
                            println!("\"{}\"\n", dialog.line);

                            if let Some(options) = dialog.options {
                                for option in options.iter() {
                                    println!("> \"{}\" ({})", option.line, option.tone);
                                }
                            }
                        }
                        Err(err) => eprintln!("[ERR] Could not parse into dialog! {}", err),
                    }
                }
                Err(err) => eprintln!("[ERR] Could not generate response! {}", err),
            }
        }
        Err(err) => eprintln!("[ERR] Could not get ollama version! {}", err),
    }
}
