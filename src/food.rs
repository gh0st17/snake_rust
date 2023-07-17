use rand::Rng;
use crossterm::style::Color;

use crate::ui::{Pos, Drawable, ui_items::Symbol};

pub enum FoodType {
  GreenApple, GoldApple, Brick
}

pub trait Food {
  fn get_symbol(&self) -> Symbol;
  fn get_value(&self) -> u16;
  fn get_pos(&self) -> Pos;
}

impl Drawable for Box<dyn Food> {
  fn draw(&self) -> std::io::Result<()> {
    self.get_symbol().draw()
  }
}

struct GreenApple(Pos);
impl Food for GreenApple {
  fn get_symbol(&self) -> Symbol {
    Symbol::new(self.0)
      .ch('◉')
      .color(Color::Green)
  }

  fn get_value(&self) -> u16 { 10 }

  fn get_pos(&self) -> Pos { self.0 }
}

struct GoldApple(Pos);
impl Food for GoldApple {
  fn get_symbol(&self) -> Symbol {
    Symbol::new(self.0)
      .ch('◉')
      .color(Color::Yellow)
  }

  fn get_value(&self) -> u16 { 20 }

  fn get_pos(&self) -> Pos { self.0 }
}

struct Brick(Pos);
impl Food for Brick {
  fn get_symbol(&self) -> Symbol {
    Symbol::new(self.0)
      .ch('▬')
      .color(Color::Red)
  }

  fn get_value(&self) -> u16 { 0 }

  fn get_pos(&self) -> Pos { self.0 }
}

pub fn generate_food(field_size: &Pos, kind: bool, snake_pos: Pos) -> Box<dyn Food> {
  let mut rng = rand::thread_rng();
  let mut pos = (0, 0);

  loop {
    pos.0 = rng.gen_range(3..=field_size.0);
    pos.1 = rng.gen_range(2..=field_size.1);

    if pos.0 != snake_pos.0 && pos.1 != snake_pos.1 {
      break;
    }
  }
  
  if kind {
    let apple = rng.gen_range(0..=1u16);
    if apple == 0 {
      Box::new(GreenApple(pos))
    }
    else {
      Box::new(GoldApple(pos))
    }
  }
  else {
    Box::new(Brick(pos))
  }
}

pub fn get_food_with_type(food_type: FoodType) -> Box<dyn Food> {
  match food_type {
    FoodType::GreenApple => Box::new(GreenApple((0, 0))),
    FoodType::GoldApple => Box::new(GoldApple((0, 0))),
    FoodType::Brick => Box::new(Brick((0, 0)))
  }
}