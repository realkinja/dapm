mod app;
mod cli;
mod dialog;
mod ollama;
mod ui;
use clap::Parser;

use crate::{app::App, cli::Cli, ollama::Ollama};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut app = {
        if let Some(custom_model) = cli.model {
            App {
                master_prompt: include_str!("../master-prompt.md").to_string(),
                ollama: Ollama {
                    model: custom_model,
                    ..Default::default()
                },
                ..Default::default()
            }
        } else {
            App {
                master_prompt: include_str!("../master-prompt.md").to_string(),
                ..Default::default()
            }
        }
    };

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal).await;
    ratatui::restore();
    result
}
