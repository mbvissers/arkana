// TODO: Move widgets here, functions to generate Widget object
// TODO: https://ratatui.rs/examples/widgets/block/
use ratatui::{
    layout::{Alignment, Rect},
    text::{Line, Text},
    Frame,
};

use crate::cards::Card;

pub fn calculate_layout() {
    todo!()
}

pub fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(Text::from("Arkana"), area);
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
    todo!()
}
