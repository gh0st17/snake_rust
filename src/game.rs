use crate::snake::{Snake, Direction};
use crate::food::{Food, generate_food};
use crate::ui::{UI, Pos, ui_items::Symbol};

use std::thread::JoinHandle;
use std::{
  io::Result,
  thread::{sleep, self},
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
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: terminal::size().unwrap()
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

    while !self.is_over() {
      sleep(Duration::from_secs(1));
    }
  }

  fn is_over(&self) -> bool {
    self.is_over.load(Ordering::Acquire)
  }

  fn time_update(&mut self) -> Result<JoinHandle<Result<()>>> {
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

  fn snake_update(&mut self) -> Result<JoinHandle<Result<()>>> {
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let dir = self.dir.clone();
    let mut snake = Snake::new();
    let ui = self.ui.clone();
    let field_size = self.field_size;
    
    let s_length = snake.get_parts().len() as u16 - 1;

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
      let mut snake_pos = snake.get_head_pos();
      let mut _boost = boost.load(Ordering::Acquire);
      let mut food = generate_food(&field_size, true, (0,0));
      
      let mut bricks: Vec<Box<dyn Food>> = Vec::new();
      let density = field_size.0 as u64 * field_size.1 as u64 / 100;

      for _ in 0..density {
        bricks.push(generate_food(&field_size, false, (0,0)));
      }

      let local_self = &mut shared_self;

      ui.lock().unwrap().draw(&food)?;
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

        if snake.check_pos(&food.get_pos()) {
          local_self.food_generator(
            &mut food, &mut bricks,
            &mut snake_pos, &mut snake
          )?;
        }

        for brick in &bricks {
          if snake.check_pos(&brick.get_pos()) {
            ui.lock().unwrap()
              .print_popup_message("Съел кирпич!".to_string(), true)?;

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

      Ok(())
    });

    Ok(handle)
  }

  fn fetch_key(&mut self) -> Result<JoinHandle<Result<()>>>{
    let stop_bool = self.is_over.clone();
    let pause = self.pause.clone();
    let boost = self.boost.clone();
    let ui = self.ui.clone();
    let dir = self.dir.clone();

    let handle = thread::spawn(move || -> Result<()> {
      loop {
        let event = read()?;

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
          }
        }

        if event == Event::Key(KeyCode::Char('p').into()) {
          let _pause = pause.load(Ordering::Acquire);
          pause.store(!_pause, Ordering::Release);
          let _ui = ui.lock().unwrap();

          if !_pause {
            _ui.print_popup_message("Пауза".to_string(), false)?;
          }
          else {
            _ui.print_static()?;
          }
        }

        if event == Event::Key(KeyCode::Esc.into()) {
          ui.lock().unwrap()
            .print_popup_message("Прерывание...".to_string(), true)?;

          stop_bool.store(true, Ordering::Release);
          break;
        }
      }

      Ok(())
    });
    
    Ok(handle)
  }

  fn food_generator(&mut self, food: &mut Box<dyn Food>, bricks: &mut Vec<Box<dyn Food>>,
      snake_pos: &mut Pos, snake: &mut Snake) -> Result<()> {
    
    self.score += food.get_value();

    self.ui.lock().unwrap().print_stats(
      &self.score,
      &(snake.get_parts().len() as u16)
    )?;

    let pos = food.get_pos();
    snake.add_part(pos);
    
    *snake_pos = snake.get_head_pos();
    loop {
      *food = generate_food(&self.field_size, true, *snake_pos);

      if !snake.check_pos(&food.get_pos()) {
        break;
      }
    }

    self.ui.lock().unwrap().draw(food)?;

    for i in 0..bricks.len() {
      self.ui.lock().unwrap().draw(&Symbol::new(bricks[i].get_pos()))?;

      'same:
      loop {
        bricks[i] = generate_food(&self.field_size, false, *snake_pos);

        if snake.check_pos(&bricks[i].get_pos()) ||
            food.get_pos() == bricks[i].get_pos() {
          continue;
        }
        
        for j in 0..bricks.len() {
          if i != j && bricks[i].get_pos() == bricks[j].get_pos() {
            continue 'same;
          }
        }
        break;
      }

      self.ui.lock().unwrap().draw(&bricks[i])?;
    }


    Ok(())
  }

  fn terminal_size_checker(&mut self) -> Result<JoinHandle<Result<()>>> {
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
}