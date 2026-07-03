use crate::ollama::Ollama;
use crate::ui::ui;
use crate::{dialog::Dialog, ollama::Model};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;
use std::io;

#[derive(Debug)]
pub enum AppState {
    Idle,
    Pulling,
    Generating,
    Error(String),
}

#[derive(Debug)]
pub struct App {
    pub ollama: Ollama,
    pub http_client: reqwest::Client,
    pub current_dialog: Option<Dialog>,
    pub master_prompt: String,
    pub exit: bool,
    pub app_state: AppState,
    pub option_state: ListState,
    pub models_available: Vec<Model>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ollama: Ollama::default(),
            current_dialog: None,
            master_prompt: String::new(),
            http_client: reqwest::Client::new(),
            exit: false,
            app_state: AppState::Idle,
            option_state: ListState::default().with_selected(Some(0)),
            models_available: Vec::new(),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let models = self.ollama.models(&self.http_client).await;
        match models {
            Ok(list) => self.models_available = list,
            Err(err) => self.app_state = AppState::Error(err.to_string()),
        }

        let model_names: Vec<String> = self
            .models_available
            .iter()
            .map(|x| x.model.clone())
            .collect();
        if !model_names.contains(&self.ollama.model) {
            match self.ollama.pull_model(&self.http_client).await {
                Ok(()) => {
                    drop(model_names);
                    let models = self.ollama.models(&self.http_client).await;
                    match models {
                        Ok(list) => self.models_available = list,
                        Err(err) => self.app_state = AppState::Error(err.to_string()),
                    }
                }
                Err(err) => {
                    self.app_state = AppState::Error(format!("failed to fetch model {}", err))
                }
            }
        }

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
            KeyCode::Char('g') => {
                self.app_state = AppState::Generating;
                self.get_dialog().await
            }
            KeyCode::Char('w') | KeyCode::Up => self.option_state.select_previous(),
            KeyCode::Char('s') | KeyCode::Down => self.option_state.select_next(),
            KeyCode::Enter => self.send_option().await,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub async fn get_dialog(&mut self) {
        self.app_state = AppState::Generating;
        self.current_dialog = None;
        let ollama_response = self
            .ollama
            .generate(None, Some(&self.master_prompt), &self.http_client)
            .await;
        match ollama_response {
            Ok(response) => {
                let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                match dialog {
                    Ok(dialog) => self.current_dialog = Some(dialog),
                    Err(err) => self.app_state = AppState::Error(err.to_string()),
                }
            }
            Err(err) => self.app_state = AppState::Error(err.to_string()),
        }
    }

    async fn send_option(&mut self) {
        if let Some(dialog) = &mut self.current_dialog {
            if let Some(options) = &mut dialog.options {
                if let Some(i) = &mut self.option_state.selected() {
                    let ollama_response = self
                        .ollama
                        .generate(
                            Some(&format!("{:#?}", options[*i])),
                            Some(&self.master_prompt),
                            &self.http_client,
                        )
                        .await;
                    match ollama_response {
                        Ok(response) => {
                            let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                            match dialog {
                                Ok(dialog) => self.current_dialog = Some(dialog),
                                Err(err) => self.app_state = AppState::Error(err.to_string()),
                            }
                        }
                        Err(err) => self.app_state = AppState::Error(err.to_string()),
                    }
                }
            }
        }
    }
}
