use std::io::{self, Result};

use crossterm::{
  terminal,
  cursor::MoveTo,
  style::{Print, Color::*, Stylize},
  execute
};

use crate::ui::{
  dimensions::{Pos, Size},
  ui_items::Label
};

use crate::food::{
  FoodType,
  get_food_with_type
};

pub struct StaticUI {
  field_size: Size,
  static_labels: Vec<Label>
}

impl StaticUI {
  pub fn new(field_size: Size) -> Self {
    Self {
      field_size,
      static_labels: vec![
        Label::new(
          Pos::from((field_size.width + 5, 1)),
          "Очки:".to_string()
            .with(Cyan)
        ),
        Label::new(
          Pos::from((field_size.width + 5, 2)),
          "Длина змеи:".to_string()
            .with(Cyan)
        ),
        Label::new(
          Pos::from((field_size.width + 5, 3)),
          "Время:".to_string()
            .with(Cyan)
        )
      ],
    }
  }

  fn print_frame(&self, pos: Pos, size: Size, title: &str) -> Result<()> {
    let title_pos = Pos::from(
      (
        (size.width / 2 + 1) - 
        (title.chars().count() as u16 / 2 + 1) +
        pos.x, pos.y
      )
    );

    execute!(
      io::stdout(),
      MoveTo::from(pos),
      Print(format!(
        "╔{:═<1$}╗", "",
          size.width as usize
      ).with(Cyan).bold()),
    )?;

    for y in pos.y + 1..=pos.y + size.height {
      execute!(
        io::stdout(),
        MoveTo(pos.x, y),
        Print("║".with(Cyan).bold()),
        MoveTo(pos.x + size.width + 1, y),
        Print("║".with(Cyan).bold())
      )?;
    }

    execute!(
      io::stdout(),
      MoveTo::from(pos.add_y(size.height + 1)),
      Print(format!(
        "╚{:═<1$}╝", "",
          size.width as usize
      ).with(Cyan).bold()),

      MoveTo::from(title_pos),
      Print(format!(" {} ", title).with(Magenta))
    )
  }

  fn print_frames(&self) -> Result<()> {
    self.print_frame(
      Pos::from((1, 0)),
      self.field_size.clone(),
      self.field_size.to_string().as_str()
    )?;

    self.print_frame(
      Pos::from((self.field_size.add_width(4).width, 0)),
      Size::from((20, 3)),
      "Статистика"
    )?;

    let terminal_size = terminal::size()?;
    self.print_frame(
      Pos::from((self.field_size.add_width(4).width, 5)),
      Size::from((terminal_size.0 - self.field_size.width - 6, 7)),
      "Инструкция"
    )
  }

  fn print_help(&self) -> Result<()> {
    let field_size = &self.field_size;
    let green_appl = get_food_with_type(FoodType::GreenApple);
    let gold_appl  = get_food_with_type(FoodType::GoldApple);
    let brick      = get_food_with_type(FoodType::Brick);

    execute!(
      io::stdout(),
      MoveTo(field_size.width + 5, 6),
      Print("Клавиши для перемещения - ".with(Cyan)),
      Print("WASD".with(Magenta).bold()),
      Print(" (англ. раскладка)".with(Cyan)),
      MoveTo(field_size.width + 5, 7),
      Print("или ".with(Cyan)),
      Print("стрелки".with(Magenta)),
      Print(". ".with(Cyan)),
      Print("B".with(Magenta).bold()),
      Print(" - переключает режим ускорения.".with(Cyan)),
      MoveTo(field_size.width + 5, 8),
      Print("P".with(Magenta).bold()),
      Print(" - пауза. ".with(Cyan)),
      Print("ESC".with(Magenta).bold()),
      Print(" для выхода. ".with(Cyan)),
      MoveTo(field_size.width + 5, 10),
      Print(format!(
        "{} {} {} {} {} {} {} {}",
          "Зеленые и".with(Cyan),
          green_appl.get_symbol(),
          "золотые".with(Cyan),
          gold_appl.get_symbol(),
          "яблоки добавляют".with(Cyan),
          green_appl.get_value()
            .to_string()
            .with(green_appl.get_symbol().color),
          "и".with(Cyan),
          gold_appl.get_value()
            .to_string()
            .with(gold_appl.get_symbol().color)
        )),
      MoveTo(field_size.width + 5, 11),
      Print(
        "очков соответственно. Игра заканчивается когда"
        .with(Cyan)),
      MoveTo(field_size.width + 5, 12),
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

    Self::print_frames(&self)?;
    Self::print_help(&self)
  }
}