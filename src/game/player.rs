use cgmath::{Vector3, Zero};
use project_shmove::engine::{
  physics::{
    collision::{EventStatus, Tag},
    input::Input,
  },
  GameObject, GameState,
};
use winit::event::VirtualKeyCode;

use super::camera::CameraController;

pub struct Controller {
  gravity: f32,
  speed: f32,
  jump_height: f32,
  pub game_object: GameObject,
  velocity: Vector3<f32>,
  grounded: bool,
}

impl Controller {
  pub fn new() -> Self {
    Self {
      gravity: 1.0,
      speed: 10.0,
      jump_height: 25.0,
      game_object: GameObject::new(
        (0.0, 4.0, 3.5),
        (0.0, 0.0, 0.0),
        (1.0, 2.0, 1.0),
        [1.0, 0.0, 0.0],
        Tag::Player,
      ),
      velocity: Vector3::new(0.0, 0.0, 0.0),
      grounded: false,
    }
  }

  pub fn update(
    &mut self,
    game: &mut GameState,
    input: &Input,
    camera: &CameraController,
    dt: f32,
  ) {
    self.update_position(game, dt);
    self.update_input(input, camera);

    if self.game_object.transform.position.y < -50.0 {
      self.velocity = Vector3::zero();
      self.game_object.transform.position = Vector3::new(0.0, 4.0, 3.5);
    }
  }

  fn update_position(&mut self, game: &mut GameState, dt: f32) {
    let horizontal_vel = Vector3::new(self.velocity.x * dt, 0.0, self.velocity.z * dt);

    self.game_object.transform.position += horizontal_vel;
    game.collision.update_object(&mut self.game_object);
    if let EventStatus::Enter = self.game_object.collision.status {
      if !horizontal_vel.is_zero() {
        self.game_object.transform.position -= horizontal_vel;

        let toi = game.collision.get_toi(
          &mut self.game_object.transform,
          horizontal_vel,
          self.game_object.collision.other_handle,
          0.02,
        );
        self.game_object.transform.position += horizontal_vel * toi;

        let normal_vel = cgmath::InnerSpace::normalize(horizontal_vel);
        let mut parallel_direction = normal_vel
          - self.game_object.collision.normal
            * cgmath::dot(self.game_object.collision.normal, normal_vel);

        if !parallel_direction.is_zero() {
          parallel_direction = cgmath::InnerSpace::normalize(parallel_direction);

          self.game_object.transform.position +=
            parallel_direction * (cgmath::InnerSpace::magnitude(horizontal_vel) * (1.0 - toi));
        }
      }
    }
    game.collision.update_object(&mut self.game_object);
    while let EventStatus::Stay = self.game_object.collision.status {
      self.game_object.transform.position.y += 0.02;
      game.collision.update_object(&mut self.game_object);
    }

    let vertical_vel = Vector3::new(0.0, self.velocity.y * dt, 0.0);
    self.game_object.transform.position += vertical_vel;
    game.collision.update_object(&mut self.game_object);
    if let EventStatus::Enter = self.game_object.collision.status {
      self.game_object.transform.position -= vertical_vel;
      let toi = game.collision.get_toi(
        &mut self.game_object.transform,
        vertical_vel,
        self.game_object.collision.other_handle,
        0.02,
      );
      self.game_object.transform.position += vertical_vel * toi;
      self.velocity.y = -5.0;
      self.grounded = true;
    } else {
      self.velocity.y -= self.gravity;
      self.grounded = false;
    }
    game.collision.update_object(&mut self.game_object);
  }

  fn update_input(&mut self, input: &Input, camera: &CameraController) {
    let mut direction = Vector3::zero();

    if input.key_held(VirtualKeyCode::W) {
      direction.x = 1.0;
    } else if input.key_held(VirtualKeyCode::S) {
      direction.x = -1.0;
    }

    if input.key_held(VirtualKeyCode::A) {
      direction.y = -1.0;
    } else if input.key_held(VirtualKeyCode::D) {
      direction.y = 1.0;
    }

    let mut movement = Vector3::new(0.0, self.velocity.y, 0.0);
    if !direction.is_zero() {
      direction = cgmath::InnerSpace::normalize(direction) * self.speed;
    }
    movement += camera.forward * (direction.x) + camera.right * (direction.y);

    self.velocity = movement;

    if input.key_pressed(VirtualKeyCode::Space) && self.grounded {
      self.velocity.y = self.jump_height;
    }
  }
}
