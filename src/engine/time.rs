pub struct Time {
  start_time: instant::Instant,
  prev_time: instant::Instant,
  pub delta_time: f32,
  pub elapsed_time: f32,
}

impl Time {
  pub fn create() -> Self {
    Self {
      start_time: instant::Instant::now(),
      prev_time: instant::Instant::now(),
      delta_time: 0.0,
      elapsed_time: 0.0,
    }
  }

  pub fn update(&mut self) {
    let now = instant::Instant::now();
    self.delta_time = (now - self.prev_time).as_secs_f32();
    self.elapsed_time = (now - self.start_time).as_secs_f32();
    self.prev_time = now;
  }
}
