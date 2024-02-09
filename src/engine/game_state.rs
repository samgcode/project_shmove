use super::{camera, GameObject};

pub struct GameState {
  pub camera: camera::Camera,
  pub game_objects: Vec<GameObject>
}

impl GameState {
  pub fn new() -> Self {
    Self {
      camera: camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0)),
      game_objects: vec![],
    }
  }
}
