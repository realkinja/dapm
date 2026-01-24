use crate::app::App;
use ratatui::{
    Frame, style::Style, style::Stylize, symbols::border, text::Line, text::Text, widgets::Block,
    widgets::Paragraph,
};

pub fn ui(frame: &mut Frame, app: &App) {
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

    if let Some(dialog) = &app.current_dialog {
        let dialog_line = Line::styled(dialog.line.clone(), Style::default().bold());
        let mut lines: Vec<Line> = vec![dialog_line];
        if let Some(options) = &dialog.options {
            for option in options {
                let option = format!("> {} ({})", option.line, option.tone);
                lines.push(Line::from(option));
            }
        }

        let text = Text::from(lines);

        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, frame.area());
    } else {
        let paragraph = Paragraph::new("Please generate a dialog.").block(block);
        frame.render_widget(paragraph, frame.area());
    }
}
