use cgmath::{Vector3, Zero};
use project_shmove::engine::{
  physics::{
    collision::{EventStatus, Tag},
    input::Input,
  },
  GameObject, GameState, Time,
};
use winit::event::VirtualKeyCode;

use super::camera::CameraController;

const GRAVITY: f32 = 1.0;
const FRICTION: f32 = 1.0;
const FAST_FRICTION: f32 = 0.15;

const WALK_SPEED: f32 = 8.0;
const SPRINT_SPEED: f32 = 15.0;
const SPRINT_JMP_BOOST: f32 = 2.0;

const NORMAL_JUMP: f32 = 25.0;
const SPRINT_JUMP: f32 = 30.0;

const REQUIRED_WALK_TIME: f32 = 0.5;

enum MovmentState {
  Static,
  Crouching,
  Walking(f32),
  Sliding(f32),
  Running,
  SpeedSliding(f32),
  Uncapped,
}

pub struct Controller {
  pub game_object: GameObject,
  velocity: Vector3<f32>,
  grounded: bool,
  movement_state: MovmentState,
  speed: f32,
  prev_direction: Vector3<f32>,
}

impl Controller {
  pub fn new() -> Self {
    Self {
      game_object: GameObject::new(
        (0.0, 4.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 2.0, 1.0),
        [1.0, 0.0, 0.0],
        Tag::Player,
      ),
      velocity: Vector3::new(0.0, 0.0, 0.0),
      grounded: false,
      movement_state: MovmentState::Static,
      speed: 0.0,
      prev_direction: Vector3::new(1.0, 0.0, 0.0),
    }
  }

  pub fn update(
    &mut self,
    game: &mut GameState,
    input: &Input,
    camera: &CameraController,
    time: &Time,
  ) {
    self.update_position(game, time.delta_time);
    self.update_input(input, camera, time);

    if self.game_object.transform.position.y < -50.0 {
      self.velocity = Vector3::zero();
      self.game_object.transform.position = Vector3::new(0.0, 4.0, 0.0);
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
      self.velocity.y -= GRAVITY;
      self.grounded = false;
    }
    game.collision.update_object(&mut self.game_object);
  }

  fn update_input(&mut self, input: &Input, camera: &CameraController, time: &Time) {
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
      self.prev_direction = direction
    } else {
      self.movement_state = MovmentState::Static;
      direction = cgmath::InnerSpace::normalize(self.prev_direction) * self.speed;
    }
    movement += camera.forward * (direction.x) + camera.right * (direction.y);

    self.velocity = movement;

    if self.speed > SPRINT_SPEED {
      self.movement_state = MovmentState::Uncapped;
    }

    if let MovmentState::Static = self.movement_state {
      if self.speed > WALK_SPEED {
        self.speed -= FRICTION;
      } else {
        self.speed = 0.0;
      }
    }

    if self.grounded {
      match self.movement_state {
        MovmentState::Static => {
          self.movement_state = MovmentState::Walking(time.elapsed_time);
        }
        MovmentState::Walking(start_time) => {
          if time.elapsed_time - start_time > REQUIRED_WALK_TIME {
            self.speed = SPRINT_SPEED;
            self.movement_state = MovmentState::Running;
          } else {
            self.speed = WALK_SPEED;
          }
        }
        MovmentState::Running => self.speed = SPRINT_SPEED,
        MovmentState::Uncapped => {
          self.speed -= FAST_FRICTION;
          if self.speed <= SPRINT_SPEED {
            self.movement_state = MovmentState::Running;
          }
        }
        _ => {}
      }

      if input.key_pressed(VirtualKeyCode::Space) {
        if let MovmentState::Running | MovmentState::Uncapped = self.movement_state {
          self.velocity.y = SPRINT_JUMP;
          self.speed += SPRINT_JMP_BOOST;
        } else {
          self.velocity.y = NORMAL_JUMP;
        }
      }
    }

    println!("speed: {:?}", self.speed);
  }
}
