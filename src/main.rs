mod cards;
mod tui;
mod widgets;

use std::io::{self};

use cards::{get_deck, Card};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    Frame,
};
use widgets::{render_card, render_controls, render_counter, render_title};

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
        let main_layout = Layout::vertical([
            Constraint::Max(1),
            Constraint::Max(1),
            Constraint::Min(2),
            Constraint::Max(1),
        ])
        .flex(Flex::SpaceBetween);

        let [header_area, counter_area, body_area, footer_area] = main_layout.areas(frame.area());

        render_title(frame, header_area);
        render_counter(frame, counter_area, &self.card_counter, &self.cards.len());
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

    fn increment_counter(&mut self) -> () {
        // TODO: Add card to spent_cards
        if self.show_back {
            let card = self.cards.pop();
            match card {
                Some(card) => {
                    self.current_card = card;
                    self.card_counter += 1;
                }
                None => self.exit(),
            };
        }
        self.show_back = !self.show_back;
    }

    fn decrement_counter(&mut self) {
        // TODO: Retrieve card from spent_cards to go back
        // TODO: Put card back into deck
        // self.show_back = !self.show_back;
        let card = self.spent_cards.pop();
        match card {
            Some(card) => {
                self.current_card = card;
            }
            None => (),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = ArkanaApp::default().run(&mut terminal);

    tui::restore()?;
    app_result
}
