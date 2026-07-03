use crate::app::{App, AppState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize, palette::tailwind::*},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Wrap},
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(frame, chunks[0], &app.app_state);
    render_footer(frame, chunks[2]);

    if let Some(dialog) = &app.current_dialog {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
            .split(chunks[1]);

        let dialog_line = Line::styled(dialog.line.clone(), Style::default().bold());
        let paragraph = Paragraph::new(dialog_line).wrap(Wrap { trim: true });

        frame.render_widget(paragraph, main_chunks[0]);

        if let Some(options) = &dialog.options {
            let mut items: Vec<ListItem> = Vec::new();
            for option in options {
                items.push(option.into())
            }

            let list = List::new(items)
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol("> ")
                .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected);

            frame.render_stateful_widget(list, main_chunks[1], &mut app.option_state);
        }
    } else {
        match &app.app_state {
            AppState::Error(err) => {
                let paragraph = Paragraph::new(err.as_str()).centered().red().bold();
                frame.render_widget(paragraph, chunks[1]);
            }
            AppState::Pulling => {
                let paragraph = Paragraph::new("Pulling model...")
                    .centered()
                    .bold()
                    .italic();
                frame.render_widget(paragraph, chunks[1]);
            }
            AppState::Generating => {
                let paragraph = Paragraph::new("Generating...").centered().bold().italic();
                frame.render_widget(paragraph, chunks[1]);
            }
            AppState::Idle => {
                // let paragraph = Paragraph::new("Please generate a dialog, by pressing G.")
                //     .centered()
                //     .bold();
                // frame.render_widget(paragraph, chunks[1]);
                let idle_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
                    .split(chunks[1]);

                let header = Paragraph::new("Models available:").bold();

                let models = &app.models_available;
                let mut items = vec![];
                for model in models {
                    items.push(model.model.clone());
                }
                let list = List::new(items);

                frame.render_widget(header, idle_chunks[0]);
                frame.render_widget(list, idle_chunks[1]);
            }
        }
    }
}

fn render_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let title = Span::styled("Date-A-Package-Manager!", Style::default().bold());
    let mut line = vec![title.clone()];

    if let AppState::Generating = state {
        line.push(Span::styled(" - generating...", Style::default().italic()));
    } else {
        line = vec![title];
    }

    let header = Paragraph::new(Line::from(line)).centered();

    frame.render_widget(header, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        "Quit ".into(),
        "<Q> ".blue().bold(),
        " Generate dialog ".into(),
        "<G>".blue().bold(),
    ]);
    let footer = Paragraph::new(line).centered();

    frame.render_widget(footer, area);
}
