use crate::dialog::Dialog;
use crate::ollama::Ollama;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, prelude::*, symbols::border, text::Line,
    widgets::*,
};
use std::io;

#[derive(Debug)]
pub struct App {
    pub ollama: Ollama,
    pub current_dialog: Option<Dialog>,
    pub master_prompt: String,
    pub exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            ollama: Ollama::default(),
            current_dialog: None,
            master_prompt: String::new(),
            exit: false,
        }
    }
}

impl App {
    pub fn new() -> App {
        Self::default()
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Enter => self.get_dialog().await,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub async fn get_dialog(&mut self) {
        let ollama_response = self.ollama.generate(None, Some(&self.master_prompt)).await;
        match ollama_response {
            Ok(response) => {
                let dialog: Result<Dialog, anyhow::Error> = response.try_into();
                match dialog {
                    Ok(dialog) => self.current_dialog = Some(dialog),
                    Err(err) => eprintln!("couldn't parse into dialog: {}", err),
                }
            }
            Err(err) => eprintln!("couldn't get response: {}", err),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Arti".bold());
        let instructions = Line::from(vec![
            "Quit ".into(),
            "<Q> ".blue().bold(),
            " Generate dialog ".into(),
            "<Enter>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions)
            .border_set(border::PLAIN);

        if let Some(dialog) = &self.current_dialog {
            let dialog_line = Line::styled(dialog.line.clone(), Style::default().bold());
            let mut lines: Vec<Line> = vec![dialog_line];
            if let Some(options) = &dialog.options {
                for option in options {
                    let option = format!("> {} ({})", option.line, option.tone);
                    lines.push(Line::from(option));
                }
            }

            let text = Text::from(lines);

            Paragraph::new(text).block(block).render(area, buf);
        } else {
            Paragraph::new("Please generate a dialog.")
                .block(block)
                .render(area, buf);
        }
    }
}
