use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize, palette::tailwind::*},
    text::Line,
    widgets::{List, ListItem, Paragraph},
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_footer(frame, chunks[2]);

    if let Some(dialog) = &app.current_dialog {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Percentage(100)])
            .split(chunks[1]);

        let dialog_line = Line::styled(dialog.line.clone(), Style::default().bold());

        let mut list_items = Vec::<ListItem>::new();
        if let Some(options) = &dialog.options {
            for option in options {
                let option = format!("{} ({})", option.line, option.tone);
                let line = Line::from(option);
                list_items.push(ListItem::new(line));
            }
        }

        let paragraph = Paragraph::new(dialog_line);
        frame.render_widget(paragraph, main_chunks[0]);
        let list = List::new(list_items)
            .highlight_symbol(">")
            .highlight_style(SELECTED_STYLE);
        frame.render_widget(list, main_chunks[1]);
    } else {
        if let Some(error) = &app.error {
            let paragraph = Paragraph::new(error.as_str()).centered().red().bold();
            frame.render_widget(paragraph, chunks[1]);
        } else {
            let paragraph = Paragraph::new("Please generate a dialog.")
                .centered()
                .bold();
            frame.render_widget(paragraph, chunks[1]);
        }
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("Date-A-Package-Manager!").bold().centered();

    frame.render_widget(header, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        "Quit ".into(),
        "<Q> ".blue().bold(),
        " Generate dialog ".into(),
        "<Enter>".blue().bold(),
    ]);
    let footer = Paragraph::new(line).centered();

    frame.render_widget(footer, area);
}
