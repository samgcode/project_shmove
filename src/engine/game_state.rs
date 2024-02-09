use super::{camera, physics::game_object};

pub struct GameState {
  pub camera: camera::Camera,
  pub transforms: Vec<game_object::Transform>
}

impl GameState {
  pub fn new() -> Self {
    Self {
      camera: camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0)),
      transforms: vec![],
    }
  }
}
