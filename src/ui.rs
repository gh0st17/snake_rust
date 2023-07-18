pub mod ui_items;
pub mod dimensions;
mod staticui;

use dimensions::{Pos, Size};

use staticui::StaticUI;

use ui_items::{
  Label, PopupMessage
};

use crate::error::{*, self};

use std::{
  io::{self, Result},
  thread::sleep,
  time::Duration
};

use crossterm::{
  terminal::{*, self},
  cursor,
  style::{self, Color::*, Stylize},
  execute
};

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 14;

pub trait Drawable {
  fn draw(&self) -> Result<()>;
}

pub struct UI {
  pub field_size: Size,
  static_ui: StaticUI,
  score: Label,
  s_length: Label,
  time: Label
}

impl UI {
  pub fn new() -> error::Result<UI> {
    enable_raw_mode()
      .expect("Could not turn on Raw mode");

    let (mut width, mut height) = crossterm::terminal::size()?;

    if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
      return Err(SnakeError::Dimension(MINIMUM_WIDTH, MINIMUM_HEIGHT));
    }
    
    execute!(
      io::stdout(),
      terminal::Clear(ClearType::All),
      cursor::Hide
    )?;

    width  = 23 + (width  - MINIMUM_WIDTH);
    height = 12 + (height - MINIMUM_HEIGHT);
    let field_size = Size{ width, height };
    let static_ui = StaticUI::new(field_size);
    static_ui.draw()?;
    
    Ok(UI {
      field_size,
      static_ui,
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

  pub fn print_static(&self) -> Result<()> {
    self.static_ui.draw()
  }

  pub fn print_popup_message(&self, message: String, is_delayed: bool) -> Result<()> {
    let mut origin = Pos::from(terminal::size()?);
    origin.x /= 2;
    origin.y /= 2;
    origin.x -= (message.chars().count() as u16 / 2) + 2;
    origin.y -= 2;
    
    PopupMessage::new(Pos::from(origin), message).draw()?;

    if is_delayed {
      sleep(Duration::from_secs(3));
    }

    Ok(())
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

  pub fn draw<D: Drawable>(&self, drawable: &D) -> Result<()> {
    drawable.draw()
  }

  pub fn draw_vec<D: Drawable>(&self, drawables: &Vec<D>) -> Result<()> {
    for drawable in drawables {
      drawable.draw()?;
    }

    Ok(())
  }
}

impl Drop for UI {
  fn drop(&mut self) {
    execute!(
      io::stdout(),
      Clear(ClearType::All),
      cursor::Show,
      style::ResetColor
    ).unwrap();
    disable_raw_mode()
      .expect("Could not disable raw mode");
  }
}