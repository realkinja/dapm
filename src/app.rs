use crate::dialog::Dialog;
use crate::dialog::DialogOptionList;
use crate::ollama::Ollama;
use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

#[derive(Debug)]
pub struct App {
    pub ollama: Ollama,
    pub http_client: reqwest::Client,
    pub current_dialog: Option<Dialog>,
    pub current_options: Option<DialogOptionList>,
    pub master_prompt: String,
    pub exit: bool,
    pub error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ollama: Ollama::default(),
            current_dialog: None,
            current_options: None,
            master_prompt: String::new(),
            http_client: reqwest::Client::new(),
            exit: false,
            error: None,
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    async fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await
            }
            _ => {}
        };
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('g') => self.get_dialog().await,
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Enter => self.send_option().await,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub async fn get_dialog(&mut self) {
        let ollama_response = self
            .ollama
            .generate(None, Some(&self.master_prompt), &self.http_client)
            .await;
        match ollama_response {
            Ok(response) => {
                eprintln!("{:#?}", response.response);
                let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                match dialog {
                    Ok(dialog) => self.current_dialog = Some(dialog),
                    Err(err) => self.error = Some(err.to_string()),
                }
            }
            Err(err) => self.error = Some(err.to_string()),
        }
    }

    fn select_next(&mut self) {
        if let Some(options) = &mut self.current_options {
            options.state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if let Some(options) = &mut self.current_options {
            options.state.select_previous();
        }
    }

    async fn send_option(&mut self) {
        if let Some(options) = &mut self.current_options {
            if let Some(i) = options.state.selected() {
                let ollama_response = self
                    .ollama
                    .generate(
                        Some(&format!("{:#?}", options.items[i])),
                        Some(&self.master_prompt),
                        &self.http_client,
                    )
                    .await;
                match ollama_response {
                    Ok(response) => {
                        eprintln!("{:#?}", response.response);
                        let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                        match dialog {
                            Ok(dialog) => self.current_dialog = Some(dialog),
                            Err(err) => self.error = Some(err.to_string()),
                        }
                    }
                    Err(err) => self.error = Some(err.to_string()),
                }
            }
        }
    }
}
