use super::{camera, GameObject, physics::collision::Collision};

pub struct GameState {
  pub camera: camera::Camera,
  pub game_objects: Vec<GameObject>,
  pub collision: Collision
}

impl GameState {
  pub fn new() -> Self {
    Self {
      camera: camera::Camera::new((-10.0, 5.0, 10.0), cgmath::Deg(-45.0), cgmath::Deg(-20.0)),
      game_objects: vec![],
      collision: Collision::new(),
    }
  }
}
