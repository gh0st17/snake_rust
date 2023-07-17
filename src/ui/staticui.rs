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

use crate::ui::ui_items::Label;
use crate::food::{FoodType, get_food_with_type};

pub struct StaticUI {
  field_size: (u16, u16),
  static_labels: Vec<Label>
}

impl StaticUI {
  pub fn new(field_size: (u16, u16)) -> Self {
    Self {
      field_size,
      static_labels: vec![
        Label::new(
          (field_size.0 + 5, 1),
          "Очки:".to_string()
            .with(Cyan)
        ),
        Label::new(
          (field_size.0 + 5, 2),
          "Длина змеи:".to_string()
            .with(Cyan)
        ),
        Label::new(
          (field_size.0 + 5, 3),
          "Время:".to_string()
            .with(Cyan)
        )
      ],
    }
  }

  fn print_frame(&self) -> Result<()> {
    let field_size = &self.field_size;
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

  pub fn print_help(&self) -> Result<()> {
    let field_size = &self.field_size;
    let green_appl = get_food_with_type(FoodType::GreenApple);
    let gold_appl  = get_food_with_type(FoodType::GoldApple);
    let brick      = get_food_with_type(FoodType::Brick);

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
      Print(format!(
        "{} {} {} {} {} {} {} {}",
          "Зеленые и".with(Cyan),
          green_appl.get_symbol(),
          "золотые".with(Cyan),
          gold_appl.get_symbol(),
          "яблоки добавляют".with(Cyan),
          green_appl.get_value()
            .to_string()
            .with(Cyan),
          "и".with(Cyan),
          gold_appl.get_value()
            .to_string()
            .with(Cyan)
        )),
      MoveTo(field_size.0 + 5, 11),
      Print(
        "очков соответственно. Игра заканчивается когда"
        .with(Cyan)),
      MoveTo(field_size.0 + 5, 12),
      Print("Змея".with(DarkGreen)),
      Print(" ест саму себя или кирпич ".with(Cyan)),
      Print(brick.get_symbol()),
      Print(" .".with(Cyan)),
    )
  }
}

use crate::ui::Drawable;

impl Drawable for StaticUI {
  fn draw(&self) -> Result<()>{
    for label in &self.static_labels {
      label.draw()?;
    }

    Self::print_frame(&self)?;
    Self::print_help(&self)
  }
}