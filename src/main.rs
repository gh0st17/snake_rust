pub mod ui;
pub mod food;
pub mod snake;
pub mod game;
pub mod error;

use ui::UI;
use game::Game;

fn main() {
  let ui = match UI::new() {
    Ok(ui) => ui,
    Err(err) => {
      panic!("Не могу инициализировать интерфейс: {}", err)
    }
  };

  Game::new(ui).run();
}