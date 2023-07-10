use crate::snake::{Snake, Direction};
use crate::food::Food;
use crate::ui::{UI, Pos};

use std::{
  io::Result,
  thread::{sleep, self, JoinHandle},
  time::Duration,
  sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering}
  }
};

use atomic::Atomic;
use crossterm::{
  event::*,
  terminal,
  style::Color::*
};

#[derive(Clone)]
pub struct Game {
  is_over: Arc<AtomicBool>,
  pause: Arc<AtomicBool>,
  boost: Arc<AtomicBool>,
  score: u16,
  dir: Arc<Atomic<Direction>>,
  snake: Arc<Mutex<Snake>>,
  ui: Arc<Mutex<UI>>,
  field_size: (u16, u16),
  terminal_size: (u16, u16)
}

impl Game {
  pub fn new(ui: UI) -> Self {
    Game {
      is_over: Arc::new(AtomicBool::new(false)),
      pause: Arc::new(AtomicBool::new(false)),
      boost: Arc::new(AtomicBool::new(false)),
      score: 0,
      dir: Arc::new(Atomic::new(Direction::RIGHT)),
      field_size: ui.field_size,
      snake: Arc::new(Mutex::new(Snake::new())),
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: terminal::size().unwrap()
    }
  }

  pub fn time(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let ui = self.ui.clone();

    let handle = thread::spawn(move || -> Result<()> { 
      let (mut time, delay) = (0.0f64, 0.1f64);
  
      loop {
        if !pause.load(Ordering::Acquire) {
          ui.lock().unwrap().print_time(&time)?;
          time += delay;
        }

        if stop_bool.load(Ordering::Acquire) {
          break;
        }
        else {
          sleep(Duration::from_millis(100));
        }
      }

      Ok(())  
    });

    Ok(handle)
  }

  pub fn snake_update(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let dir = self.dir.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = self.field_size;
    
    let s_length = snake.lock().unwrap().get_parts().len() as u16 - 1;

    let result = self.ui
      .lock()
      .unwrap()
      .print_stats(&self.score, &s_length);

    match result {
      Ok(_) => (),
      Err(err) => {
        panic!("Ошибка при печати статистики: {}", err)
      }
    };

    let mut shared_self = self.clone();

    let handle = thread::spawn(move || -> Result<()> {
      let mut snake_pos = snake.lock().unwrap().get_pos();
      let mut food = Food::generate_food(&field_size, true, snake_pos);
      let mut brick = Food::generate_food(&field_size, false, snake_pos);
      let local_self = &mut shared_self;

      ui.lock().unwrap().print_food(&food)?;
      ui.lock().unwrap().print_food(&brick)?;
      ui.lock().unwrap().print_snake(&snake.lock().unwrap())?;

      loop {
        if pause.load(Ordering::Acquire) {
          sleep(Duration::from_millis(100));
          continue;
        }

        snake.lock().unwrap().set_direction(dir.load(Ordering::Acquire));
        let last_pos = snake.lock().unwrap().update(field_size);

        ui.lock().unwrap().clear_char(last_pos)?;
        ui.lock().unwrap().print_snake(&snake.lock().unwrap())?;

        if snake.lock().unwrap().check_self_eaten() {
          ui.lock().unwrap()
            .print_end_game_message("Сам себя съел!", true)?;

          stop_bool.store(true, Ordering::Release);
          break;
        }

        if snake.lock().unwrap().check_pos(food.get_pos()) {
          local_self.food_generator(&mut food, &mut brick, &mut snake_pos)?;
        }

        if snake.lock().unwrap().check_pos(brick.get_pos()) {
          ui.lock().unwrap()
            .print_end_game_message("Съел кирпич!", true)?;

          stop_bool.store(true, Ordering::Release);
          break;
        }

        if stop_bool.load(Ordering::Acquire) {
          break;
        }
        else {
          if boost.load(Ordering::Acquire) {
            sleep(Duration::from_millis(125))
          }
          else {
            sleep(Duration::from_millis(225));
          }
        }
      }

      Ok(())
    });

    Ok(handle)
  }

  pub fn fetch_key(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost: Arc<AtomicBool> = self.boost.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let dir = self.dir.clone();

    let handle = thread::spawn(move || -> Result<()> {
      loop {
        let event = read().unwrap();

        if !pause.load(Ordering::Acquire) {
          if event == Event::Key(KeyCode::Char('w').into()) || 
            event == Event::Key(KeyCode::Up.into()) {
            dir.store(Direction::UP, Ordering::Release);
          }
          if event == Event::Key(KeyCode::Char('s').into()) || 
            event == Event::Key(KeyCode::Down.into()) {
            dir.store(Direction::DOWN, Ordering::Release);
          }
          if event == Event::Key(KeyCode::Char('a').into()) || 
            event == Event::Key(KeyCode::Left.into()) {
            dir.store(Direction::LEFT, Ordering::Release);
          }
          if event == Event::Key(KeyCode::Char('d').into()) || 
            event == Event::Key(KeyCode::Right.into()) {
            dir.store(Direction::RIGHT, Ordering::Release);
          }

          if event == Event::Key(KeyCode::Char('b').into()) {
            let _boost = boost.load(Ordering::Acquire);
            boost.store(!_boost, Ordering::Release);
            let mut _snake = snake.lock().unwrap();

            if !_boost {
              _snake.set_head_color(Cyan);
            }
            else {
              _snake.set_head_color(Green);
            }
          }
        }

        if event == Event::Key(KeyCode::Char('p').into()) {
          let _pause = pause.load(Ordering::Acquire);
          pause.store(!_pause, Ordering::Release);
          let _ui = ui.lock().unwrap();

          if !_pause {
            _ui.print_end_game_message("Пауза", false)?;
          }
          else {
            _ui.print_frame_help()?;
          }
        }

        if event == Event::Key(KeyCode::Esc.into()) {
          ui.lock().unwrap()
            .print_end_game_message("Прерывание...", true)?;

          stop_bool.store(true, Ordering::Release);
          break;
        }
      }

      Ok(())
    });

    Ok(handle)
  }

  fn food_generator(&mut self, food: &mut Food, brick: &mut Food, snake_pos: &mut Pos) -> Result<()> {
    self.score += food.get_value();

    self.ui.lock().unwrap().print_stats(
      &self.score,
      &(self.snake.lock().unwrap().get_parts().len() as u16)
    )?;

    let pos = food.get_pos();
    self.snake.lock().unwrap().add_part(pos);
    
    *snake_pos = self.snake.lock().unwrap().get_pos();
    loop {
      *food = Food::generate_food(&self.field_size, true, *snake_pos);

      if !self.snake.lock().unwrap().check_pos(food.get_pos()) {
        break;
      }
    }

    self.ui.lock().unwrap().print_food(&food)?;
    self.ui.lock().unwrap().clear_char(brick.get_pos())?;

    loop {
      *brick = Food::generate_food(&self.field_size, false, *snake_pos);

      if !self.snake.lock().unwrap().check_pos(brick.get_pos()) &&
          food.get_pos() != brick.get_pos() {
        break;
      }
    }

    self.ui.lock().unwrap().print_food(&brick)?;

    Ok(())
  }

  pub fn terminal_size_checker(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let terminal_size = self.terminal_size.clone();

    let handle = thread::spawn(move || -> Result<()> {
      loop {
        if terminal_size != terminal::size().unwrap() {
          stop_bool.store(true, Ordering::Release);
          break;
        }
        else {
          sleep(Duration::from_millis(200));
        }
      }

      Ok(())
    });

    Ok(handle)
  } 

  pub fn stop(&mut self) {
    self.is_over.store(true, Ordering::Release);
  }

  pub fn is_over(&self) -> bool {
    self.is_over.load(Ordering::Acquire)
  }
}