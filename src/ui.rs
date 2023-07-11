mod dynamicui;
mod staticui;

use dynamicui::DynamicUI;
use staticui::StaticUI;

use crate::{snake::Snake, food::Food, error::{*, self}};

use std::{
  io::{self, Result},
  thread::sleep,
  time::Duration
};

use crossterm::{
  terminal::{*, self},
  cursor, style,
  execute
};

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 14;

pub type Pos = (u16, u16);

pub struct UI {
  pub field_size: Pos
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
    Ok(UI { field_size: (width, height) })
  }

  fn print_frame(&self) -> Result<()> {
    StaticUI::print_frame(&self.field_size)
  }

  fn print_help(&self) -> Result<()> {
    StaticUI::print_help(&self.field_size)
  }

  pub fn print_frame_help(&self) -> Result<()>{
    self.print_frame()?;
    self.print_help()
  }

  pub fn print_end_game_message(&self, message: &str, is_delayed: bool) -> Result<()> {
    StaticUI::print_end_game_message(message)?;
    if is_delayed {
      sleep(Duration::from_secs(3));
    }

    Ok(())
  }

  pub fn print_stats(&self, score: &u16, s_length: &u16) -> Result<()> {
    DynamicUI::print_stats(score, s_length, &self.field_size)
  }

  pub fn print_time(&self, time: &f64) -> Result<()> {
    DynamicUI::print_time(time, &self.field_size)
  }

  pub fn print_snake(&self, snake: &Snake) -> Result<()> {
    DynamicUI::print_snake(snake)
  }

  pub fn print_food(&self, food: &Food) -> Result<()> {
    DynamicUI::print_food(food)
  }

  pub fn clear_char(&self, pos: Pos) -> Result<()> {
    DynamicUI::clear_char(pos)
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