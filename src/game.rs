use cgmath::{Vector3, Zero};
use winit::event::VirtualKeyCode;

use engine::{physics::input::Input, GameObject, GameState, Scene, Transform};
use project_shmove::engine::{self, physics::collision::colliding};

use self::camera::CameraController;

mod camera;

pub struct GameScene {
  camera_controller: CameraController,
  camera_speed: f32,
  player: GameObject,
  ground: GameObject,
  timer: f32,
  prev: bool,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController { sensitivity: 1.0 },
      camera_speed: 5.0,
      player: GameObject {
        transform: Transform::default(),
        color: [1.0, 0.0, 0.0],
      },
      ground: GameObject {
        transform: Transform::default(),
        color: [1.0, 0.0, 0.0],
      },
      timer: 0.0,
      prev: false,
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, _game: &mut GameState) {
    self.player.transform.scale = Vector3 {
      x: 1.0,
      y: 2.0,
      z: 1.0,
    };
    self.player.color = [0.0, 0.0, 1.0];
    self.player.transform.position = Vector3 {
      x: 0.0,
      y: 8.0,
      z: 0.0,
    };
    self.ground.transform.scale = Vector3 {
      x: 10.0,
      y: 0.5,
      z: 10.0,
    };
    self.ground.transform.rotation = Vector3 {
      x: 20.0,
      y: 20.0,
      z: 0.0,
    };
    self.ground.color = [0.0, 1.0, 0.0];
  }

  fn update(&mut self, game: &mut GameState, input: &Input, dt: f32) {
    self.timer += 1.0;

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

    self.player.transform.position.y -= 2.0 * dt;
    let collision = colliding(&self.player, &self.ground);
    if collision.colliding != self.prev {
      if collision.colliding {
        println!(
          "depth: {:?}, normal: {:?}",
          collision.depth, collision.normal
        );
      }
      self.prev = collision.colliding;
    }
  }

  fn get_active_game_objects(&mut self) -> Vec<&GameObject> {
    let mut objects = Vec::<&GameObject>::new();
    objects.push(&self.player);
    objects.push(&self.ground);
    objects
  }
}
