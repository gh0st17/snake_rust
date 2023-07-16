use rand::Rng;
use crossterm::style::Color;

use crate::ui::{Pos, Drawable, ui_items::Symbol};

pub struct Food {
  pub symbol: Symbol,
  value: u16,
}

impl Food {
  pub fn new(symbol: Symbol) -> Self {
    Self { symbol, value: 0 }
  }

  pub fn value(mut self, value: u16) -> Self {
    self.value = value;
    self
  }

  pub fn get_value(&self) -> u16 {
    self.value
  }

  pub fn get_pos(&self) -> Pos {
    self.symbol.pos
  }

  pub fn generate_food(field_size: &Pos, kind: bool, snake_pos: Pos) -> Self {
    let mut rng = rand::thread_rng();
    let food: Self;
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
        food = Self::new(
          Symbol::new(pos)
            .ch('◉')
            .color(Color::Green)
          ).value(10);
      }
      else {
        food = Self::new(
          Symbol::new(pos)
            .ch('◉')
            .color(Color::Yellow)
          ).value(20);
      }
    }
    else {
      food = Self::new(
        Symbol::new(pos)
          .ch('▬')
          .color(Color::Red)
        );
    }

    food
  }
}

impl Drawable for Food {
  fn draw(&self) -> std::io::Result<()> {
    self.symbol.draw()
  }
}

impl Drawable for &mut Food {
  fn draw(&self) -> std::io::Result<()> {
    self.symbol.draw()
  }
}