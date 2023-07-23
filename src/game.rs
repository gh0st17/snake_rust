mod game_action;

use game_action::{KeyAction, KeyController};

use crate::snake::{
  Snake, Direction
};

use crate::food::{
  Food, generate_food
};

use crate::ui::{
  UI,
  dimensions::Size,
  ui_items::Symbol
};

use std::{
  collections::VecDeque,
  io::Result,
  thread::{sleep, self},
  time::Duration,
  sync::{
    Arc, Mutex, Barrier,
    atomic::{AtomicBool, Ordering}
  }
};

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
      ui: Arc::new(Mutex::new(ui)),
      terminal_size: Size::from(terminal::size().unwrap())
    }
  }

  pub fn run(&mut self) {
    let threads = vec![
      Self::time_update,
      Self::snake_update,
      Self::collision_update,
      Self::fetch_event,
      Self::terminal_size_checker
    ];

    for thread in threads {
      let shared_self = self.clone();

      let _ = thread::spawn(move || -> Result<()> {
        let local_self = &mut shared_self.clone();

        thread(local_self)
      });
    }

    self.barrier.wait();
  }

  fn time_update(&mut self) -> Result<()> {
    let (mut time, delay) = (0.0f64, 0.1f64);

    loop {
      if !self.pause.load(Ordering::Acquire) {
        self.ui.lock().unwrap().print_time(&time)?;
        time += delay;
      }

      if self.stop_bool.load(Ordering::Acquire) {
        break;
      }
      else {
        sleep(Duration::from_millis(100));
      }
    }

    self.barrier.wait();
    Ok(())
  }

  fn snake_update(&mut self) -> Result<()> {
    let mut _boost = false;

    loop {
      while self.pause.load(Ordering::Acquire) {
        sleep(Duration::from_millis(50));
      }
        
      if self.boost.load(Ordering::Acquire) {
        sleep(Duration::from_millis(130));
      }
      else {
        sleep(Duration::from_millis(200));
      }

      if let Some(dir) = self.sequence.lock().unwrap().pop_front() {
        self.snake.lock().unwrap().set_direction(dir)
      }

      if !self.stop_bool.load(Ordering::Acquire) {
        self.snake
          .lock().unwrap()
          .update(self.ui.clone())?;
        self.ui.lock().unwrap().draw::<Snake>(
          &self.snake
            .lock().unwrap()
        )?;
      }
      else {
        break;
      }
    }

    self.barrier.wait();
    Ok(())
  }

  fn collision_update(&mut self) -> Result<()> {
    let mut apple;    
    let mut bricks = Vec::new();
    
    {
      let head_pos = self.snake.lock().unwrap().get_head_pos();
      let density = self.field_size.width as u64 * 
      self.field_size.height as u64 / 100;

      for _ in 0..density {
        bricks.push(generate_food(
          &self.field_size, false, &head_pos
        ));
      }

      apple = generate_food(
        &self.field_size, true, &head_pos
      );

      self.ui.lock().unwrap().print_stats(&self.score, &0)?;
      self.ui.lock().unwrap().draw::<Snake>(&self.snake.lock().unwrap())?;
      self.ui.lock().unwrap().draw(&apple)?;
      self.ui.lock().unwrap().draw_vec(&bricks)?;
    }

    loop {
      while self.pause.load(Ordering::Acquire) {
        sleep(Duration::from_millis(50));
      }

      self.collision_check(&mut apple, &mut bricks)?;
      if self.stop_bool.load(Ordering::Acquire) {
        break;
      }
      else {
        sleep(Duration::from_millis(50));
      }
    }

    self.barrier.wait();
    Ok(())
  }

  fn collision_check(&mut self, apple: &mut Box<dyn Food>,
      bricks: &mut Vec<Box<dyn Food>>) -> Result<()> {

    if self.snake.lock().unwrap().check_self_eaten() {
      self.ui.lock().unwrap()
        .print_popup_message("Сам себя съел!")?;

      self.stop_bool.store(true, Ordering::Release);
      sleep(Duration::from_secs(3));
    }

    if self.snake.lock().unwrap().check_pos(&apple.get_pos()) {
      self.food_update(apple, bricks)?;
    }

    for brick in bricks {
      if self.snake.lock().unwrap().check_pos(&brick.get_pos()) {
        self.ui.lock().unwrap()
          .print_popup_message("Съел кирпич!")?;

        self.stop_bool.store(true, Ordering::Release);
        sleep(Duration::from_secs(3));
      }
    }

    Ok(())
  }

  fn boost_mode_toggle(&mut self) {
    let boost = self.boost.load(Ordering::Acquire);
    let color = if !boost { Cyan } else { Green };
    self.snake.lock().unwrap().set_head_color(color);
    self.boost.store(!boost, Ordering::Release);
  }

  fn pause_mode_toggle(&mut self) -> Result<()>{
    let pause = self.pause.load(Ordering::Acquire);
    if !pause {
      self.ui.lock().unwrap().print_popup_message("Пауза")?;
    }
    else {
      self.ui.lock().unwrap().clear_popup_message()?;
    }
    self.pause.store(!pause, Ordering::Release);

    Ok(())
  }

  fn fetch_event(&mut self) -> Result<()> {
    let key_controller = KeyController::new();
    loop {
      let action = key_controller.fetch_action()?;

      if !self.pause.load(Ordering::Acquire) {
        let sequence = self.sequence.clone();
        let mut sequence = sequence.lock().unwrap();

        match action {
          KeyAction::None | KeyAction::Pause => (),
          KeyAction::MoveUp    => sequence.push_back(Direction::Up),
          KeyAction::MoveDown  => sequence.push_back(Direction::Down),
          KeyAction::MoveLeft  => sequence.push_back(Direction::Left),
          KeyAction::MoveRight => sequence.push_back(Direction::Right),
          KeyAction::Boost     => self.boost_mode_toggle(),
          KeyAction::Exit => {
            self.ui.lock().unwrap()
            .print_popup_message("Прерывание...")?;

            self.stop_bool.store(true, Ordering::Release);
            break;
          }
        }
      }

      if let KeyAction::Pause = action {
        self.pause_mode_toggle().unwrap();
      }
    }

    Ok(())
  }

  fn food_update(&mut self, apple: &mut Box<dyn Food>,
      bricks: &mut Vec<Box<dyn Food>>) -> Result<()> {
    
    let mut ui = self.ui.lock().unwrap();

    self.score += apple.get_value();

    ui.print_stats(
      &self.score,
      &(self.snake
        .lock().unwrap()
        .get_parts().len() as u16)
    )?;

    self.snake
      .lock().unwrap()
      .add_part();
    
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
    }

    ui.draw_vec(&bricks)?;
    ui.draw(apple)?;

    Ok(())
  }

  fn terminal_size_checker(&mut self) -> Result<()> {
    let terminal_size = self.terminal_size.clone();

    loop {
      if terminal_size != Size::from(terminal::size()?) {
        self.stop_bool.store(true, Ordering::Release);
        break;
      }
      else {
        sleep(Duration::from_millis(200));
      }
    }

    Ok(())
  }
}

impl Drop for Game {
  fn drop(&mut self) {
    self.ui.lock().unwrap().disable_raw_mode();
  }
}