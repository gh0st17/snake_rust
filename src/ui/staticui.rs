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
    let x = field_size.width + 5;
    Self {
      field_size,
      static_labels: vec![
        Label::new(
          Pos::from((x, 1)),
          "Очки:".to_string()
            .with(Cyan)
        ),
        Label::new(
          Pos::from((x, 2)),
          "Длина змеи:".to_string()
            .with(Cyan)
        ),
        Label::new(
          Pos::from((x, 3)),
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

    let x = self.field_size.width + 4;

    self.print_frame(
      Pos::from((x, 0)),
      Size::from((20, 3)),
      "Статистика"
    )?;

    let terminal_size = terminal::size()?;
    self.print_frame(
      Pos::from((x, 5)),
      Size::from((terminal_size.0 - self.field_size.width - 6, 7)),
      "Инструкция"
    )
  }

  fn print_help(&self) -> Result<()> {
    let field_size = &self.field_size;
    let green_appl = get_food_with_type(FoodType::GreenApple);
    let gold_appl  = get_food_with_type(FoodType::GoldApple);
    let brick      = get_food_with_type(FoodType::Brick);
    let x = field_size.width + 5;

    execute!(
      io::stdout(),
      MoveTo(x, 6),
      Print("Клавиши для перемещения - ".with(Cyan)),
      Print("WASD".with(Magenta).bold()),
      Print(" или ".with(Cyan)),
      Print("стрелки".with(Magenta)),
      Print(". ".with(Cyan)),
      MoveTo(x, 7),
      Print("B".with(Magenta).bold()),
      Print(" - переключает режим ускорения.".with(Cyan)),
      MoveTo(x, 8),
      Print("P".with(Magenta).bold()),
      Print(" - пауза. ".with(Cyan)),
      Print("ESC".with(Magenta).bold()),
      Print(" для выхода. ".with(Cyan)),
      MoveTo(x, 10),
      Print("Яблоки ".with(Cyan)),
      Print(format!(
        "{} {} {} {} {} {}",
          green_appl.get_symbol(),
          gold_appl.get_symbol(),
          "различных цветов добавляют".with(Cyan),
          green_appl.get_value()
            .to_string()
            .with(green_appl.get_symbol().color),
          "и".with(Cyan),
          gold_appl.get_value()
            .to_string()
            .with(gold_appl.get_symbol().color)
        )),
      MoveTo(x, 11),
      Print(
        "очков соответственно. Игра заканчивается когда"
        .with(Cyan)),
      MoveTo(x, 12),
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