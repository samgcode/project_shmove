use cgmath::{Rad, Vector2, Vector3, InnerSpace};
use std::f32::consts::FRAC_PI_2;

use project_shmove::engine::Camera;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

pub struct CameraController {
  pub sensitivity: f32,
}

impl CameraController {
  pub fn update(&self, camera: &mut Camera, mouse_speed: Vector2<f32>, movement_speed: Vector3<f32>, dt: f32) {
    let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
    let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();

    camera.position += forward * (movement_speed.x) * dt;
    camera.position += right * (movement_speed.y) * dt;
    camera.position.y += movement_speed.z * dt;

    camera.yaw += Rad(mouse_speed.x) * self.sensitivity * dt;
    camera.pitch += Rad(-mouse_speed.y) * self.sensitivity * dt;

    if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
      camera.pitch = -Rad(SAFE_FRAC_PI_2);
    } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
      camera.pitch = Rad(SAFE_FRAC_PI_2);
    }
  }
}
