use rand::Rng;
use crossterm::style::Color;

use crate::ui::Pos;

pub struct Food {
  symbol: char,
  pos: Pos,
  value: u16,
  color: Color
}

impl Food {
  pub fn new(pos: Pos) -> Food {
    Food{
      symbol: ' ',
      pos,
      value: 0,
      color: Color::White 
    }
  }

  pub fn symbol(mut self, symbol: char) -> Food {
    self.symbol = symbol;
    self
  }

  pub fn value(mut self, value: u16) -> Food {
    self.value = value;
    self
  }

  pub fn color(mut self, color: Color) -> Food {
    self.color = color;
    self
  }

  pub fn get_symbol(&self) -> char {
    self.symbol
  }

  pub fn get_pos(&self) -> Pos {
    self.pos
  }

  pub fn get_value(&self) -> u16 {
    self.value
  }

  pub fn get_color(&self) -> Color {
    self.color
  }

  pub fn generate_food(field_size: &Pos, kind: bool, snake_pos: Pos) -> Food {
    let mut rng = rand::thread_rng();
    let food: Food;
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
        food = Food::new(pos)
          .symbol('◉')
          .value(10)
          .color(Color::Green);
      }
      else {
        food = Food::new(pos)
          .symbol('◉')
          .value(20)
          .color(Color::Yellow);
      }
    }
    else {
      food = Food::new(pos)
        .symbol('▃')
        .value(0)
        .color(Color::Red);
    }

    food
  }
}