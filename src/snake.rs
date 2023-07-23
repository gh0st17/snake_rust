use std::{sync::{Arc, Mutex}, io::Result};

use crossterm::style::Color::{self, *};

use crate::ui::{
  dimensions::{Pos, Size},
  Drawable,
  ui_items::Symbol, UI
};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction { Up, Down, Left, Right }

impl Direction {
  pub fn is_opposite(&self, other: &Self) -> bool {
    match self {
      Direction::Up => {
        if let Direction::Down = other {
          return true;
        }
      },
      Direction::Down => {
        if let Direction::Up = other {
          return true;
        }
      },
      Direction::Left => {
        if let Direction::Right = other {
          return true;
        }
      },
      Direction::Right => {
        if let Direction::Left = other {
          return true;
        }
      }
    }

    false
  }
}

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
      Direction::Up    => self.symbol.pos.y -= 1,
      Direction::Down  => self.symbol.pos.y += 1,
      Direction::Left  => self.symbol.pos.x -= 1,
      Direction::Right => self.symbol.pos.x += 1
    }

    if self.symbol.pos.x == max_size.width + 2 {
      self.symbol.pos.x = 2;
    }
    else if self.symbol.pos.x == 1 {
      self.symbol.pos.x = max_size.width + 1;
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
  dir: Direction,
  field_size: Size
}

impl Snake {
  pub fn new(field_size: Size, dir: Direction) -> Snake {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..field_size.width);
    let y = rng.gen_range(0..field_size.height);

    Snake {
      parts: vec![
        SnakePart::new(
          Symbol::new(Pos::from((x + 2, y + 1)))
            .ch('◇')
            .color(Green)
          )
        ],
      dir,
      field_size
    }
  }

  pub fn get_parts(&self) -> &Vec<SnakePart> {
    &self.parts
  }

  pub fn update(&mut self, ui: Arc<Mutex<UI>>) -> Result<()> {
    let mut prev_pos = self.parts[0].get_pos();
    let mut new_pos = prev_pos.clone();

    self.parts[0].update(self.dir, self.field_size);

    for i in 1..self.parts.len() {
      prev_pos = self.parts[i].get_pos();
      self.parts[i].set_pos(new_pos);
      new_pos = prev_pos;
    }

    ui.lock().unwrap().draw(&Symbol::new(new_pos))
  }

  pub fn set_direction(&mut self, dir: Direction) {
    if !self.dir.is_opposite(&dir) {
      self.dir = dir;
    }
  }

  pub fn add_part(&mut self) {
    self.parts.push(
      SnakePart::new(
        Symbol::new(
          self.parts.last().unwrap().get_pos()
        ).ch('◆').color(DarkGreen)
      )
    );
  }

  pub fn check_self_eaten(&self) -> bool {
    for i in 2..self.parts.len() {
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
    self.parts.first().unwrap().draw()?;
    if let Some(second_part) = self.parts.iter().nth(1) {
      second_part.draw()?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::ui::{
    ui_items::Symbol,
    dimensions::{Pos, Size}
  };
  use super::{Direction, SnakePart};

  #[test]
  fn test_is_opposite() {
    assert_eq!(
      Direction::Up
        .is_opposite(&Direction::Down), true);
    assert_eq!(
      Direction::Down
        .is_opposite(&Direction::Up), true);
    
    assert_eq!(
      Direction::Left
        .is_opposite(&Direction::Right), true);
    assert_eq!(
      Direction::Right
        .is_opposite(&Direction::Left), true);
    
    assert_eq!(
      Direction::Left
        .is_opposite(&Direction::Up), false);
    
    assert_eq!(
      Direction::Right
        .is_opposite(&Direction::Down), false);
  }

  #[test]
  fn test_snake_part_update() {
    let max_size = Size::from((10, 10));
    let pos = Pos::from((5, 5));
    let symbol = Symbol::new(pos);
    let mut snake_part = SnakePart::new(symbol);

    snake_part.update(Direction::Up, max_size);
    assert_eq!(snake_part.get_pos().y, 4);
    snake_part.update(Direction::Down, max_size);
    assert_eq!(snake_part.get_pos().y, 5);
    snake_part.update(Direction::Left, max_size);
    assert_eq!(snake_part.get_pos().x, 4);
    snake_part.update(Direction::Right, max_size);
    assert_eq!(snake_part.get_pos().x, 5);
    
    snake_part.set_pos(Pos::from((6, 4)));
    snake_part.update(Direction::Right, Size::from((5, 5)));
    assert_eq!(snake_part.get_pos().x, 2);
    snake_part.update(Direction::Left, Size::from((5, 5)));
    assert_eq!(snake_part.get_pos().x, 6);

    snake_part.update(Direction::Down, Size::from((5, 4)));
    assert_eq!(snake_part.get_pos().y, 1);
    snake_part.update(Direction::Up, Size::from((5, 4)));
    assert_eq!(snake_part.get_pos().y, 4);
  }
}