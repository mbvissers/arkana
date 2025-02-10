// TODO: Move widgets here, functions to generate Widget object
// TODO: https://ratatui.rs/examples/widgets/block/
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, Paragraph},
    Frame,
};

use crate::cards::Card;

pub fn render_title(frame: &mut Frame, area: Rect) {
    let text = Text::from(Line::from(Span::styled(
        " Arkana ",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    )));
    frame.render_widget(text.alignment(Alignment::Center), area);
}

pub fn render_counter(frame: &mut Frame, area: Rect, card_counter: &usize, card_length: &usize) {
    let cards_viewed = card_counter.to_string();
    let cards_left = card_length.to_string();
    let counter_text = Text::from(vec![Line::from(vec![
        "Cards viewed: ".into(),
        cards_viewed.as_str().into(),
        " Cards left: ".into(),
        cards_left.as_str().into(),
    ])])
    .alignment(Alignment::Center);
    frame.render_widget(counter_text.alignment(Alignment::Center), area);
}

pub fn render_card(frame: &mut Frame, area: Rect, card: &Card, show_back: bool) {
    let [centered_area] = Layout::horizontal([Constraint::Percentage(20)])
        .flex(Flex::Center)
        .areas(area);

    let [centered_area] = Layout::vertical([Constraint::Percentage(50)])
        .flex(Flex::Center)
        .areas(centered_area);

    frame.render_widget(
        Text::from(vec![
            Line::from(vec![Span::styled(
                card.front.as_str(),
                Style::default().bold(),
            )]),
            if show_back {
                Line::from(card.back.as_str())
            } else {
                Line::from("")
            },
        ])
        .centered(),
        centered_area,
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
