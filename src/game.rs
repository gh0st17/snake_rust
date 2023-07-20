mod game_action;

use game_action::*;

use crate::snake::{
  Snake, Direction
};

use crate::food::{
  Food, generate_food
};

use crate::ui::{
  UI,
  dimensions::{Pos, Size},
  ui_items::Symbol
};

use std::collections::VecDeque;
use std::{
  io::Result,
  thread::{sleep, self},
  time::Duration,
  sync::{
    Arc, Mutex, Barrier,
    atomic::{AtomicBool, Ordering}
  }
};

use atomic::Atomic;

use crossterm::{
  terminal,
  style::Color::*
};

#[derive(Clone)]
pub struct Game {
  barrier: Arc<Barrier>,
  stop_bool: Arc<AtomicBool>,
  pause: Arc<AtomicBool>,
  boost: Arc<AtomicBool>,
  score: u16,
  ui: Arc<Mutex<UI>>,
  snake: Arc<Mutex<Snake>>,
  sequence: Arc<Mutex<VecDeque<Direction>>>,
  last_pos: Arc<Atomic<Pos>>,
  field_size: Size,
  terminal_size: Size
}

impl Game {
  pub fn new(ui: UI) -> Self {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let dir = match rng.gen_range(0..4) {
      0 => Direction::Up,
      1 => Direction::Down,
      2 => Direction::Left,
      _ => Direction::Right,
    };

    Game {
      barrier: Arc::new(Barrier::new(4)),
      stop_bool: Arc::new(AtomicBool::new(false)),
      pause: Arc::new(AtomicBool::new(false)),
      boost: Arc::new(AtomicBool::new(false)),
      score: 0,
      field_size: ui.field_size,
      snake: Arc::new(Mutex::new(Snake::new(ui.field_size, dir))),
      sequence: Arc::new(Mutex::new(VecDeque::new())),
      last_pos: Arc::new(Atomic::new(Pos::from((0, 0)))),
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: Size::from(terminal::size().unwrap())
    }
  }

  pub fn run(&mut self) {
    let threads = vec![
      self.time_update(),
      self.snake_update(),
      self.collision_update(),
      self.fetch_event(),
      self.terminal_size_checker()
    ];

    for thread in threads {
      match thread {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
      }
    }

    self.barrier.wait();
    self.ui.lock().unwrap().disable_raw_mode();
  }

  fn time_update(&mut self) -> Result<()> {
    let barrier = self.barrier.clone();
    let stop_bool = self.stop_bool.clone();
    let pause = self.pause.clone();
    let ui = self.ui.clone();

    let _ = thread::spawn(move || -> Result<()> { 
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

      barrier.wait();
      Ok(())  
    });

    Ok(())
  }

  fn snake_update(&mut self) -> Result<()> {
    let stop_bool = self.stop_bool.clone();
    let barrier = self.barrier.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = self.field_size;
    let last_pos = self.last_pos.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let mut _boost = false;
    let sequence = self.sequence.clone();

    let _ = thread::spawn(move || -> Result<()> {
      loop {
        if pause.load(Ordering::Acquire) {
          sleep(Duration::from_millis(100));
          continue;
        }
          
        if boost.load(Ordering::Acquire) {
          sleep(Duration::from_millis(130));

          if !_boost {
            _boost = true;
            snake.lock().unwrap().set_head_color(Cyan);
          }
        }
        else {
          sleep(Duration::from_millis(200));

          if _boost {
            _boost = false;
            snake.lock().unwrap().set_head_color(Green);
          }
        }

        match sequence.lock().unwrap().pop_front() {
          Some(dir) => snake.lock().unwrap().set_direction(dir),
          None => ()
        };

        last_pos.store(
          snake
            .lock().unwrap()
            .update(field_size),
          Ordering::Release
        );
        ui.lock().unwrap().draw(
          &Symbol::new(
            last_pos.load(Ordering::Acquire)
          )
        )?;
        ui.lock().unwrap().draw::<Snake>(
          &snake
            .lock().unwrap()
        )?;

        if stop_bool.load(Ordering::Acquire) {
          break;
        }
      }

      barrier.wait();
      Ok(())
    });

    Ok(())
  }

  fn collision_update(&mut self) -> Result<()> {
    let barrier = self.barrier.clone();
    let stop_bool = self.stop_bool.clone();
    let snake = self.snake.clone();
    let ui = self.ui.clone();
    let field_size = self.field_size;
    
    let s_length = snake.lock().unwrap().get_parts().len() as u16 - 1;

    self.ui
      .lock()
      .unwrap()
      .print_stats(&self.score, &s_length)?;

    let mut shared_self = self.clone();

    let _ = thread::spawn(move || -> Result<()> {
      let mut apple = generate_food(
        &field_size, true, &snake.lock().unwrap().get_head_pos()
      );
      
      let mut bricks: Vec<Box<dyn Food>> = Vec::new();
      let density = field_size.width as u64 * 
        field_size.height as u64 / 100;

      for _ in 0..density {
        bricks.push(generate_food(
          &field_size, false, &snake.lock().unwrap().get_head_pos()
        ));
      }

      let local_self = &mut shared_self;

      ui.lock().unwrap().draw(&apple)?;
      ui.lock().unwrap().draw::<Snake>(&snake.lock().unwrap())?;
      ui.lock().unwrap().draw_vec(&bricks)?;

      loop {
        local_self.collision_check(&mut apple, &mut bricks)?;
        if stop_bool.load(Ordering::Acquire) {
          break;
        }
        else {
          sleep(Duration::from_millis(50));
        }
      }

      barrier.wait();
      Ok(())
    });

    Ok(())
  }

  fn collision_check(&mut self, apple: &mut Box<dyn Food>,
      bricks: &mut Vec<Box<dyn Food>>) -> Result<()> {

    if self.snake.lock().unwrap().check_self_eaten() {
      self.ui.lock().unwrap()
        .print_popup_message("Сам себя съел!".to_string(), true)?;

      self.stop_bool.store(true, Ordering::Release);
    }

    if self.snake.lock().unwrap().check_pos(&apple.get_pos()) {
      self.food_update(apple, bricks)?;
    }

    for brick in bricks {
      if self.snake.lock().unwrap().check_pos(&brick.get_pos()) {
        self.ui.lock().unwrap()
          .print_popup_message(
            "Съел кирпич!".to_string(), true
          )?;

        self.stop_bool.store(true, Ordering::Release);
      }
    }

    Ok(())
  }

  fn fetch_event(&mut self) -> Result<()> {
    let stop_bool = self.stop_bool.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let ui = self.ui.clone();
    let key_controller = KeyController::new();
    let sequence = self.sequence.clone();

    let _ = thread::spawn(move || -> Result<()> {
      loop {
        let action = key_controller.fetch_action()?;

        if !pause.load(Ordering::Acquire) {
          match action {
            KeyAction::None => (),
            KeyAction::MoveUp    => sequence.lock().unwrap().push_back(Direction::Up),
            KeyAction::MoveDown  => sequence.lock().unwrap().push_back(Direction::Down),
            KeyAction::MoveLeft  => sequence.lock().unwrap().push_back(Direction::Left),
            KeyAction::MoveRight => sequence.lock().unwrap().push_back(Direction::Right),
            KeyAction::Boost => {
              let _boost = boost.load(Ordering::Acquire);
              boost.store(!_boost, Ordering::Release);
            }
            KeyAction::Pause => (),
            KeyAction::Exit => {
              ui.lock().unwrap()
              .print_popup_message(
                "Прерывание...".to_string(), true
              )?;

              stop_bool.store(true, Ordering::Release);
              break;
            }
          }
        }

        if let KeyAction::Pause = action {
          let _pause = pause.load(Ordering::Acquire);
          pause.store(!_pause, Ordering::Release);
          let _ui = ui.lock().unwrap();

          if !_pause {
            _ui.print_popup_message(
              "Пауза".to_string(), false
            )?;
          }
          else {
            _ui.clear_popup_message()?;
          }
        }
      }

      Ok(())
    });

    Ok(())
  }

  fn food_update(&mut self, apple: &mut Box<dyn Food>,
      bricks: &mut Vec<Box<dyn Food>>) -> Result<()> {
    
    let mut ui = self.ui.lock().unwrap();

    self.score += apple.get_value();

    ui.print_stats(
      &self.score,
      &(
        self.snake
          .lock().unwrap()
          .get_parts().len() as u16
      )
    )?;

    self.snake.lock().unwrap().add_part(self.last_pos.load(Ordering::Acquire));
    
    let snake_pos = self.snake
      .lock().unwrap()
      .get_head_pos();

    loop {
      *apple = generate_food(
        &self.field_size, true, &snake_pos
      );

      if !self.snake
        .lock().unwrap()
        .check_pos(&apple.get_pos()) {
        
        break;
      }
    }

    for i in 0..bricks.len() {
      ui.draw(&Symbol::new(bricks[i].get_pos()))?;

      'same:
      loop {
        bricks[i] = generate_food(
          &self.field_size, false, &snake_pos
        );

        if self.snake
          .lock().unwrap()
          .check_pos(&bricks[i].get_pos()) ||
            apple.get_pos() == bricks[i].get_pos() {
          
          continue;
        }
        
        for j in 0..bricks.len() {
          if i != j && bricks[i].get_pos() == bricks[j].get_pos() {
            continue 'same;
          }
        }
        break;
      }

      ui.draw(&bricks[i])?;
      ui.draw(apple)?;
    }

    Ok(())
  }

  fn terminal_size_checker(&mut self) -> Result<()> {
    let stop_bool = self.stop_bool.clone();
    let terminal_size = self.terminal_size.clone();

    let _ = thread::spawn(move || -> Result<()> {
      loop {
        if terminal_size != Size::from(terminal::size()?) {
          stop_bool.store(true, Ordering::Release);
          break;
        }
        else {
          sleep(Duration::from_millis(200));
        }
      }

      Ok(())
    });

    Ok(())
  }
}