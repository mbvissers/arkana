mod cards;
mod tui;

use std::io::{self};

use cards::Card;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

#[derive(Debug, Default)]
pub struct App {
    card_counter: usize, // Card counter 0 indexed
    show_back: bool,
    exit: bool,
    spent_cards: Vec<Card>,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
        let cards = cards::get_deck();
        if (self.card_counter < usize::max_value()
            && self.card_counter < cards::get_deck().len() - 1)
            || !self.show_back
        {
            if self.show_back {
                self.card_counter += 1;
            }
            self.show_back = !self.show_back;
        }
    }

    fn decrement_counter(&mut self) {
        if self.card_counter != 0 || self.show_back {
            self.show_back = !self.show_back;
            if self.show_back {
                self.card_counter -= 1;
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from(" Arkana ".bold());
        let instructions = Title::from(Line::from(vec![
            " Back ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::THICK);

        let cards = cards::get_deck();

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Value: ".into(),
                self.card_counter.to_string().yellow(),
            ]),
            Line::from(cards[self.card_counter].front.as_str()),
            if self.show_back {
                Line::from(cards[self.card_counter].back.as_str())
            } else {
                Line::from("")
            },
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);

    tui::restore()?;
    app_result
}
