use anyhow::Result;
use crossterm::event;

use pokemon::{ApiList, Card};
use ratatui::DefaultTerminal;

mod pokemon;

fn main() -> Result<()> {
  dotenvy::dotenv().ok();

  ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
  search: String,
  pokemon_cards: Vec<ApiList<Card>>,
}

impl App {
  const fn new() -> Self {
    Self {
      search: String::new(),
      pokemon_cards: Vec::new(),
    }
  }

  fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
        if event::read()?.is_key_press() {
            break Ok(());
        }
    }
  }
}
