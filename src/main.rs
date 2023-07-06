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
  let ui = UI::new();
  ui.print_frame();
  ui.print_help();

  let mut game = Game::new(ui);

  game.time();
  game.fetch_key();
  game.snake_update();
  game.food_generator();
  game.terminal_size_checker();

  while !game.is_over() {
   sleep(Duration::from_secs(3));
  }
}