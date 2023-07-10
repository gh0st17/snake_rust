pub mod ui;
pub mod food;
pub mod snake;
pub mod game;

use ui::UI;
use game::Game;

use std::{
  thread::sleep,
  time::Duration
};

fn main() {
  let ui = match UI::new() {
    Ok(ui) => ui,
    Err(err) => {
      panic!("Не могу инициализировать интерфейс: {}", err)
    }
  };

  match ui.print_frame() {
    Ok(_) => (),
    Err(err) => {
      panic!("Ошибка при отрисовке рамки: {}", err)
    }
  };
  
  match ui.print_help() {
    Ok(_) => (),
    Err(err) => {
      panic!("Ошибка при печати инструкции: {}", err)
    }
  };

  let mut game = Game::new(ui);

  let threads = vec![
    game.time(),
    game.fetch_key(),
    game.snake_update(),
    game.terminal_size_checker()
  ];

  for thread in threads {
    match thread {
      Ok(_) => (),
      Err(err) => {
        panic!("{}", err);
      }
    }
  }

  while !game.is_over() {
   sleep(Duration::from_secs(1));
  }
}