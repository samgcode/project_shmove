use cgmath::Vector2;

use crate::engine::render::color::Color;

pub const UNBOUNDED_F32: f32 = std::f32::INFINITY;

pub struct TextObject {
  pub centered: bool,
  pub enabled: bool,
  pub size: f32,
  pub color: Color,
  pub text: String,
  pub position: Vector2<f32>,
  pub bounds: Vector2<f32>,
}

impl Default for TextObject {
  fn default() -> Self {
    Self {
      centered: false,
      enabled: true,
      size: 16.0,
      color: Color::from_rgb(1.0, 0.0, 0.0),
      text: String::from("default"),
      position: (0.0, 0.0).into(),
      bounds: (UNBOUNDED_F32, UNBOUNDED_F32).into(),
    }
  }
}
