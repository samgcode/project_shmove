use super::{camera, physics::collision::Collision, render::color::Color, GameObject};

pub struct GameState {
  pub camera: camera::Camera,
  pub game_objects: Vec<GameObject>,
  pub collision: Collision,
  pub background_color: Color,
}

impl GameState {
  pub fn new() -> Self {
    Self {
      camera: camera::Camera::new((-10.0, 5.0, 10.0), cgmath::Deg(90.0), cgmath::Deg(0.0)),
      game_objects: vec![],
      collision: Collision::new(),
      background_color: Color::from_rgb(1.0, 0.0, 0.0),
    }
  }
}
