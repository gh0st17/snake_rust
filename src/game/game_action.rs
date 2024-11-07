use crossterm::event::{
  KeyCode, read, Event, KeyEventKind
};

use std::{
  collections::HashMap, io::Result, time::Duration
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
    let event;
    let mut action = KeyAction::None;

    if crossterm::event::poll(Duration::from_millis(100))? {
      event = read()?;
    } else {
      return Ok(action);
    }

    match event {
      Event::Key(key_event) => {
        if let KeyEventKind::Press = key_event.kind {
          if let Some(key) = self.keys.get(&key_event.code) {
            action = key.clone()
          }
        }
        else {
          action = KeyAction::None
        }
      },
      _ => action = KeyAction::None
    }

    Ok(action)
  }
}