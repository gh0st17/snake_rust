use rand::Rng;
use crossterm::style::Color;

pub struct Food {
  symbol: char,
  pos: (u16, u16),
  value: u16,
  color: Color
}

impl Food {
  pub fn new(pos: (u16, u16)) -> Food {
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

  pub fn get_pos(&self) -> (u16, u16) {
    self.pos
  }

  pub fn get_value(&self) -> u16 {
    self.value
  }

  pub fn get_color(&self) -> Color {
    self.color
  }

  pub fn generate_food(field_size: &(u16, u16), kind: bool) -> Food {
    let mut rng = rand::thread_rng();
    let food: Food;
    let x = rng.gen_range(3..=field_size.0);
    let y = rng.gen_range(2..=field_size.1);
    if kind {
      let apple = rng.gen_range(0..=1u16);
      if apple == 0 {
        food = Food::new((x, y))
          .symbol('◉')
          .value(10)
          .color(Color::Green);
      }
      else {
        food = Food::new((x, y))
          .symbol('◉')
          .value(20)
          .color(Color::Yellow);
      }
    }
    else {
      food = Food::new((x, y))
        .symbol('▃')
        .value(0)
        .color(Color::Red);
    }

    food
  }
}