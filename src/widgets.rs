// TODO: Move widgets here, functions to generate Widget object
// TODO: https://ratatui.rs/examples/widgets/block/
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Paragraph},
    Frame,
};

use crate::cards::Card;

pub fn calculate_layout() {
    todo!()
}

pub fn render_title(frame: &mut Frame, area: Rect) {
    let text = Text::from(Line::from(Span::styled(
        " Arkana ",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    )));
    frame.render_widget(text.alignment(Alignment::Center), area);
}

pub fn render_counter(frame: &mut Frame, area: Rect) {
    todo!()
}

pub fn render_card(frame: &mut Frame, area: Rect, card: &Card, show_back: bool) {
    frame.render_widget(
        Text::from(vec![
            Line::from(card.front.as_str()).alignment(Alignment::Center),
            if show_back {
                Line::from(card.back.as_str()).alignment(Alignment::Center)
            } else {
                Line::from("")
            },
        ]),
        area,
    )
}

pub fn render_controls(frame: &mut Frame, area: Rect) {
    let text = Text::from(vec![Line::from(vec![
        Span::raw(" Back "),
        Span::styled(
            "<Left>",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Next "),
        Span::styled(
            "<Right>",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Quit "),
        Span::styled(
            "<Q>",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
    ])]);

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
