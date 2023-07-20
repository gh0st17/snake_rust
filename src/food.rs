use rand::Rng;
use crossterm::style::Color;

use crate::ui::{
  dimensions::{Pos, Size},
  Drawable,
  ui_items::Symbol
};

use std::any::Any;

pub enum FoodType {
  GreenApple, GoldApple, Brick
}

pub trait Food {
  fn get_symbol(&self) -> Symbol;
  fn get_value(&self) -> u16;
  fn get_pos(&self) -> Pos;
  fn as_any(&self) -> &dyn Any;
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

  fn as_any(&self) -> &dyn Any { self }
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

  fn as_any(&self) -> &dyn Any { self }
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

  fn as_any(&self) -> &dyn Any { self }
}

pub fn generate_food(
  field_size: &Size, edible: bool, snake_pos: &Pos
) -> Box<dyn Food> {
  let mut rng = rand::thread_rng();
  let mut pos = Pos::from((0, 0));

  loop {
    pos.x = rng.gen_range(3..=field_size.width);
    pos.y = rng.gen_range(2..=field_size.height);

    if !pos.is_overlaps(snake_pos) {
      break;
    }
  }
  
  if edible {
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
    FoodType::GreenApple => Box::new(GreenApple(Pos::from((0, 0)))),
    FoodType::GoldApple  => Box::new(GoldApple(Pos::from((0, 0)))),
    FoodType::Brick      => Box::new(Brick(Pos::from((0, 0))))
  }
}

#[cfg(test)]
mod tests {
  use super::{
    GreenApple,
    GoldApple, Brick,
    FoodType, Pos, Size,
    get_food_with_type,
    generate_food
  };

  #[test]
  fn test_get_food_with_type() {
    let green_apple = get_food_with_type(FoodType::GreenApple);
    match green_apple.as_any().downcast_ref::<GreenApple>() {
      Some(_) => (),
      None => panic!("green_apple isn't a GreenApple!"),
    };

    let gold_apple = get_food_with_type(FoodType::GoldApple);
    match gold_apple.as_any().downcast_ref::<GoldApple>() {
      Some(_) => (),
      None => panic!("gold_apple isn't a GoldApple!"),
    };

    let brick = get_food_with_type(FoodType::Brick);
    match brick.as_any().downcast_ref::<Brick>() {
      Some(_) => (),
      None => panic!("brick isn't a Brick!"),
    };
  }

  #[test]
  fn test_generate_food() {
    let snake_pos = Pos::from((5, 5));
    let field_size = Size::from((10, 10));

    for _ in 0..10 {
      let apple = generate_food(&field_size, true, &snake_pos);
      match apple.as_any().downcast_ref::<GreenApple>() {
        Some(_) => (),
        None => {
          match apple.as_any().downcast_ref::<GoldApple>() {
            Some(_) => (),
            None => panic!("apple isn't one of AppleType!"),
          };
        },
      };

      assert_eq!(apple.get_pos().is_overlaps(&snake_pos), false);

      let brick = generate_food(&field_size, false, &snake_pos);
      match brick.as_any().downcast_ref::<Brick>() {
        Some(_) => (),
        None => panic!("brick isn't a Brick!"),
      };

      assert_eq!(brick.get_pos().is_overlaps(&snake_pos), false);
    }
  }
}