extern crate crossterm;
use crate::{snake::Snake, food::Food};

use std::{
  process::exit,
  io::{self},
  thread::sleep,
  time::Duration
};

use crossterm::{
  terminal::{*, self},
  cursor::{MoveTo, self},
  style::{
    Print,
    Color::*,
    *
  },
  execute
};

const DEFAULT_WIDTH: u16 = 80;
const DEFAULT_HEIGHT: u16 = 24;

pub struct UI {
  pub field_size: (u16, u16)
}

impl UI {
  pub fn new() -> Self {
    enable_raw_mode()
      .expect("Could not turn on Raw mode");
    
    execute!(
      io::stdout(),
      terminal::Clear(ClearType::All),
      cursor::Hide
    ).unwrap();

    let (mut width, mut height) = crossterm::terminal::size().unwrap();

    if width < DEFAULT_WIDTH || height < DEFAULT_HEIGHT {
      execute!(
        io::stdout(),
        Print(format!("Минимательный размер терминала {} столбцов {} строк",
          DEFAULT_WIDTH, DEFAULT_HEIGHT))
      ).unwrap();
      exit(1);
    }

    width  = 23 + (width  - DEFAULT_WIDTH);
    height = 22 + (height - DEFAULT_HEIGHT);
    UI { field_size: (width, height) }
  }

  pub fn print_frame(&self) {
    let terminal_size = terminal::size().unwrap();
    let substract = terminal_size.0 - self.field_size.0;
    let half = (substract as usize - 17) / 2;
    execute!(
      io::stdout(),
      MoveTo(1, 0),
      SetColors(Colors::new(Cyan, Black)),
      Print(format!("╔{:═<1$}╗ ╔════", "", self.field_size.0 as usize)),
      SetColors(Colors::new(Magenta, Black)),
      Print(format!(" Статистика ")),
      SetColors(Colors::new(Cyan, Black)),
      Print(format!("════╗")),
      MoveTo(self.field_size.0 + 4, 4),
      Print(format!("╚{:═<1$}╝", "", 20)),

      MoveTo(self.field_size.0 + 4, 5),
      Print(format!("╔{:═<1$}", "", half - 1)),
      SetColors(Colors::new(Magenta, Black)),
      Print(format!(" Инструкция ")),
      SetColors(Colors::new(Cyan, Black)),
      Print(format!("{:═<1$}╗", "", half)),
      MoveTo(1, self.field_size.1 + 1),
      Print(format!("╚{:═<1$}╝", "", self.field_size.0 as usize))
    ).unwrap();

    for y in 1..=(self.field_size.1) { // Поле
      execute!(
        io::stdout(),
        MoveTo(1, y),
        Print(format!("║{: <1$}║", "", self.field_size.0 as usize))
      ).unwrap();
    }

    for y in 1..=3 { // Статистика
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!("║{: <1$}║", "", 20))
      ).unwrap();
    }

    for y in 6..=11 { // Инструкция
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!("║{: <1$}║", "", substract as usize - 6))
      ).unwrap();
    }

    execute!(
      io::stdout(),

      MoveTo(self.field_size.0 + 4, 12),
      Print(format!("╚{:═<1$}╝", "", substract as usize - 6))
    ).unwrap();

  }

  pub fn print_snake(&self, snake: &Snake) {
    let mut symbol: char;
    let mut pos: (u16, u16);

    execute!(
      io::stdout(),
      SetAttribute(Attribute::Bold),
      SetColors(Colors::new(Green, Black)),
    ).unwrap();

    for part in snake.get_parts() {
      symbol = part.get_symbol();
      pos = part.get_pos();

      execute!(
        io::stdout(),
        SetAttribute(Attribute::Bold),
        SetColors(Colors::new(Green, Black)),
        MoveTo(pos.0, pos.1),
        Print(format!("{}", symbol))
      ).unwrap();
    }

    execute!(
      io::stdout(),
      SetAttribute(Attribute::NormalIntensity)
    ).unwrap();

  }

  pub fn print_help(&self) {
    execute!(
      io::stdout(),
      SetColors(Colors::new(Cyan, Black)),
      MoveTo(self.field_size.0 + 5, 6),
      Print("Клавиши для перемещения - WASD или стрелки."),
      MoveTo(self.field_size.0 + 5, 7),
      Print("ESC для выхода."),
      MoveTo(self.field_size.0 + 5, 9),
      Print("Зеленые "),
      SetColors(Colors::new(Green, Black)),
      Print("◉"),
      SetColors(Colors::new(Cyan, Black)),
      Print(" и золотые "),
      SetColors(Colors::new(Yellow, Black)),
      Print("◉"),
      SetColors(Colors::new(Cyan, Black)),
      Print(" яблоки добавляют 10 и 20"),
      MoveTo(self.field_size.0 + 5, 10),
      Print("очков соответственно. Игра заканчивается когда"),
      MoveTo(self.field_size.0 + 5, 11),
      SetColors(Colors::new(DarkGreen, Black)),
      Print("Змея"),
      SetColors(Colors::new(Cyan, Black)),
      Print(" ест саму себя или кирпич "),
      SetColors(Colors::new(Red, Black)),
      Print("▃"),
      SetColors(Colors::new(Cyan, Black)),
      Print(" ."),
    ).unwrap();
  }

  pub fn print_stats(&self, score: &u16, s_length: &u16) {
    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 5, 1),
      SetColors(Colors::new(Cyan, Black)),
      SetAttribute(Attribute::Bold),
      Print("Очки: "),
      SetColors(Colors::new(Magenta, Black)),
      SetAttribute(Attribute::NormalIntensity),
      Print(format!("{}", score)),

      MoveTo(self.field_size.0 + 5, 2),
      SetColors(Colors::new(Cyan, Black)),
      SetAttribute(Attribute::Bold),
      Print("Длина змеи: "),
      SetColors(Colors::new(Magenta, Black)),
      SetAttribute(Attribute::NormalIntensity),
      Print(format!("{}", s_length))
    ).unwrap();
  }

  pub fn print_time(&self, time: &f64) {
    let minutes = (time / 60.0f64).floor();
    let seconds = time % 60.0f64;

    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 5, 3),
      SetColors(Colors::new(Cyan, Black)),
      SetAttribute(Attribute::Bold),
      Print("Время: "),
      SetColors(Colors::new(Magenta, Black)),
      SetAttribute(Attribute::NormalIntensity),
      Print(format!("{}м{:.1}с ", minutes as u64, seconds))
    ).unwrap();
  }

  pub fn print_end_game_message(&self, message: &str) {
    let mut origin = terminal::size().unwrap();
    let char_count = message.chars().count();
    origin.0 /= 2;
    origin.1 /= 2;
    origin.0 -= char_count as u16 / 2;
    origin.1 -= 1;
    
    execute!(
      io::stdout(),
      SetAttribute(Attribute::Bold),
      SetColors(Colors::new(DarkRed, Black)),
      cursor::Show
    ).unwrap();

    execute!(
      io::stdout(),
      SetAttribute(Attribute::Bold),
      SetColors(Colors::new(DarkRed, Black)),
      MoveTo(origin.0, origin.1),
      Print(format!("╔{:═<1$}╗", "", char_count + 2)),
      MoveTo(origin.0, origin.1 + 1),
      Print(format!("║{: <1$}║", "", char_count + 2)),
      MoveTo(origin.0, origin.1 + 2),
      Print(format!("╚{:═<1$}╝", "", char_count + 2)),

      MoveTo(origin.0 + 2, origin.1 + 1),
      Print(message),
      SetAttribute(Attribute::NormalIntensity),
      MoveTo(0, 0)
    ).unwrap();

    sleep(Duration::from_secs(3));
  }

  pub fn print_food(&self, food: &Food) {
    let pos = food.get_pos();
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      SetColors(Colors::new(food.get_color(), Black)),
      Print(food.get_symbol()),
    ).unwrap();
  }

  pub fn clear_char(&self, pos: (u16, u16)) {
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      Print(" ".to_string())
    ).unwrap();
  }
}

impl Drop for UI {
  fn drop(&mut self) {
    execute!(
      io::stdout(),
      cursor::Show,
      ResetColor
    ).unwrap();
    disable_raw_mode()
      .expect("Could not disable raw mode");
  }
}
