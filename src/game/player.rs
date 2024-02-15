use cgmath::{Vector2, Vector3, Zero};
use project_shmove::engine::{
  camera,
  physics::{
    collision::{EventStatus, Tag},
    input::Input,
  },
  GameObject, GameState, Time,
};
use winit::event::VirtualKeyCode;

use super::camera::CameraController;

const GRAVITY: f32 = 1.0;
const FRICTION: f32 = 0.1;
const FAST_FRICTION: f32 = 0.015;
const MIN_OPPOSING_MULTIPLIER: f32 = 0.95;

const CROUCH_WALK_SPEED: f32 = 4.0;
const WALK_SPEED: f32 = 8.0;
const SPRINT_SPEED: f32 = 15.0;

const SPRINT_JUMP_BOOST: f32 = 2.0;
const SLIDE_BOOST: f32 = 1.2;

const CROUCH_JUMP: f32 = 20.0;
const NORMAL_JUMP: f32 = 30.0;
const SPRINT_JUMP: f32 = 25.0;

const CROUCH_JUMP_HEIGHT: f32 = (-CROUCH_JUMP * CROUCH_JUMP) / (2.0 * GRAVITY);
const NORMAL_JUMP_HEIGHT: f32 = (-NORMAL_JUMP * NORMAL_JUMP) / (2.0 * GRAVITY);
const SPRINT_JUMP_HEIGHT: f32 = (-SPRINT_JUMP * SPRINT_JUMP) / (2.0 * GRAVITY);

const REQUIRED_WALK_TIME: f32 = 0.5;
const SLIDE_TIME: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
enum MovementState {
  Static,
  Crouching,
  CrouchWalking,
  Walking(f32),
  Sprinting,
  Sliding(f32),
  SpeedSliding(f32),
  Uncapped,
}
use MovementState::*;

pub struct Controller {
  pub game_object: GameObject,
  grounded: bool,
  movement_state: MovementState,
  velocity: Vector3<f32>,
  direction: Vector2<f32>,
  speed: f32,
  input_direction: Vector2<f32>,
  jump_pressed: bool,
  crouch_pressed: bool,
}

impl Controller {
  pub fn new() -> Self {
    Self {
      game_object: GameObject::new(
        (0.0, 5.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 2.0, 1.0),
        [1.0, 0.0, 0.0],
        Tag::Player,
      ),
      grounded: false,
      movement_state: Static,
      velocity: Vector3::zero(),
      direction: Vector2::zero(),
      speed: 0.0,
      input_direction: Vector2::zero(),
      jump_pressed: false,
      crouch_pressed: false,
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
    self.update_input(input, camera);
    self.update_velocity(time);

    if self.game_object.transform.position.y < -50.0 {
      self.velocity = Vector3::zero();
      self.speed = 0.0;
      self.movement_state = Static;
      self.input_direction = Vector2::zero();
      self.direction = Vector2::zero();
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

  fn update_input(&mut self, input: &Input, camera: &CameraController) {
    let mut direction = Vector3::zero();

    self.jump_pressed = input.key_pressed(VirtualKeyCode::Space);
    self.crouch_pressed = input.key_pressed(VirtualKeyCode::LShift);

    if input.key_held(VirtualKeyCode::W) {
      direction.x = 1.0;
    } else if input.key_held(VirtualKeyCode::S) {
      direction.x = -1.0;
    }

    if input.key_held(VirtualKeyCode::A) {
      direction.z = -1.0;
    } else if input.key_held(VirtualKeyCode::D) {
      direction.z = 1.0;
    }

    if direction.is_zero() {
      self.input_direction = Vector2::zero();
    } else {
      let direction = camera.forward * (direction.x) + camera.right * (direction.z);
      self.input_direction = cgmath::InnerSpace::normalize(Vector2::new(direction.x, direction.z));
      if f32::abs(self.input_direction.x) < 0.000001 {
        self.input_direction.x = 0.0;
      }
      if f32::abs(self.input_direction.y) < 0.000001 {
        self.input_direction.y = 0.0;
      }
    }
  }

  fn update_velocity(&mut self, time: &Time) {
    if !self.jump_pressed {
      self.apply_friction();
    }

    if self.grounded {
      if self.jump_pressed {
        self.update_jump();
      }
      self.update_crouch(time);
    }

    if self.speed <= SPRINT_SPEED {
      if !self.input_direction.is_zero() {
        self.direction = self.input_direction;
        self.update_capped_movement(time);
      }
    }

    if !self.direction.is_zero() {
      self.direction = cgmath::InnerSpace::normalize(self.direction);
    }

    if self.speed > SPRINT_SPEED {
      if !self.grounded {
        if !self.direction.is_zero() && !self.input_direction.is_zero() {
          if self.direction != self.input_direction {
            let alignment = cgmath::dot(self.direction, self.input_direction);
            let perpendicular = self.input_direction - self.direction * alignment;
            self.direction += perpendicular * 0.03;

            const DIVISOR: f32 = 2.0 / (1.0 - MIN_OPPOSING_MULTIPLIER);
            let alignment = (alignment + 1.0) / DIVISOR;
            if alignment <= 1.0 {
              self.speed *= MIN_OPPOSING_MULTIPLIER + alignment;
            }
            if self.speed < SPRINT_SPEED {
              self.movement_state = Sprinting;
            }
          }
        }
      }
    }

    if self.speed == 0.0 {
      self.direction = Vector2::zero();
    }

    let vel = if self.direction.is_zero() {
      Vector2::zero()
    } else {
      cgmath::InnerSpace::normalize(self.direction) * self.speed
    };
    self.velocity = Vector3::new(vel.x, self.velocity.y, vel.y);
  }

  fn update_capped_movement(&mut self, time: &Time) {
    match self.movement_state {
      Static => {
        self.speed = WALK_SPEED;
        self.movement_state = Walking(time.elapsed_time);
      }
      Crouching => {
        self.speed = CROUCH_WALK_SPEED;
        self.movement_state = CrouchWalking;
      }
      CrouchWalking => {
        self.speed = CROUCH_WALK_SPEED;
      }
      Walking(start_time) => {
        self.speed = WALK_SPEED;
        if time.elapsed_time - start_time > REQUIRED_WALK_TIME {
          self.movement_state = Sprinting;
        }
      }
      Sliding(start_time) => {
        self.speed = SPRINT_SPEED;
        if time.elapsed_time - start_time > SLIDE_TIME {
          self.movement_state = Sprinting;
        }
      }
      Sprinting => {
        self.speed = SPRINT_SPEED;
      }
      Uncapped | SpeedSliding(_) => {
        panic!("Invalid movment state: expected capped, is uncapped")
      }
    }
  }

  fn update_jump(&mut self) {
    match self.movement_state {
      Static | Walking(_) => {
        self.velocity.y = NORMAL_JUMP;
      }
      Crouching | CrouchWalking => {
        self.velocity.y = CROUCH_JUMP;
      }
      Sliding(_) => {
        self.velocity.y = CROUCH_JUMP;
        self.movement_state = Sprinting;
      }
      SpeedSliding(_) => {
        self.velocity.y = CROUCH_JUMP;
        self.movement_state = Uncapped;
      }
      Sprinting => {
        self.velocity.y = SPRINT_JUMP;
        self.speed += SPRINT_JUMP_BOOST;
        self.movement_state = Uncapped;
      }
      Uncapped => {
        self.velocity.y = SPRINT_JUMP;
        self.speed += SPRINT_JUMP_BOOST;
      }
    }
  }

  fn update_crouch(&mut self, time: &Time) {
    if self.crouch_pressed {
      self.movement_state = match self.movement_state {
        Static => Crouching,
        Walking(_) => Sliding(time.elapsed_time),
        Crouching => Static,
        CrouchWalking => Walking(time.elapsed_time),
        Sprinting | Uncapped => {
          self.speed *= SLIDE_BOOST;
          SpeedSliding(time.elapsed_time)
        }
        Sliding(_) | SpeedSliding(_) => Sprinting,
      }
    } else {
      match self.movement_state {
        Sliding(start_time) => {
          self.speed = SPRINT_SPEED;
          if time.elapsed_time - start_time > SLIDE_TIME {
            self.movement_state = Sprinting;
          }
        }
        SpeedSliding(start_time) => {
          if time.elapsed_time - start_time > SLIDE_TIME {
            self.movement_state = Uncapped;
          }
        }
        Crouching => {
          self.speed = CROUCH_WALK_SPEED;
          self.movement_state = CrouchWalking;
        }
        _ => {}
      }
    }
  }

  fn apply_friction(&mut self) {
    if self.grounded {
      if self.speed > SPRINT_SPEED {
        if let Sliding(_) | SpeedSliding(_) = self.movement_state {
        } else {
          self.speed *= 1.0 - FAST_FRICTION;
        }
      } else if self.input_direction.is_zero() {
        if self.speed > WALK_SPEED {
          self.speed *= 1.0 - FRICTION;
        } else if self.speed > 1.0 {
          if let Sprinting = self.movement_state {
            self.movement_state = Static;
          }
          self.speed *= 1.0 - FRICTION;
        } else {
          self.speed = 0.0;
          self.movement_state = Static;
        }
      }
    }

    if let Uncapped = self.movement_state {
      if self.speed <= SPRINT_SPEED {
        self.movement_state = Sprinting;
      }
    }
  }
}
