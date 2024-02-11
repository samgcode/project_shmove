use cgmath::{Vector3, Zero};
use winit::event::VirtualKeyCode;

use engine::{physics::input::Input, GameObject, GameState, Scene};
use project_shmove::engine::{
  self,
  physics::collision::{CollisionEvent, Tag},
};

use self::camera::CameraController;

mod camera;

const GRAVITY: f32 = -0.0025;

pub struct GameScene {
  camera_controller: CameraController,
  camera_speed: f32,
  player: GameObject,
  ground: GameObject,
  grav: f32,
  velocity: Vector3<f32>,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController { sensitivity: 1.0 },
      camera_speed: 5.0,
      player: GameObject::new(
        (0.0, 8.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 2.0, 1.0),
        [1.0, 0.0, 0.0],
        Tag::Player,
      ),
      ground: GameObject::new(
        (0.0, 0.0, 0.0),
        (45.0, 0.0, 0.0),
        (0.8, 0.5, 2.0),
        [0.0, 1.0, 0.0],
        Tag::Platform,
      ),
      grav: GRAVITY,
      velocity: Vector3::zero(),
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, game: &mut GameState) {
    self.player.register_collision(&mut game.collision);
    self.ground.register_collision(&mut game.collision);
  }

  fn update(&mut self, game: &mut GameState, input: &Input, dt: f32) {
    if let Some(collision_event) = self.player.collision {
      self.player.transform.position -= collision_event.normal * (collision_event.depth + 0.02);
      self.velocity = Vector3::zero();
    }

    let mut direction = Vector3::zero();

    if input.key_pressed(VirtualKeyCode::Up) {
      self.velocity.x += 1.0 * dt;
    } else if input.key_pressed(VirtualKeyCode::Down) {
      self.velocity.x -= 1.0 * dt;
    } else {
      self.velocity.x = 0.0;
    }

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

    self.player.transform.position += self.velocity;
    self.velocity.y += self.grav;
  }

  fn get_active_game_objects(&mut self) -> Vec<&mut GameObject> {
    let mut objects = Vec::<&mut GameObject>::new();
    objects.push(&mut self.player);
    objects.push(&mut self.ground);
    objects
  }
}
