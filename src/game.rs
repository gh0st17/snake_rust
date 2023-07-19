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
  is_over: Arc<AtomicBool>,
  pause: Arc<AtomicBool>,
  boost: Arc<AtomicBool>,
  score: u16,
  dir: Arc<Atomic<Direction>>,
  ui: Arc<Mutex<UI>>,
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
      3 => Direction::Right,
      _ => Direction::Up,
    };

    Game {
      barrier: Arc::new(Barrier::new(3)),
      is_over: Arc::new(AtomicBool::new(false)),
      pause: Arc::new(AtomicBool::new(false)),
      boost: Arc::new(AtomicBool::new(false)),
      score: 0,
      dir: Arc::new(Atomic::new(dir)),
      field_size: ui.field_size,
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: Size::from(terminal::size().unwrap())
    }
  }

  pub fn run(&mut self) {
    let threads = vec![
      self.time_update(),
      self.snake_update(),
      self.fetch_key(),
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
    let stop_bool = self.is_over.clone();
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
    let barrier = self.barrier.clone();
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let dir = self.dir.clone();
    let mut snake = Snake::new(
      self.field_size, dir.load(Ordering::Acquire)
    );
    let ui = self.ui.clone();
    let field_size = self.field_size;
    
    let s_length = snake.get_parts().len() as u16 - 1;

    self.ui
      .lock()
      .unwrap()
      .print_stats(&self.score, &s_length)?;

    let mut shared_self = self.clone();

    let _ = thread::spawn(move || -> Result<()> {
      let mut _boost = boost.load(Ordering::Acquire);

      let mut apple = generate_food(
        &field_size, true, &snake.get_head_pos()
      );
      
      let mut bricks: Vec<Box<dyn Food>> = Vec::new();
      let density = field_size.width as u64 * 
        field_size.height as u64 / 100;

      for _ in 0..density {
        bricks.push(generate_food(
          &field_size, false, &snake.get_head_pos()
        ));
      }

      let local_self = &mut shared_self;

      ui.lock().unwrap().draw(&apple)?;
      ui.lock().unwrap().draw(&snake)?;
      ui.lock().unwrap().draw_vec(&bricks)?;

      'stop:
      loop {
        if pause.load(Ordering::Acquire) {
          sleep(Duration::from_millis(100));
          continue;
        }

        snake.set_direction(dir.load(Ordering::Acquire));
        let last_pos = snake.update(field_size);

        ui.lock().unwrap().draw(&Symbol::new(last_pos))?;
        ui.lock().unwrap().draw(&snake)?;

        if snake.check_self_eaten() {
          ui.lock().unwrap()
            .print_popup_message("Сам себя съел!".to_string(), true)?;

          stop_bool.store(true, Ordering::Release);
          break;
        }

        if snake.check_pos(&apple.get_pos()) {
          local_self.food_update(
            &mut apple, &mut bricks, &mut snake, &last_pos
          )?;
        }

        for brick in &bricks {
          if snake.check_pos(&brick.get_pos()) {
            ui.lock().unwrap()
              .print_popup_message(
                "Съел кирпич!".to_string(), true
              )?;

            stop_bool.store(true, Ordering::Release);
            break 'stop;
          }
        }

        if stop_bool.load(Ordering::Acquire) {
          break;
        }
        else {
          if boost.load(Ordering::Acquire) {
            sleep(Duration::from_millis(130));

            if !_boost {
              _boost = true;
              snake.set_head_color(Cyan);
            }
          }
          else {
            sleep(Duration::from_millis(200));

            if _boost {
              _boost = false;
              snake.set_head_color(Green);
            }
          }
        }
      }

      barrier.wait();
      Ok(())
    });

    Ok(())
  }

  fn fetch_key(&mut self) -> Result<()> {
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let ui = self.ui.clone();
    let dir = self.dir.clone();
    let key_controller = KeyController::new();

    let _ = thread::spawn(move || -> Result<()> {
      loop {
        let action = key_controller.fetch_action()?;

        if !pause.load(Ordering::Acquire) {
          match action {
            KeyAction::None => (),
            KeyAction::MoveUp => dir.store(Direction::Up, Ordering::Release),
            KeyAction::MoveDown => dir.store(Direction::Down, Ordering::Release),
            KeyAction::MoveLeft => dir.store(Direction::Left, Ordering::Release),
            KeyAction::MoveRight => dir.store(Direction::Right, Ordering::Release),
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
            _ui.print_static()?;
          }
        }
      }

      Ok(())
    });

    Ok(())
  }

  fn food_update(&mut self, apple: &mut Box<dyn Food>,
      bricks: &mut Vec<Box<dyn Food>>,
      snake: &mut Snake, last_pos: &Pos) -> Result<()> {
    
    let mut ui = self.ui.lock().unwrap();

    self.score += apple.get_value();

    ui.print_stats(
      &self.score,
      &(snake.get_parts().len() as u16)
    )?;

    snake.add_part(*last_pos);
    
    let snake_pos = snake.get_head_pos();
    loop {
      *apple = generate_food(
        &self.field_size, true, &snake_pos
      );

      if !snake.check_pos(&apple.get_pos()) {
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

        if snake.check_pos(&bricks[i].get_pos()) ||
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
    let stop_bool = self.is_over.clone();
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