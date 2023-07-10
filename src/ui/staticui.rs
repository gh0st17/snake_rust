use std::io::{self, Result};

use crossterm::{
  terminal,
  cursor::{MoveTo, self},
  style::{
    Print,
    Color::*,
    *
  },
  execute
};

pub struct StaticUI;

impl StaticUI {
  pub fn print_frame(field_size: &(u16, u16)) -> Result<()> {
    let terminal_size = terminal::size().unwrap();
    let substract = terminal_size.0 - field_size.0;
    let half = (substract as usize - 17) / 2;
    let size_str = format!(
      " {}x{} ",
      field_size.0,
      field_size.1
    );
    let len_str = size_str.chars().count()
;
    execute!(
      io::stdout(),
      MoveTo(1, 0),
      Print(format!(
        "╔{:═<1$}╗ ╔════", "",
        field_size.0 as usize
      ).with(Cyan).bold()),
      cursor::SavePosition,
      MoveTo((field_size.0 - len_str as u16 / 2) / 2, 0),
      Print(size_str.with(Magenta)),
      cursor::RestorePosition,
      Print(format!(" Статистика ").with(Magenta)),
      Print(format!("════╗").with(Cyan).bold()),
      MoveTo(field_size.0 + 4, 4),
      Print(format!("╚{:═<1$}╝", "", 20).with(Cyan).bold()),

      MoveTo(field_size.0 + 4, 5),
      Print(format!("╔{:═<1$}", "", half - 1).with(Cyan).bold()),
      Print(" Инструкция ".with(Magenta)),
      Print(format!("{:═<1$}╗", "", half).with(Cyan).bold()),
      MoveTo(1, field_size.1 + 1),
      Print(format!(
        "╚{:═<1$}╝", "",
        field_size.0 as usize
      ).with(Cyan).bold())
    )?;

    for y in 1..=(field_size.1) { // Поле
      execute!(
        io::stdout(),
        MoveTo(1, y),
        Print("║".with(Cyan).bold()),
        MoveTo(field_size.0 + 2, y),
        Print("║".with(Cyan).bold())
      )?;
    }

    for y in 1..=3 { // Статистика
      execute!(
        io::stdout(),
        MoveTo(field_size.0 + 4, y),
        Print("║".with(Cyan).bold()),
        MoveTo(field_size.0 + 25, y),
        Print("║".with(Cyan).bold())
      )?;
    }

    for y in 6..=12 { // Инструкция
      execute!(
        io::stdout(),
        MoveTo(field_size.0 + 4, y),
        Print("║".with(Cyan).bold()),
        MoveTo(field_size.0 + substract + 7, y),
        Print("║".with(Cyan).bold())
      )?;
    }

    execute!(
      io::stdout(),
      MoveTo(field_size.0 + 4, 13),
      Print(format!(
        "╚{:═<1$}╝", "",
        substract as usize - 6
      ).with(Cyan).bold())
    )
  }

  pub fn print_help(field_size: &(u16, u16)) -> Result<()> {
    execute!(
      io::stdout(),
      MoveTo(field_size.0 + 5, 6),
      Print("Клавиши для перемещения - ".with(Cyan)),
      Print("WASD".with(Magenta).bold()),
      Print(" (англ. раскладка)".with(Cyan)),
      MoveTo(field_size.0 + 5, 7),
      Print("или ".with(Cyan)),
      Print("стрелки".with(Magenta)),
      Print(". ".with(Cyan)),
      Print("B".with(Magenta).bold()),
      Print(" - переключает режим ускорения.".with(Cyan)),
      MoveTo(field_size.0 + 5, 8),
      Print("P".with(Magenta).bold()),
      Print(" - пауза. ".with(Cyan)),
      Print("ESC".with(Magenta).bold()),
      Print(" для выхода. ".with(Cyan)),
      MoveTo(field_size.0 + 5, 10),
      Print("Зеленые ".with(Cyan)),
      Print("◉".with(Green)),
      Print(" и золотые ".with(Cyan)),
      Print("◉".with(Yellow)),
      Print(" яблоки добавляют 10 и 20".with(Cyan)),
      MoveTo(field_size.0 + 5, 11),
      Print(
        "очков соответственно. Игра заканчивается когда"
        .with(Cyan)),
      MoveTo(field_size.0 + 5, 12),
      Print("Змея".with(DarkGreen)),
      Print(" ест саму себя или кирпич ".with(Cyan)),
      Print("▬".with(Red)),
      Print(" .".with(Cyan)),
    )
  }

  pub fn print_end_game_message(message: &str) -> Result<()> {
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
      Print(message.with(DarkRed).bold())
    )?;

    Ok(())
  }
}