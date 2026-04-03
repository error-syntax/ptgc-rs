use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use pokemon::{ApiList, Card, fetch_cards};
use ratatui::{
  DefaultTerminal, Frame,
  layout::{Constraint, Layout, Rect},
  style::{Color, Style},
  widgets::{Block, Paragraph},
};
use tui_input::Input;

mod pokemon;

fn main() -> Result<()> {
  dotenvy::dotenv().ok();

  let mut terminal = ratatui::init();
  let result = App::default().run(&mut terminal);
  ratatui::restore();
  result
}

#[derive(Debug, Default)]
struct App {
  input: Input,
  input_mode: InputMode,
  pokemon_cards: Vec<ApiList<Card>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
  #[default]
  Normal,
  Editing,
}

impl App {
  fn run(self, terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
      terminal.draw(|frame| self.render(frame))?;

      let event = event::read()?;
      if let Event::Key(key) = event {
        match self.input_mode {
          InputMode::Normal => match key.code {
            KeyCode::Char('e') => self.start_editing(),
            KeyCode::Char('q') => return Ok(()), // exit
            _ => {}
          },
          InputMode::Editing => match key.code {
            KeyCode::Enter => self.send_query(self.input.value()),
            KeyCode::Esc => self.stop_editing(),
            _ => {
              self.input.handle_event(&event);
            }
          },
        }
      }
    }
  }

  fn start_editing(&mut self) {
    self.input_mode = InputMode::Editing
  }

  fn stop_editing(&mut self) {
    self.input_mode = InputMode::Normal
  }

  async fn send_query(&mut self, q: &str) -> Result<Vec<Card>, anyhow::Error> {
    let cards = fetch_cards(q).await;

    cards
  }

  fn render(&self, frame: &mut Frame) {
    let [input_area] = Layout::vertical([Constraint::Length(1)]).areas(frame.area());

    self.render_input(frame, input_area);
  }

  fn render_input(&self, frame: &mut Frame, area: Rect) {
    // keep 2 for borders and 1 for cursor
    let width = area.width.max(3) - 3;
    let scroll = self.input.visual_scroll(width as usize);
    let style = match self.input_mode {
      InputMode::Normal => Style::default(),
      InputMode::Editing => Color::Yellow.into(),
    };
    let input = Paragraph::new(self.input.value())
      .style(style)
      .scroll((0, scroll as u16))
      .block(Block::bordered().title("Input"));
    frame.render_widget(input, area);

    if self.input_mode == InputMode::Editing {
      // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
      // end of the input text and one line down from the border to the input line
      let x = self.input.visual_cursor().max(scroll) - scroll + 1;
      frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
  }
}
