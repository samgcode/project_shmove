use std::collections::HashMap;

use cgmath::Vector2;
use winit::{
  dpi::PhysicalPosition,
  event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
    WindowEvent,
  },
};

pub struct Input {
  key_states: HashMap<VirtualKeyCode, bool>,
  mouse_states: HashMap<MouseButton, bool>,
  mouse_position: Vector2<f32>,
  scroll_speed: f32,
}

impl Input {
  pub fn new() -> Self {
    Self {
      key_states: HashMap::new(),
      mouse_states: HashMap::new(),
      mouse_position: Vector2 { x: 0.0, y: 0.0 },
      scroll_speed: 0.0,
    }
  }

  fn keyboard_event(&mut self, key: VirtualKeyCode, state: bool) {
    self.key_states.insert(key, state);
  }

  fn mouse_event(&mut self, button: MouseButton, state: bool) {
    self.mouse_states.insert(button, state);
  }

  fn mouse_moved(&mut self, mouse_dx: f64, mouse_dy: f64) {
    self.mouse_position.x = mouse_dx as f32;
    self.mouse_position.y = mouse_dy as f32;
  }
  fn process_scroll(&mut self, delta: f32) {
    self.scroll_speed = delta;
  }

  pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
    match self.key_states.get(&key) {
      Some(state) => *state,
      None => false,
    }
  }

  pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
    match self.mouse_states.get(&button) {
      Some(state) => *state,
      None => false,
    }
  }

  pub fn get_mouse_position(&self) -> Vector2<f32> {
    Vector2 { x: self.mouse_position.x, y: self.mouse_position.y }
  }

  pub fn handle_event(&mut self, event: &Event<'_, ()>) {
    match event {
      Event::DeviceEvent {
        event: DeviceEvent::MouseMotion { delta },
        ..
      } => self.mouse_moved(delta.0, delta.1),
      Event::WindowEvent {
        ref event,
        window_id: _,
      } => match event {
        WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              virtual_keycode: Some(key),
              state,
              ..
            },
          ..
        } => self.keyboard_event(
          *key,
          match state {
            ElementState::Pressed => true,
            _ => false,
          },
        ),
        WindowEvent::MouseWheel { delta, .. } => {
          let scroll = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
          };
          self.process_scroll(scroll);
        }
        WindowEvent::MouseInput { button, state, .. } => self.mouse_event(
          *button,
          match state {
            ElementState::Pressed => true,
            _ => false,
          },
        ),
        _ => {}
      },
      _ => {}
    };
  }
}
