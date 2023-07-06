extern crate crossterm;
use crate::{snake::Snake, food::Food};

use std::{
  process::exit,
  io::{self, Result},
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
const DEFAULT_HEIGHT: u16 = 13;

pub struct UI {
  pub field_size: (u16, u16)
}

impl UI {
  pub fn new() -> Result<UI> {
    enable_raw_mode()
      .expect("Could not turn on Raw mode");

    let (mut width, mut height) = crossterm::terminal::size()?;

    if width < DEFAULT_WIDTH || height < DEFAULT_HEIGHT {
      println!("Минимальный размер терминала {} столбцов {} строк",
      DEFAULT_WIDTH, DEFAULT_HEIGHT);
      exit(1);
    }
    
    execute!(
      io::stdout(),
      terminal::Clear(ClearType::All),
      cursor::Hide
    )?;

    for _ in 0..height {
      execute!(
        io::stdout(),
        SetColors(Colors::new(Cyan, Black)),
        Print(format!("{: <1$}", "", width as usize))
      )?;
    }

    width  = 23 + (width  - DEFAULT_WIDTH);
    height = 11 + (height - DEFAULT_HEIGHT);
    Ok(UI { field_size: (width, height) })
  }

  pub fn print_frame(&self) -> Result<()> {
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
    )?;

    for y in 1..=(self.field_size.1) { // Поле
      execute!(
        io::stdout(),
        MoveTo(1, y),
        Print(format!("║{: <1$}║", "", self.field_size.0 as usize))
      )?;
    }

    for y in 1..=3 { // Статистика
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!("║{: <1$}║", "", 20))
      )?;
    }

    for y in 6..=11 { // Инструкция
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!("║{: <1$}║", "", substract as usize - 6))
      )?;
    }

    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 4, 12),
      Print(format!("╚{:═<1$}╝", "", substract as usize - 6))
    )
  }

  pub fn print_snake(&self, snake: &Snake) -> Result<()> {
    let mut symbol: char;
    let mut pos: (u16, u16);

    execute!(
      io::stdout(),
      SetAttribute(Attribute::Bold),
      SetColors(Colors::new(Green, Black)),
    )?;

    for part in snake.get_parts() {
      symbol = part.get_symbol();
      pos = part.get_pos();

      execute!(
        io::stdout(),
        MoveTo(pos.0, pos.1),
        Print(format!("{}", symbol))
      )?;
    }

    execute!(
      io::stdout(),
      SetAttribute(Attribute::NormalIntensity)
    )

  }

  pub fn print_help(&self) -> Result<()> {
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
    )
  }

  pub fn print_stats(&self, score: &u16, s_length: &u16) -> Result<()> {
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
    )
  }

  pub fn print_time(&self, time: &f64) -> Result<()> {
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
    )
  }

  pub fn print_end_game_message(&self, message: &str) -> Result<()> {
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
    )?;

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
    )?;

    sleep(Duration::from_secs(3));

    Ok(())
  }

  pub fn print_food(&self, food: &Food) -> Result<()> {
    let pos = food.get_pos();
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      SetColors(Colors::new(food.get_color(), Black)),
      Print(food.get_symbol()),
    )
  }

  pub fn clear_char(&self, pos: (u16, u16)) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      Print(" ".to_string())
    )
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
