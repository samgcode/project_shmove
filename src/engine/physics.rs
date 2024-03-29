
pub mod input;
pub mod game_object;
pub mod collision;

pub struct State {
  pub input: input::Input,
}

impl State {
  pub fn new() -> Self {
    Self {
      input: input::Input::new()
    }
  }
}
