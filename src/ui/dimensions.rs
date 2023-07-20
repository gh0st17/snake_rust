use core::fmt;
use crossterm::cursor::MoveTo;

#[derive(Copy, Clone, PartialEq)]
pub struct Pos {
  pub x: u16,
  pub y: u16
}

impl Pos {
  pub fn add_x(mut self, x: u16) -> Self {
    self.x += x;
    self
  }

  pub fn add_y(mut self, y: u16) -> Self {
    self.y += y;
    self
  }

  pub fn is_overlaps(&self, pos: &Self) -> bool {
    self.x == pos.x || self.y == pos.y
  }
}

impl From<Pos> for MoveTo {
  fn from(pos: Pos) -> MoveTo {
    MoveTo(pos.x, pos.y)
  }
}

impl From<(u16, u16)> for Pos {
  fn from(pos: (u16, u16)) -> Pos {
    Pos {
      x: pos.0,
      y: pos.1
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Size {
  pub width: u16,
  pub height: u16
}

impl Size {
  pub fn add_width(mut self, width: u16) -> Self {
    self.width += width;
    self
  }

  pub fn add_height(mut self, height: u16) -> Self {
    self.height += height;
    self
  }
}

impl From<Size> for MoveTo {
  fn from(size: Size) -> MoveTo {
    MoveTo(size.width, size.height)
  }
}

impl From<(u16, u16)> for Size {
  fn from(size: (u16, u16)) -> Size {
    Size {
      width: size.0,
      height: size.1
    }
  }
}

impl fmt::Display for Size {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

#[cfg(test)]
mod tests {
  use super::{Pos, Size};

  #[test]
  fn test_pos_add() {
    let pos = Pos::from((0, 0));
    assert_eq!(pos.add_x(10).x, 10);
    assert_eq!(pos.add_y(10).y, 10);
  }

  #[test]
  fn test_pos_overlap() {
    let origin = Pos::from((5, 5));
    assert_eq!(origin.is_overlaps(&Pos::from((5, 5))), true);
    assert_eq!(origin.is_overlaps(&Pos::from((5, 6))), true);
    assert_eq!(origin.is_overlaps(&Pos::from((5, 4))), true);
    assert_eq!(origin.is_overlaps(&Pos::from((6, 5))), true);
    assert_eq!(origin.is_overlaps(&Pos::from((4, 5))), true);
    assert_eq!(origin.is_overlaps(&Pos::from((6, 54))), false);
  }

  #[test]
  fn test_size_add() {
    let size = Size::from((0, 0));
    assert_eq!(size.add_width(10).width, 10);
    assert_eq!(size.add_height(10).height, 10);
  }
}