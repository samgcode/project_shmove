use cgmath::{Vector2, Vector3, Zero};
use project_shmove::engine::{
  physics::{
    collision::{EventStatus, Tag},
    input::Input,
  },
  Color, GameObject, GameState, TextObject, Time,
};
use winit::event::VirtualKeyCode;

use super::camera::CameraController;

const GRAVITY: f32 = 1.0;
const FRICTION: f32 = 0.5;
const AIR_CONTROL: f32 = 0.25;
const REVERSE_AIR_CONTROL: f32 = 0.05;
const MIN_OPPOSING_MULTIPLIER: f32 = 0.95;

// const CROUCH_WALK_SPEED: f32 = 8.0;
const WALK_SPEED: f32 = 12.0;
const SPRINT_SPEED: f32 = 15.0;

const SPEED_LIMIT: f32 = 30.0;

// const CROUCH_JUMP: f32 = 15.0;
const NORMAL_JUMP: f32 = 20.0;
const SPRINT_JUMP_BOOST: f32 = 2.0;

// const CROUCH_JUMP_HEIGHT: f32 = (-CROUCH_JUMP * CROUCH_JUMP) / (2.0 * GRAVITY);
// const NORMAL_JUMP_HEIGHT: f32 = (-NORMAL_JUMP * NORMAL_JUMP) / (2.0 * GRAVITY);
// const SPRINT_JUMP_HEIGHT: f32 = (-SPRINT_JUMP * SPRINT_JUMP) / (2.0 * GRAVITY);

// const NORMAL_HEIGHT: f32 = 2.0;
// const CROUCHED_HEIGHT: f32 = 0.5;

pub struct Controller {
  pub game_object: GameObject,
  pub camera_position: Vector3<f32>,
  grounded: bool,
  velocity: Vector3<f32>,
  direction: Vector2<f32>,
  speed: f32,
  input_direction: Vector2<f32>,
  jump_pressed: bool,
  crouch_pressed: bool,
  crouch_held: bool,
  pub debug_text: TextObject,
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
      camera_position: Vector3::new(0.0, 5.0, 0.0),
      grounded: false,
      velocity: Vector3::zero(),
      direction: Vector2::zero(),
      speed: 0.0,
      input_direction: Vector2::zero(),
      jump_pressed: false,
      crouch_pressed: false,
      crouch_held: false,
      debug_text: TextObject::default(),
    }
  }

  pub fn start(&mut self) {
    self.debug_text.size = 17.0;
    self.debug_text.position = Vector2::<f32>::new(2.0, 30.0);
    self.debug_text.color = Color::from_rgb(1.0, 1.0, 1.0);
  }

  pub fn update(
    &mut self,
    game: &mut GameState,
    input: &Input,
    camera: &CameraController,
    time: &Time,
  ) {
    self.debug_text.text = String::from(format!("speed: {}", self.speed));
    self.debug_text.text += "\n";

    self.update_position(game, time.delta_time);
    self.update_input(input, camera);
    self.update_velocity(time);

    // if let Crouching | CrouchWalking | Sliding(_) | SpeedSliding(_) = self.movement_state {
    //   self.game_object.transform.scale.y = CROUCHED_HEIGHT;
    // } else {
    //   self.game_object.transform.scale.y = NORMAL_HEIGHT;
    // }
    self.camera_position = self.game_object.transform.position;

    if self.game_object.transform.position.y < -50.0 {
      self.velocity = Vector3::zero();
      self.speed = 0.0;
      self.input_direction = Vector2::zero();
      self.direction = Vector2::zero();
      self.game_object.transform.position = Vector3::new(0.0, 4.0, 0.0);
    }

    // println!("{}", self.debug_text.text);
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

    self.jump_pressed = input.key_held(VirtualKeyCode::Space);
    self.crouch_pressed = input.key_pressed(VirtualKeyCode::LShift);
    self.crouch_held = input.key_held(VirtualKeyCode::LShift);

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

  fn update_velocity(&mut self, _time: &Time) {
    if self.grounded && self.speed > SPRINT_SPEED {
      self.speed -= FRICTION;
      if self.speed < SPRINT_SPEED {
        self.speed = SPRINT_SPEED;
      }
    }

    if self.grounded && self.jump_pressed {
      self.velocity.y = NORMAL_JUMP;
      self.speed += SPRINT_JUMP_BOOST;
    }

    self.debug_text.text += &format!("direction: {:?}\n", self.direction);
    self.debug_text.text += &format!("input: {:?}\n", self.input_direction);

    if self.input_direction.is_zero() {
      self.speed -= FRICTION;
    } else {
      if self.speed < SPEED_LIMIT {
        self.direction = self.input_direction;
        if self.speed < WALK_SPEED {
          self.speed = WALK_SPEED;
        }
      } else {
        let alignment = cgmath::dot(self.direction, self.input_direction);
        let perpendicular = (self.input_direction - self.direction) * alignment;
        if alignment > 0.0 {
          self.direction += perpendicular * alignment * AIR_CONTROL;
        } else {
          self.direction += perpendicular * REVERSE_AIR_CONTROL;
          const DIVISOR: f32 = 2.0 / (1.0 - MIN_OPPOSING_MULTIPLIER);
          let alignment = (alignment + 1.0) / DIVISOR;
          self.speed *= MIN_OPPOSING_MULTIPLIER + alignment;
        }
        self.direction = cgmath::InnerSpace::normalize(self.direction);
      }
    }

    if self.speed < 3.0 {
      self.speed = 0.0;
    }

    let vel = if self.direction.is_zero() {
      Vector2::zero()
    } else {
      cgmath::InnerSpace::normalize(self.direction) * self.speed
    };

    self.velocity = Vector3::new(vel.x, self.velocity.y, vel.y);
  }
}
