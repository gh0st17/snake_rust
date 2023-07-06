use crate::snake::{Snake, Direction};
use crate::food::Food;
use crate::ui::UI;

use std::time::Instant;
use std::{
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
  running_thread_handles: Vec<JoinHandle<()>>,
  is_over: Arc<AtomicBool>,
  score: Arc<AtomicU16>,
  dir: Arc<Atomic<Direction>>,
  snake: Arc<Mutex<Snake>>,
  ui: Arc<Mutex<UI>>,
  field_size: (u16, u16),
  terminal_size: (u16, u16)
}

impl Game {
  pub fn new(ui: UI) -> Self {
    Game {
      running_thread_handles: Vec::new(),
      is_over: Arc::new(AtomicBool::new(false)),
      score: Arc::new(AtomicU16::new(0)),
      dir: Arc::new(Atomic::new(Direction::RIGHT)),
      field_size: ui.field_size,
      snake: Arc::new(Mutex::new(Snake::new())),
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: terminal::size().unwrap()
    }
  }

  pub fn time(&mut self) {
    let stop_bool = self.is_over.clone();
    let ui = self.ui.clone();

    let handle = thread::spawn(move || { 
      let mut t2: Instant;
      let t1 = Instant::now();
  
      loop {
        t2 = Instant::now();
        
        let result = ui.lock().unwrap().print_time(&(t2 - t1).as_secs_f64());
        match result {
          Ok(_) => (),
          Err(err) => {
            panic!("Ошибка при печати времени: {}", err)
          }
        };

        if stop_bool.load(Ordering::Relaxed) {
          break;
        }
        else {
          sleep(Duration::from_millis(100));
        }
      }
  
    });

    self.running_thread_handles.push(handle);
  }

  pub fn snake_update(&mut self) {
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


    let handle = thread::spawn(move || {
      let result = ui
        .lock()
        .unwrap()
        .print_snake(&snake.lock().unwrap());
      match result {
        Ok(_) => (),
        Err(err) => {
          panic!("Ошибка при печати змейки: {}", err)
        }
      }

      loop {
        snake.lock().unwrap().set_direction(dir.load(Ordering::Relaxed));
        let last_pos = snake
                                    .lock()
                                    .unwrap()
                                    .update(field_size);

        let result = ui
          .lock()
          .unwrap()
          .clear_char(last_pos);
        match result {
          Ok(_) => (),
          Err(err) => {
            panic!("{}", err)
          }
        }

        let result = ui
          .lock()
          .unwrap()
          .print_snake(&snake.lock().unwrap());
        match result {
          Ok(_) => (),
          Err(err) => {
            panic!("Ошибка при печати змейки: {}", err)
          }
        }

        let result = ui
          .lock()
          .unwrap()
          .print_snake(&snake.lock().unwrap());
        match result {
          Ok(_) => (),
          Err(err) => {
            panic!("Ошибка при печати змейки: {}", err)
          }
        }

        if snake.lock().unwrap().check_self_eaten() {
          let result = ui
            .lock()
            .unwrap()
            .print_end_game_message("Сам себя съел!");
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("{}", err)
            }
          }

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
    });

    self.running_thread_handles.push(handle);
  }

  pub fn fetch_key(&mut self) {
    let stop_bool = self.is_over.clone();
    let ui = self.ui.clone();
    let dir = self.dir.clone();

    let handle = thread::spawn(move || {
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
          let result = ui
            .lock()
            .unwrap()
            .print_end_game_message("Прерывание...");
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("{}", err)
            }
          }

          stop_bool.store(true, Ordering::Relaxed);
          break;
        }
      }
    });

    self.running_thread_handles.push(handle);
  }

  pub fn food_generator(&mut self) {
    let stop_bool = self.is_over.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = Arc::new(self.field_size);
    let score = self.score.clone();

    let handle = thread::spawn(move || {
      let mut food = Food::generate_food(&field_size, true);
      let mut brick = Food::generate_food(&field_size, false);

      let result = ui.lock().unwrap().print_food(&food);
      match result {
        Ok(_) => (),
        Err(err) => {
          panic!("Ошибка при печати еды: {}", err)
        }
      }

      let result = ui.lock().unwrap().print_food(&brick);
      match result {
        Ok(_) => (),
        Err(err) => {
          panic!("Ошибка при печати кирпича: {}", err)
        }
      }

      loop {
        if snake.lock().unwrap().check_pos(food.get_pos()) {
          score.fetch_add(food.get_value(), Ordering::SeqCst);

          let result = ui.lock().unwrap().print_stats(
            &score.load(Ordering::Relaxed),
            &(snake.lock().unwrap().get_parts().len() as u16)
          );
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("Ошибка при печати статистики: {}", err)
            }
          }

          let pos = food.get_pos();
          snake.lock().unwrap().add_part(pos);
          
          loop {
            food = Food::generate_food(&field_size, true);

            if !snake.lock().unwrap().check_pos(food.get_pos()) {
              break;
            }
          }

          let result = ui.lock().unwrap().print_food(&food);
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("Ошибка при печати еды: {}", err)
            }
          }

          let result = ui.lock().unwrap().clear_char(brick.get_pos());
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("{}", err)
            }
          }

          loop {
            brick = Food::generate_food(&field_size, false);

            if !snake.lock().unwrap().check_pos(brick.get_pos()) {
              break;
            }
          }

          let result = ui.lock().unwrap().print_food(&brick);
          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("Ошибка при печати кирпича: {}", err)
            }
          }
        }

        if snake.lock().unwrap().check_pos(brick.get_pos()) {
          let result = ui
            .lock()
            .unwrap()
            .print_end_game_message("Съел кирпич!");

          match result {
            Ok(_) => (),
            Err(err) => {
              panic!("{}", err)
            }
          }

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
    });

    self.running_thread_handles.push(handle);
  }

  pub fn terminal_size_checker(&mut self) {
    let stop_bool = self.is_over.clone();
    let terminal_size = self.terminal_size.clone();

    let handle = thread::spawn(move || {
      loop {     
        if terminal_size != terminal::size().unwrap() {
          stop_bool.store(true, Ordering::Relaxed);
          break;
        }
        else {
          sleep(Duration::from_millis(300));
        }
      }
    });

    self.running_thread_handles.push(handle);
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