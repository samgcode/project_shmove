use cgmath::{Vector3, Zero};
use winit::event::VirtualKeyCode;

use engine::{physics::input::Input, GameObject, GameState, Scene};
use project_shmove::engine::{
  self,
  physics::collision::{EventStatus, Tag},
};

use self::camera::CameraController;

mod camera;

const GRAVITY: f32 = 0.01;
const SPEED: f32 = 0.1;

pub struct GameScene {
  camera_controller: CameraController,
  camera_speed: f32,
  player: GameObject,
  ground: GameObject,
  gravity: f32,
  velocity: Vector3<f32>,
  y_velocity: f32,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController { sensitivity: 1.0 },
      camera_speed: 5.0,
      player: GameObject::new(
        (0.0, 2.0, 10.0),
        (0.0, 0.0, 0.0),
        (1.0, 2.0, 1.0),
        [1.0, 0.0, 0.0],
        Tag::Player,
      ),
      ground: GameObject::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        (5.0, 0.5, 15.0),
        [0.0, 1.0, 0.0],
        Tag::Platform,
      ),
      gravity: GRAVITY,
      velocity: Vector3::new(0.0, 0.0, 0.0),
      y_velocity: 0.0,
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, game: &mut GameState) {
    self.player.register_collision(&mut game.collision);
    self.ground.register_collision(&mut game.collision);
  }

  fn update(&mut self, game: &mut GameState, input: &Input, dt: f32) {
    if let EventStatus::Enter | EventStatus::Stay = self.player.collision.status {
      if !self.velocity.is_zero() {
        self.player.transform.position -= self.velocity;
        let toi = game.collision.get_toi(
          &mut self.player.transform,
          self.velocity,
          self.player.collision.other_handle,
        );
        self.player.transform.position += self.velocity * toi;

        let normal_vel = cgmath::InnerSpace::normalize(self.velocity);
        let parallel_direction = cgmath::InnerSpace::normalize(
          normal_vel
            - self.player.collision.normal * cgmath::dot(self.player.collision.normal, normal_vel),
        );

        self.player.transform.position +=
          parallel_direction * (cgmath::InnerSpace::magnitude(self.velocity) * (1.0 - toi));
      }
    } else {
      self.y_velocity -= self.gravity;
      self.player.transform.position.y += self.y_velocity;
      game.collision.update_object(&mut self.player);
      if let EventStatus::Enter | EventStatus::Stay = self.player.collision.status {
        self.player.transform.position.y -= self.y_velocity;
        let toi = game.collision.get_toi(
          &mut self.player.transform,
          Vector3::new(0.0, self.y_velocity, 0.0),
          self.player.collision.other_handle,
        );
        self.player.transform.position.y += self.y_velocity * toi;
        self.y_velocity = 0.0;
      }
    }

    if input.key_pressed(VirtualKeyCode::Up) {
      self.velocity.z = SPEED;
    } else if input.key_pressed(VirtualKeyCode::Down) {
      self.velocity.z = -SPEED;
    } else {
      self.velocity.z = 0.0;
    }
    self.player.transform.position += self.velocity;

    let mut direction = Vector3::zero();

    if input.key_pressed(VirtualKeyCode::W) {
      direction.x = 1.0;
    } else if input.key_pressed(VirtualKeyCode::S) {
      direction.x = -1.0;
    }

    if input.key_pressed(VirtualKeyCode::A) {
      direction.y = -1.0;
    } else if input.key_pressed(VirtualKeyCode::D) {
      direction.y = 1.0;
    }

    if input.key_pressed(VirtualKeyCode::Space) {
      direction.z = 1.0;
    } else if input.key_pressed(VirtualKeyCode::LShift) {
      direction.z = -1.0;
    }

    self.camera_controller.update(
      &mut game.camera,
      input.get_mouse_speed(),
      direction * self.camera_speed,
      dt,
    );
  }

  fn get_active_game_objects(&mut self) -> Vec<&mut GameObject> {
    let mut objects = Vec::<&mut GameObject>::new();
    objects.push(&mut self.player);
    objects.push(&mut self.ground);
    objects
  }
}
