use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use crate::cards::Card;

pub fn render_title(frame: &mut Frame, area: Rect) {
    let text = Text::from(Line::from(Span::styled(
        " Arkana ",
        Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    frame.render_widget(text.alignment(Alignment::Center), area);
}

pub fn render_counter(frame: &mut Frame, area: Rect, spent_cards: &usize, card_length: &usize) {
    let cards_viewed = spent_cards.to_string();
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
    // Get the text dimensions
    let front_text = card.front.as_str();
    let back_text = if show_back { card.back.as_str() } else { "" };

    let centered_width = front_text.len().max(back_text.len()) as u16; // Approximation of the text width
    let centered_height = 2;

    // Calculate the centered area for the text
    let centered_x = area.x + (area.width.saturating_sub(centered_width)) / 2;
    let centered_y = area.y + (area.height.saturating_sub(centered_height)) / 2;

    let centered_area = Rect {
        x: centered_x,
        y: centered_y,
        width: centered_width,
        height: centered_height,
    };

    // Render the centered text
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
        ]),
        centered_area,
    );
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
