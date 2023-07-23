use crossterm::event::{
  KeyCode, read, Event
};

use std::{
  io::Result,
  collections::HashMap
};

#[derive(Clone, Copy)]
pub enum KeyAction {
  MoveUp, MoveDown,
  MoveLeft, MoveRight,
  Boost, Pause, Exit,
  None
}

pub struct KeyController {
  keys: HashMap<KeyCode, KeyAction>
}

impl KeyController {
  pub fn new() -> Self {
    let keys = HashMap::from([
      (KeyCode::Char('w'), KeyAction::MoveUp),
      (KeyCode::Char('ц'), KeyAction::MoveUp),
      (KeyCode::Up,        KeyAction::MoveUp),
      (KeyCode::Char('s'), KeyAction::MoveDown),
      (KeyCode::Char('ы'), KeyAction::MoveDown),
      (KeyCode::Down,      KeyAction::MoveDown),
      (KeyCode::Char('a'), KeyAction::MoveLeft),
      (KeyCode::Char('ф'), KeyAction::MoveLeft),
      (KeyCode::Left,      KeyAction::MoveLeft),
      (KeyCode::Char('d'), KeyAction::MoveRight),
      (KeyCode::Char('в'), KeyAction::MoveRight),
      (KeyCode::Right,     KeyAction::MoveRight),
      (KeyCode::Char('b'), KeyAction::Boost),
      (KeyCode::Char('и'), KeyAction::Boost),
      (KeyCode::Char('p'), KeyAction::Pause),
      (KeyCode::Char('з'), KeyAction::Pause),
      (KeyCode::Pause,     KeyAction::Pause),
      (KeyCode::Esc,       KeyAction::Exit)
    ]);

    Self { keys }
  }

  pub fn fetch_action(&self) -> Result<KeyAction> {
    let event = read()?;
    let action;

    match event {
      Event::Key(key_event) => {
        action = self.keys[&key_event.code];
      },
      _ => action = KeyAction::None
    }

    Ok(action)
  }
}