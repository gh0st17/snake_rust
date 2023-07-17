use crate::ui::{Pos, Drawable};

use crossterm::{
  style::{
    Color::{self, *},
    Stylize,
    StyledContent,
    Print
  },
  cursor::MoveTo,
  execute
};

use core::fmt;
use std::io::{self, Result};

#[derive(Copy, Clone)]
pub struct Symbol {
  pub color: Color,
  pub pos: Pos,
  pub ch: char
}

impl Symbol {
  pub fn new(pos: Pos) -> Self {
    Self {
      color: White, 
      pos,
      ch: ' '
    }
  }

  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }

  pub fn ch(mut self, ch: char) -> Self {
    self.ch = ch;
    self
  }
}

impl Drawable for Symbol {
  fn draw(&self) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(self.pos.x, self.pos.y),
      Print(self.ch.with(self.color))
    )
  }
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.ch.with(self.color).fmt(f)
  }
}

pub struct Label {
  pos: Pos,
  message: StyledContent<String>
}

impl Label {
  pub fn new(pos: Pos, message: StyledContent<String>) -> Self {
    Self { pos, message }
  }

  pub fn set_message(&mut self, new_message: StyledContent<String>) {
    self.message = new_message;
  }
}

impl Drawable for Label {
  fn draw(&self) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo::from(self.pos),
      Print(&self.message)
    )
  }
}

pub struct PopupMessage {
  origin: Pos,
  message: String
}

impl PopupMessage {
  pub fn new(origin: Pos,  message: String) -> Self {
    Self { origin, message }
  }
}

impl Drawable for PopupMessage {
  fn draw(&self) -> Result<()> {
    let char_count = self.message.chars().count();

    execute!(
      io::stdout(),
      MoveTo::from(self.origin),
      Print(format!(
        "╔{:═<1$}╗", "",
        char_count + 2
      ).with(DarkRed).bold()),
      MoveTo::from(self.origin.add_y(1)),
      Print(format!(
        "║{: <1$}║", "",
        char_count + 2
      ).with(DarkRed).bold()),
      MoveTo::from(self.origin.add_y(2)),
      Print(format!(
        "╚{:═<1$}╝", "",
        char_count + 2
      ).with(DarkRed).bold()),

      MoveTo::from(self.origin.add_x(2).add_y(1)),
      Print(&self.message.clone().with(Red).bold())
    )?;

    Ok(())
  }
}