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

const MINIMUM_WIDTH: u16 = 80;
const MINIMUM_HEIGHT: u16 = 14;

pub type Pos = (u16, u16);

pub struct UI {
  pub field_size: Pos
}

impl UI {
  pub fn new() -> Result<UI> {
    enable_raw_mode()
      .expect("Could not turn on Raw mode");

    let (mut width, mut height) = crossterm::terminal::size()?;

    if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
      println!("Минимальный размер терминала {} столбцов {} строк",
      MINIMUM_WIDTH, MINIMUM_HEIGHT);
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
        Print(format!("{: <1$}", "", width as usize).with(Cyan))
      )?;
    }

    width  = 23 + (width  - MINIMUM_WIDTH);
    height = 12 + (height - MINIMUM_HEIGHT);
    Ok(UI { field_size: (width, height) })
  }

  pub fn print_frame(&self) -> Result<()> {
    let terminal_size = terminal::size().unwrap();
    let substract = terminal_size.0 - self.field_size.0;
    let half = (substract as usize - 17) / 2;
    execute!(
      io::stdout(),
      MoveTo(1, 0),
      Print(format!(
        "╔{:═<1$}╗ ╔════", "",
        self.field_size.0 as usize
      ).with(Cyan).bold()),
      Print(format!(" Статистика ").with(Magenta)),
      Print(format!("════╗").with(Cyan).bold()),
      MoveTo(self.field_size.0 + 4, 4),
      Print(format!("╚{:═<1$}╝", "", 20).with(Cyan).bold()),

      MoveTo(self.field_size.0 + 4, 5),
      Print(format!("╔{:═<1$}", "", half - 1).with(Cyan).bold()),
      Print(" Инструкция ".with(Magenta)),
      Print(format!("{:═<1$}╗", "", half).with(Cyan).bold()),
      MoveTo(1, self.field_size.1 + 1),
      Print(format!(
        "╚{:═<1$}╝", "",
        self.field_size.0 as usize
      ).with(Cyan).bold())
    )?;

    for y in 1..=(self.field_size.1) { // Поле
      execute!(
        io::stdout(),
        MoveTo(1, y),
        Print(format!(
          "║{: <1$}║", "",
          self.field_size.0 as usize
        ).with(Cyan).bold())
      )?;
    }

    for y in 1..=3 { // Статистика
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!("║{: <1$}║", "", 20).with(Cyan).bold())
      )?;
    }

    for y in 6..=12 { // Инструкция
      execute!(
        io::stdout(),
        MoveTo(self.field_size.0 + 4, y),
        Print(format!(
          "║{: <1$}║", "",
          substract as usize - 6
        ).with(Cyan).bold())
      )?;
    }

    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 4, 13),
      Print(format!(
        "╚{:═<1$}╝", "",
        substract as usize - 6
      ).with(Cyan).bold())
    )
  }

  pub fn print_snake(&self, snake: &Snake) -> Result<()> {
    let mut symbol: char;
    let mut pos: Pos;

    for part in snake.get_parts().iter().rev() {
      symbol = part.get_symbol();
      pos = part.get_pos();

      execute!(
        io::stdout(),
        MoveTo(pos.0, pos.1),
        Print(format!("{}", symbol)
          .with(part.get_color())
          .bold())
      )?;
    }
    
    Ok(())
  }

  pub fn print_help(&self) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 5, 6),
      Print("Клавиши для перемещения - ".with(Cyan)),
      Print("WASD".with(Magenta).bold()),
      Print(" (англ. раскладка)".with(Cyan)),
      MoveTo(self.field_size.0 + 5, 7),
      Print("или ".with(Cyan)),
      Print("стрелки".with(Magenta)),
      Print(". ".with(Cyan)),
      Print("B".with(Magenta).bold()),
      Print(" - переключает режим ускорения.".with(Cyan)),
      MoveTo(self.field_size.0 + 5, 8),
      Print("P".with(Magenta).bold()),
      Print(" - пауза. ".with(Cyan)),
      Print("ESC".with(Magenta).bold()),
      Print(" для выхода. ".with(Cyan)),
      MoveTo(self.field_size.0 + 5, 10),
      Print("Зеленые ".with(Cyan)),
      Print("◉".with(Green)),
      Print(" и золотые ".with(Cyan)),
      Print("◉".with(Yellow)),
      Print(" яблоки добавляют 10 и 20".with(Cyan)),
      MoveTo(self.field_size.0 + 5, 11),
      Print(
        "очков соответственно. Игра заканчивается когда"
        .with(Cyan)),
      MoveTo(self.field_size.0 + 5, 12),
      Print("Змея".with(DarkGreen)),
      Print(" ест саму себя или кирпич ".with(Cyan)),
      Print("▃".with(Red)),
      Print(" .".with(Cyan)),
    )
  }

  pub fn print_stats(&self, score: &u16, s_length: &u16) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 5, 1),
      Print("Очки: ".with(Cyan)),
      Print(format!("{}", score).with(Magenta).bold()),

      MoveTo(self.field_size.0 + 5, 2),
      Print("Длина змеи: ".with(Cyan)),
      Print(format!("{}", s_length).with(Magenta).bold())
    )
  }

  pub fn print_time(&self, time: &f64) -> Result<()> {
    let minutes = (time / 60.0f64).floor();
    let seconds = time % 60.0f64;

    execute!(
      io::stdout(),
      MoveTo(self.field_size.0 + 5, 3),
      Print("Время: ".with(Cyan)),
      Print(format!(
        "{}м{:.1}с ",
        minutes as u64,
        seconds
      ).with(Magenta).bold())
    )
  }

  pub fn set_alternate(&self, is_alternate: bool) -> Result<()> {
    if is_alternate {
      execute!(
        io::stdout(),
        EnterAlternateScreen
      )
    }
    else {
      execute!(
        io::stdout(),
        LeaveAlternateScreen
      )
    }
  }

  pub fn print_end_game_message(&self, message: &str) -> Result<()> {
    let mut origin = terminal::size()?;
    let char_count = message.chars().count();
    origin.0 /= 2;
    origin.1 /= 2;
    origin.0 -= (char_count as u16 / 2) + 2;
    origin.1 -= 2;

    execute!(
      io::stdout(),
      MoveTo(origin.0, origin.1),
      Print(format!(
        "╔{:═<1$}╗", "",
        char_count + 2
      ).with(DarkRed).bold()),
      MoveTo(origin.0, origin.1 + 1),
      Print(format!(
        "║{: <1$}║", "",
        char_count + 2
      ).with(DarkRed).bold()),
      MoveTo(origin.0, origin.1 + 2),
      Print(format!(
        "╚{:═<1$}╝", "",
        char_count + 2
      ).with(DarkRed).bold()),

      MoveTo(origin.0 + 2, origin.1 + 1),
      Print(message.with(DarkRed).bold()),
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
      Print(food.get_symbol().with(food.get_color())),
    )
  }

  pub fn clear_char(&self, pos: Pos) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(pos.0, pos.1),
      Print(" ")
    )
  }
}

impl Drop for UI {
  fn drop(&mut self) {
    execute!(
      io::stdout(),
      Clear(ClearType::All),
      cursor::Show,
      ResetColor
    ).unwrap();
    disable_raw_mode()
      .expect("Could not disable raw mode");
  }
}