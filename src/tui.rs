use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{
    layout::{Constraint, Flex, Layout},
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
    spent_cards: Vec<Card>,
    cards: Vec<Card>,
    current_card: Card,
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

// TODO: App Config (file path, delim, header, etc.)
pub struct AppConfig {
    pub file_path: Option<String>,
    pub delimiter: Option<String>,
    pub header: Option<bool>,
}

impl ArkanaApp {
    // TODO: Use AppConfig
    pub fn run(&mut self, terminal: &mut Tui, config: AppConfig) -> io::Result<()> {
        self.cards = get_deck(config.file_path.unwrap()).unwrap();

        if self.cards.len() < 1 {
            panic!("No cards in deck");
        }

        // Shuffle cards
        self.cards.shuffle(&mut thread_rng());

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

        widgets::render_title(frame, header_area);
        widgets::render_counter(frame, counter_area, &self.card_counter, &self.cards.len());
        widgets::render_card(frame, body_area, &self.current_card, self.show_back);
        widgets::render_controls(frame, footer_area);
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
