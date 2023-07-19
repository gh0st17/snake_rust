use crossterm::style::Color::{self, *};

use crate::ui::{
  dimensions::{Pos, Size},
  Drawable,
  ui_items::Symbol
};

#[derive(Copy, Clone)]
pub enum Direction { Up, Down, Left, Right }

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
  pub fn new(field_size: Size, dir: Direction) -> Snake {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..=field_size.width);
    let y = rng.gen_range(0..=field_size.height);

    Snake {
      parts: vec![
        SnakePart::new(
          Symbol::new(Pos::from((x + 2, y + 1)))
            .ch('◇')
            .color(Green)
          )
        ],
      dir
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
          matches!(self.dir, Direction::Left) &&
          matches!(dir, Direction::Right)
        ) && 
       !(
          matches!(self.dir, Direction::Right) &&
          matches!(dir, Direction::Left)
       ) &&
       !(
          matches!(self.dir, Direction::Up) &&
          matches!(dir, Direction::Down)
       ) && 
       !(
          matches!(self.dir, Direction::Down) &&
          matches!(dir, Direction::Up)
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