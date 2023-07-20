use crossterm::event::{
  KeyEvent, KeyCode, read, Event
};

use std::io::Result;

pub enum KeyAction {
  MoveUp, MoveDown,
  MoveLeft, MoveRight,
  Boost, Pause, Exit,
  None
}

pub struct KeyController {
  up_keys: Vec<KeyEvent>,
  down_keys: Vec<KeyEvent>,
  left_keys: Vec<KeyEvent>,
  right_keys: Vec<KeyEvent>,
  boost_keys: Vec<KeyEvent>,
  pause_keys: Vec<KeyEvent>,
  exit_key: KeyEvent
}

impl KeyController {
  pub fn new() -> Self {
    let up_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('w').into(),
      KeyCode::Char('ц').into(),
      KeyCode::Up.into()
    ];

    let down_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('s').into(),
      KeyCode::Char('ы').into(),
      KeyCode::Down.into()
    ];

    let left_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('a').into(),
      KeyCode::Char('ф').into(),
      KeyCode::Left.into()
    ];

    let right_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('d').into(),
      KeyCode::Char('в').into(),
      KeyCode::Right.into()
    ];

    let boost_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('b').into(),
      KeyCode::Char('и').into()
    ];

    let pause_keys: Vec<KeyEvent> = vec![
      KeyCode::Char('p').into(),
      KeyCode::Char('з').into(),
      KeyCode::Pause.into()
    ];

    let exit_key = KeyCode::Esc.into();

    Self {
      up_keys, down_keys, left_keys, right_keys,
      boost_keys, pause_keys, exit_key
    }
  }

  fn check_vec(&self, keys: &Vec<KeyEvent>, key_event: &KeyEvent) -> bool {
    for key in keys {
      if key == key_event {
        return true;
      }
    }

    false
  }

  pub fn fetch_action(&self) -> Result<KeyAction> {
    let event = read()?;
    let mut action = KeyAction::None;

    match event {
      Event::Key(key_event) => {
        if self.check_vec(&self.up_keys, &key_event) {
          action = KeyAction::MoveUp;
        }
        else if self.check_vec(&self.down_keys, &key_event) {
          action = KeyAction::MoveDown;
        }
        else if self.check_vec(&self.left_keys, &key_event) {
          action = KeyAction::MoveLeft;
        }
        else if self.check_vec(&self.right_keys, &key_event) {
          action = KeyAction::MoveRight;
        }
        else if self.check_vec(&self.boost_keys, &key_event) {
          action = KeyAction::Boost;
        }
        else if self.check_vec(&self.pause_keys, &key_event) {
          action = KeyAction::Pause;
        }
        else if key_event == self.exit_key {
          action = KeyAction::Exit;
        }
      },
      _ => action = KeyAction::None
    }

    Ok(action)
  }
}