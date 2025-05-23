use std::{
    error::Error,
    io::{self, stdout, Stdout},
    ops::Add,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    prelude::CrosstermBackend,
    Frame, Terminal,
};

use crate::{
    cards::{get_deck, Card},
    widgets,
};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Default)]
pub struct ArkanaApp {
    card_counter: usize, // Card counter 0 indexed
    show_back: bool,
    exit: bool,
    cards: Vec<Card>,
    show_confirmation_bar: bool,
}

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub struct AppConfig {
    pub file_path: String,
    pub has_headers: bool,
}

impl ArkanaApp {
    pub fn run(&mut self, terminal: &mut Tui, config: AppConfig) -> Result<(), Box<dyn Error>> {
        self.cards = get_deck(config.file_path, config.has_headers)?;

        if self.cards.len() < 1 {
            panic!("No cards in deck");
        }

        // Shuffle cards
        self.cards.shuffle(&mut thread_rng());

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

        widgets::render_title(frame, header_area);
        widgets::render_counter(
            frame,
            counter_area,
            &self.card_counter,
            &self.cards.len().saturating_sub(self.card_counter.add(1)),
        );
        widgets::render_card(
            frame,
            body_area,
            &self.cards[self.card_counter],
            self.show_back,
        );

        if self.show_confirmation_bar {
            widgets::render_confirmation_bar(frame, footer_area);
        } else {
            widgets::render_controls(frame, footer_area);
        }
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
        use KeyCode::*;

        if self.show_confirmation_bar {
            self.handle_confirmation_key(key_event.code);
            return;
        }

        match key_event.code {
            Char('q') => self.exit(),
            Left | Char('h') => self.decrement_counter(),
            Right | Char('l') => self.increment_counter(),
            _ => {}
        }
    }

    fn handle_confirmation_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('y') => self.exit = true,
            KeyCode::Char('n') | KeyCode::Char('q') => self.show_confirmation_bar = false,
            _ => {}
        }
    }

    fn increment_counter(&mut self) {
        if self.show_back {
            if self.card_counter > self.cards.len().saturating_sub(1) {
                self.exit();
            } else {
                self.card_counter += 1;
            }
        }
        self.show_back = !self.show_back;
    }

    fn decrement_counter(&mut self) {
        // saturating_sub makes sure we don't go below 0
        self.card_counter = self.card_counter.saturating_sub(1);
        self.show_back = true;
    }

    fn exit(&mut self) {
        // TODO: "Are you sure" prompt
        self.show_confirmation_bar = !self.show_confirmation_bar;
    }
}
