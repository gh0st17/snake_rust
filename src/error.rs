use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, SnakeError>;

#[derive(Debug)]
pub enum SnakeError {
  Dimension(u16, u16),
  Parse(std::io::Error)
}

impl fmt::Display for SnakeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SnakeError::Dimension(w, h)=>
            write!(f, "Минимальный размер терминала {} столбцов {} строк", w, h),
      SnakeError::Parse(ref e) => e.fmt(f),
    }
  }
}

impl error::Error for SnakeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      SnakeError::Parse(ref e) => Some(e),
      _ => None,
    }
  }
}

impl From<std::io::Error> for SnakeError {
  fn from(err: std::io::Error) -> SnakeError {
    SnakeError::Parse(err)
  }
}