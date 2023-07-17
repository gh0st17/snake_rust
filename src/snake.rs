use crossterm::style::Color::{self, *};

use crate::ui::{
  dimensions::{Pos, Size},
  Drawable,
  ui_items::Symbol
};

#[derive(Copy, Clone)]
pub enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(Copy, Clone)]
pub struct SnakePart {
  symbol: Symbol
}

impl SnakePart {
  pub fn new(symbol: Symbol) -> SnakePart {
    SnakePart { symbol }
  }

  pub fn get_pos(&self) -> Pos {
    self.symbol.pos
  }

  pub fn set_pos(&mut self, new_pos: Pos) {
    self.symbol.pos = new_pos;
  }

  pub fn set_color(&mut self, color: Color) {
    self.symbol.color = color;
  }

  pub fn update(&mut self, dir: Direction, max_size: Size) {
    match dir {
      Direction::UP    => self.symbol.pos.y -= 1,
      Direction::DOWN  => self.symbol.pos.y += 1,
      Direction::LEFT  => self.symbol.pos.x -= 1,
      Direction::RIGHT => self.symbol.pos.x += 1
    }

    if self.symbol.pos.x == 1 {
      self.symbol.pos.x =  max_size.width + 1;
    }
    else if self.symbol.pos.x == max_size.width + 2 {
      self.symbol.pos.x = 2;
    }

    if self.symbol.pos.y == 0 {
      self.symbol.pos.y = max_size.height;
    }
    else if self.symbol.pos.y == max_size.height + 1 {
      self.symbol.pos.y = 1;
    }
  }
}

impl Drawable for SnakePart {
  fn draw(&self) -> std::io::Result<()> {
    self.symbol.draw()
  }
}

pub struct Snake {
  parts: Vec<SnakePart>,
  dir: Direction
}

impl Snake {
  pub fn new() -> Snake {
    Snake {
      parts: vec![
        SnakePart::new(
          Symbol::new(Pos::from((3, 1)))
            .ch('◇')
            .color(Green)
          )
        ],
      dir: Direction::RIGHT
    }
  }

  pub fn get_parts(&self) -> &Vec<SnakePart> {
    &self.parts
  }

  pub fn update(&mut self, max_size: Size) -> Pos {
    let mut prev_pos = self.parts[0].get_pos();
    let mut new_pos = prev_pos.clone();

    self.parts[0].update(self.dir, max_size);

    for i in 1..self.parts.len() {
      prev_pos = self.parts[i].get_pos();
      self.parts[i].set_pos(new_pos);
      new_pos = prev_pos;
    }

    new_pos
  }

  pub fn set_direction(&mut self, dir: Direction) {
    if !(
          matches!(self.dir, Direction::LEFT) &&
          matches!(dir, Direction::RIGHT)
        ) && 
       !(
          matches!(self.dir, Direction::RIGHT) &&
          matches!(dir, Direction::LEFT)
       ) &&
       !(
          matches!(self.dir, Direction::UP) &&
          matches!(dir, Direction::DOWN)
       ) && 
       !(
          matches!(self.dir, Direction::DOWN) &&
          matches!(dir, Direction::UP)
       ) {
        self.dir = dir;
       }
  }

  pub fn add_part(&mut self, pos: Pos) {
    self.parts.push(
      SnakePart::new(
        Symbol::new(pos)
          .ch('◆')
          .color(DarkGreen)
        )
      );
  }

  pub fn check_self_eaten(&self) -> bool {
    if self.parts.len() == 1 {
      return false;
    }

    for i in 1..self.parts.len() {
      if self.parts[0].get_pos() == self.parts[i].get_pos() {
        return true;
      }
    }

    false
  }

  pub fn check_pos(&self, pos: &Pos) -> bool {
    for part in &self.parts {
      if *pos == part.get_pos() {
        return true;
      }
    }

    false
  }

  pub fn get_head_pos(&self) -> Pos {
    self.parts.first().unwrap().get_pos()
  }

  pub fn set_head_color(&mut self, color: Color) {
    let head = &mut self.parts[0];
    head.set_color(color);
  }
}

impl Drawable for Snake {
  fn draw(&self) -> std::io::Result<()> {
    for part in self.parts.iter().rev() {
      part.draw()?;
    }

    Ok(())
  }
}