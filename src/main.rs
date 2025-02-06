mod cards;
mod tui;
mod widgets;

use std::io::{self};

use cards::{get_deck, Card};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};
use widgets::{render_card, render_controls, render_title};

#[derive(Debug, Default)]
pub struct ArkanaApp {
    card_counter: usize, // Card counter 0 indexed
    show_back: bool,
    exit: bool,
    spent_cards: Vec<Card>,
    cards: Vec<Card>,
    current_card: Card,
}

impl ArkanaApp {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.cards = get_deck();
        self.spent_cards = vec![];
        self.current_card = self.cards.pop().unwrap();

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let main_layout =
            Layout::vertical([Constraint::Max(2), Constraint::Fill(0), Constraint::Max(2)])
                .flex(Flex::SpaceBetween);

        let [header_area, body_area, footer_area] = main_layout.areas(frame.area());

        // frame.render_widget(self, header_area);
        render_title(frame, header_area);
        // TODO: Center this one
        render_card(frame, body_area, &self.current_card, self.show_back);
        render_controls(frame, footer_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => self.decrement_counter(),
            KeyCode::Right | KeyCode::Char('l') => self.increment_counter(),
            _ => {}
        }
    }

    fn increment_counter(&mut self) {
        if self.show_back {
            let card = self.cards.pop();
            match card {
                Some(card) => self.current_card = card,
                None => self.exit(),
            };
        }
        self.show_back = !self.show_back;
    }

    fn decrement_counter(&mut self) {
        // self.show_back = !self.show_back;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &ArkanaApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout =
            Layout::horizontal([Constraint::Max(2), Constraint::Min(2), Constraint::Min(2)])
                .flex(Flex::SpaceBetween);

        let title = Title::from(" Arkana ".bold());

        let instructions = Title::from(Line::from(vec![
            " Back ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let counter_block = Block::new()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Card number: ".into(),
            self.card_counter.to_string().yellow(),
            " In Deck: ".into(),
            self.cards.len().to_string().into(),
            " ".into(),
            self.show_back.to_string().into(),
        ])])
        .alignment(Alignment::Center);

        // let card_text = Text::from(vec![
        //     Line::from(self.current_card.front.as_str()),
        //     if self.show_back {
        //         Line::from(self.current_card.back.as_str()).alignment(Alignment::Center)
        //     } else {
        //         Line::from("")
        //     },
        // ]);

        Paragraph::new(counter_text)
            .centered()
            .block(counter_block)
            .render(area, buf);

        // render_card(self, area, self.current_card, self.show_back);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = ArkanaApp::default().run(&mut terminal);

    tui::restore()?;
    app_result
}
