
use crate::snake::Snake;
use crate::food::Food;
use crate::ui::Pos;

use std::io::{self, Result};

use crossterm::{
  cursor::MoveTo,
  style::{
    Print,
    Color::*,
    *
  },
  execute
};

pub struct DynamicUI;

impl DynamicUI {
  pub fn print_snake(snake: &Snake) -> Result<()> {
    let mut symbol: char;
    let mut pos: Pos;

    for part in snake.get_parts().iter().rev() {
      symbol = part.get_symbol();
      pos = part.get_pos();

      execute!(
        io::stdout(),
        MoveTo(pos.0, pos.1),
        Print(format!("{}", symbol)
          .with(part.get_color())
          .bold())
      )?;
    }
    
    Ok(())
  }

  pub fn print_stats(score: &u16, s_length: &u16, field_size: &(u16, u16)) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(field_size.0 + 5, 1),
      Print("Очки: ".with(Cyan)),
      Print(format!("{}", score).with(Magenta).bold()),

      MoveTo(field_size.0 + 5, 2),
      Print("Длина змеи: ".with(Cyan)),
      Print(format!("{}", s_length).with(Magenta).bold())
    )
  }

  pub fn print_time(time: &f64, field_size: &(u16, u16)) -> Result<()> {
    let minutes = (time / 60.0f64).floor();
    let seconds = time % 60.0f64;

    execute!(
      io::stdout(),
      MoveTo(field_size.0 + 5, 3),
      Print("Время: ".with(Cyan)),
      Print(format!(
        "{}м{:.1}с ",
        minutes as u64,
        seconds
      ).with(Magenta).bold())
    )
  }

  pub fn print_food(food: &Food) -> Result<()> {
    let pos = food.get_pos();
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      Print(food.get_symbol().with(food.get_color())),
    )
  }

  pub fn clear_char(pos: Pos) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      Print(" ")
    )
  }
}