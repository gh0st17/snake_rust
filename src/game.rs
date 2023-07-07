use crate::snake::{Snake, Direction};
use crate::food::Food;
use crate::ui::{UI, Pos};

use std::time::Instant;
use std::{
  io::{Result},
  thread::{sleep, self, JoinHandle},
  time::Duration,
  sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU16, Ordering}
  }
};

use atomic::Atomic;
use crossterm::{event::*, terminal};

pub struct Game {
  is_over: Arc<AtomicBool>,
  score: Arc<AtomicU16>,
  dir: Arc<Atomic<Direction>>,
  snake: Arc<Mutex<Snake>>,
  ui: Arc<Mutex<UI>>,
  field_size: Pos,
  terminal_size: (u16, u16)
}

impl Game {
  pub fn new(ui: UI) -> Self {
    Game {
      is_over: Arc::new(AtomicBool::new(false)),
      score: Arc::new(AtomicU16::new(0)),
      dir: Arc::new(Atomic::new(Direction::RIGHT)),
      field_size: ui.field_size,
      snake: Arc::new(Mutex::new(Snake::new())),
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: terminal::size().unwrap()
    }
  }

  pub fn time(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let ui = self.ui.clone();

    let handle = thread::spawn(move || -> Result<()> { 
      let mut t2: Instant;
      let t1 = Instant::now();
  
      loop {
        t2 = Instant::now();

        ui.lock().unwrap().print_time(&(t2 - t1).as_secs_f64())?;

        if stop_bool.load(Ordering::Relaxed) {
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
    let dir = self.dir.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = self.field_size;
    
    let s_length = snake.lock().unwrap().get_parts().len() as u16 - 1;

    let result = self.ui
      .lock()
      .unwrap()
      .print_stats(
        &self.score.load(Ordering::Relaxed),
        &s_length
      );
    match result {
      Ok(_) => (),
      Err(err) => {
        panic!("Ошибка при печати статистики: {}", err)
      }
    };

    let handle = thread::spawn(move || -> Result<()> {
      ui.lock().unwrap()
        .print_snake(&snake.lock().unwrap())?;

      loop {
        snake.lock().unwrap().set_direction(dir.load(Ordering::Relaxed));
        let last_pos = snake.lock().unwrap().update(field_size);

        ui.lock().unwrap().clear_char(last_pos)?;
        ui.lock().unwrap().print_snake(&snake.lock().unwrap())?;

        if snake.lock().unwrap().check_self_eaten() {
          ui.lock().unwrap()
            .print_end_game_message("Сам себя съел!")?;

          stop_bool.store(true, Ordering::Relaxed);
          break;
        }

        if stop_bool.load(Ordering::Relaxed) {
          break;
        }
        else {
          sleep(Duration::from_millis(175));
        }
      }

      Ok(())
    });

    Ok(handle)
  }

  pub fn fetch_key(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let ui = self.ui.clone();
    let dir = self.dir.clone();

    let handle = thread::spawn(move || -> Result<()> {
      loop {
        let event = read().unwrap();

        if event == Event::Key(KeyCode::Char('w').into()) || 
           event == Event::Key(KeyCode::Up.into()) {
          dir.store(Direction::UP, Ordering::Relaxed);
        }
        if event == Event::Key(KeyCode::Char('s').into()) || 
           event == Event::Key(KeyCode::Down.into()) {
          dir.store(Direction::DOWN, Ordering::Relaxed);
        }
        if event == Event::Key(KeyCode::Char('a').into()) || 
           event == Event::Key(KeyCode::Left.into()) {
          dir.store(Direction::LEFT, Ordering::Relaxed);
        }
        if event == Event::Key(KeyCode::Char('d').into()) || 
           event == Event::Key(KeyCode::Right.into()) {
          dir.store(Direction::RIGHT, Ordering::Relaxed);
        }
        if event == Event::Key(KeyCode::Esc.into()) {
          ui.lock().unwrap()
            .print_end_game_message("Прерывание...")?;

          stop_bool.store(true, Ordering::Relaxed);
          break;
        }
      }

      Ok(())
    });

    Ok(handle)
  }

  pub fn food_generator(&mut self) -> Result<JoinHandle<Result<()>>> {
    let stop_bool = self.is_over.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = Arc::new(self.field_size);
    let score = self.score.clone();

    let handle = thread::spawn(move || -> Result<()> {
      let mut snake_pos = snake.lock().unwrap().get_pos();
      let mut food = Food::generate_food(&field_size, true, snake_pos);
      let mut brick = Food::generate_food(&field_size, false, snake_pos);

      ui.lock().unwrap().print_food(&food)?;
      ui.lock().unwrap().print_food(&brick)?;

      loop {
        if snake.lock().unwrap().check_pos(food.get_pos()) {
          score.fetch_add(food.get_value(), Ordering::SeqCst);

          ui.lock().unwrap().print_stats(
            &score.load(Ordering::Relaxed),
            &(snake.lock().unwrap().get_parts().len() as u16)
          )?;

          let pos = food.get_pos();
          snake.lock().unwrap().add_part(pos);
          
          snake_pos = snake.lock().unwrap().get_pos();
          loop {
            food = Food::generate_food(&field_size, true, snake_pos);

            if !snake.lock().unwrap().check_pos(food.get_pos()) {
              break;
            }
          }

          ui.lock().unwrap().print_food(&food)?;
          ui.lock().unwrap().clear_char(brick.get_pos())?;

          loop {
            brick = Food::generate_food(&field_size, false, snake_pos);

            if !snake.lock().unwrap().check_pos(brick.get_pos()) {
              break;
            }
          }

          ui.lock().unwrap().print_food(&brick)?;
        }

        if snake.lock().unwrap().check_pos(brick.get_pos()) {
          ui.lock().unwrap()
            .print_end_game_message("Съел кирпич!")?;

          stop_bool.store(true, Ordering::Relaxed);
          break;
        }

        if stop_bool.load(Ordering::Relaxed) {
          break;
        }
        else {
          sleep(Duration::from_millis(150));
        }
      }

      Ok(())
    });

    Ok(handle)
  }

  pub fn terminal_size_checker(&mut self) -> Result<JoinHandle<Result<()>>>  {
    let stop_bool = self.is_over.clone();
    let terminal_size = self.terminal_size.clone();

    let handle = thread::spawn(move || -> Result<()> {
      loop {     
        if terminal_size != terminal::size().unwrap() {
          stop_bool.store(true, Ordering::Relaxed);
          break;
        }
        else {
          sleep(Duration::from_millis(300));
        }
      }

      Ok(())
    });

    Ok(handle)
  } 

  pub fn stop(&mut self) {
    self.is_over.store(true, Ordering::Relaxed);
  }

  pub fn is_over(&self) -> bool {
    self.is_over.load(Ordering::Relaxed)
  }
}

impl Drop for Game {
  fn drop(&mut self) {
    self.stop();
  }
}