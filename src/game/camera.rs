use cgmath::{InnerSpace, Rad, Vector2, Vector3, Zero};
use std::f32::consts::FRAC_PI_2;

use project_shmove::engine::Camera;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

pub struct CameraController {
  sensitivity: f32,
  position: Vector3<f32>,
  pub forward: Vector3<f32>,
  pub right: Vector3<f32>,
}

impl CameraController {
  pub fn new(sensitivity: f32) -> Self {
    Self {
      sensitivity,
      position: Vector3::zero(),
      forward: Vector3::zero(),
      right: Vector3::zero(),
    }
  }

  pub fn update(&mut self, camera: &mut Camera, mouse_speed: Vector2<f32>, dt: f32) {
    camera.position.x = self.position.x;
    camera.position.z = self.position.z;
    camera.position.y = self.position.y;

    camera.yaw += Rad(mouse_speed.x) * self.sensitivity * dt;
    camera.pitch += Rad(-mouse_speed.y) * self.sensitivity * dt;

    if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
      camera.pitch = -Rad(SAFE_FRAC_PI_2);
    } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
      camera.pitch = Rad(SAFE_FRAC_PI_2);
    }

    let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
    self.forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
    self.right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
  }

  pub fn set_pos(&mut self, pos: Vector3<f32>) {
    self.position = pos;
  }
}
