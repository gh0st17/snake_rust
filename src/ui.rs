pub mod ui_items;
pub mod dimensions;
mod staticui;

use dimensions::{Pos, Size};

use staticui::StaticUI;

use ui_items::{
  Symbol, Label, PopupMessage
};

use crate::error::{*, self};

use std::io::{Result, stdout};

use crossterm::{
  terminal::{*, self},
  cursor,
  style::{Color::*, Stylize},
  execute
};

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 14;

pub trait Drawable {
  fn draw(&self) -> Result<()>;
}

pub struct UI {
  pub field_size: Size,
  score: Label,
  s_length: Label,
  time: Label
}

impl UI {
  pub fn new() -> error::Result<UI> {
    let (mut width, mut height) = crossterm::terminal::size()?;

    if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
      return Err(SnakeError::Dimension(MINIMUM_WIDTH, MINIMUM_HEIGHT));
    }

    enable_raw_mode()?;
    
    execute!(
      stdout(),
      terminal::Clear(ClearType::All),
      cursor::Hide
    )?;

    width  = 27 + (width  - MINIMUM_WIDTH);
    height = 12 + (height - MINIMUM_HEIGHT);
    let field_size = Size{ width, height };
    let static_ui = StaticUI::new(field_size);
    static_ui.draw()?;
    
    Ok(UI {
      field_size,
      score: Label::new(
        Pos::from((width + 11, 1)),
        "0".to_string()
          .with(Magenta)
          .bold()
        ),
      s_length: Label::new(
        Pos::from((width + 17, 2)),
        "0".to_string()
          .with(Magenta)
          .bold()
        ),
      time: Label::new(
        Pos::from((width + 12, 3)),
        "0м0.0с".to_string()
          .with(Magenta)
          .bold()
        )
    })
  }

  pub fn clear_popup_message(&self) -> Result<()> {
    let terminal_size = terminal::size()?;
    let start_x = terminal_size.0 - 27;
    for x in start_x..terminal_size.0 {
      for y in 1..5 {
        Symbol::new(Pos::from((x, y))).draw()?;
      }
    }

    Ok(())
  }

  pub fn print_popup_message(&self, message: &str) -> Result<()> {
    let mut x = terminal::size()?.0 - 14;
    x -= (message.chars().count() as u16 / 2) + 2;
    
    PopupMessage::new(Pos::from((x, 1)), message.to_string()).draw()
  }

  pub fn print_stats(&mut self, score: &u16, s_length: &u16) -> Result<()> {
    self.score.set_message(
      score.to_string().with(Magenta).bold()
    );
    self.s_length.set_message(
      s_length.to_string().with(Magenta).bold()
    );

    self.score.draw()?;
    self.s_length.draw()
  }

  pub fn print_time(&mut self, time: &f64) -> Result<()> {
    let minutes = (time / 60.0).floor() as u64;
    let seconds = time % 60.0;

    self.time.set_message(
      format!(
        "{}м{:.1}с ",
        minutes,
        seconds
      ).with(Magenta).bold()
    );

    self.time.draw()
  }

  pub fn draw<D>(&self, drawable: &D) -> Result<()>
  where D: Drawable, {
    drawable.draw()
  }

  pub fn draw_vec<D>(&self, drawables: &Vec<D>) -> Result<()>
  where D: Drawable, {
    for drawable in drawables {
      drawable.draw()?;
    }

    Ok(())
  }

  pub fn disable_raw_mode(&self) {
    execute!(stdout(), cursor::Show).unwrap();
    disable_raw_mode()
      .expect("Could not disable raw mode");
  }
}