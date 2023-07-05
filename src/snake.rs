#[derive(Copy, Clone)]
pub enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(Copy, Clone)]
pub struct SnakePart {
  symbol: char,
  pos_x: u16,
  pos_y: u16
}

impl SnakePart {
  pub fn new(symbol: char, pos_x: u16, pos_y: u16) -> SnakePart {
    SnakePart { symbol, pos_x, pos_y }
  }

  pub fn get_pos(&self) -> (u16, u16) {
    (self.pos_x, self.pos_y)
  }

  pub fn set_pos(&mut self, new_pos: (u16, u16)) {
    self.pos_x = new_pos.0;
    self.pos_y = new_pos.1;
  }

  pub fn get_symbol(&self) -> char {
    self.symbol
  }

  pub fn update(&mut self, dir: Direction, max_size: (u16, u16)) {
    match dir {
      Direction::UP    => self.pos_y -= 1,
      Direction::DOWN  => self.pos_y += 1,
      Direction::LEFT  => self.pos_x -= 1,
      Direction::RIGHT => self.pos_x += 1
    }

    if self.pos_x == 1 {
      self.pos_x = max_size.0 + 1;
    }
    else if self.pos_x == max_size.0 + 2 {
      self.pos_x = 2;
    }

    if self.pos_y == 0 {
      self.pos_y = max_size.1;
    }
    else if self.pos_y == max_size.1 + 1{
      self.pos_y = 1;
    }
  }
}

pub struct Snake {
  parts: Vec<SnakePart>,
  dir: Direction
}

impl Snake {
  pub fn new() -> Snake {
    Snake {
      parts: vec![SnakePart::new('O', 3, 1)],
      dir: Direction::RIGHT
    }
  }

  pub fn get_parts(&self) -> &Vec<SnakePart> {
    &self.parts
  }

  pub fn update(&mut self, max_size: (u16, u16)) -> (u16, u16) {
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

  pub fn add_part(&mut self, pos: (u16, u16)) {
    self.parts.push(SnakePart {
      symbol: 'o',
      pos_x: pos.0, 
      pos_y: pos.1
    })
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

  pub fn check_pos(&self, pos: (u16, u16)) -> bool {
    for part in &self.parts {
      if pos == part.get_pos() {
        return true;
      }
    }

    false
  }
}