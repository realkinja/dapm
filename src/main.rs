mod app;
mod dialog;
mod ollama;
use crate::app::App;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut app = App {
        master_prompt: include_str!("../master-prompt.md").to_string(),
        ..Default::default()
    };

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal).await;
    ratatui::restore();
    result
}
