use cgmath::{Vector3, Zero};
use winit::event::VirtualKeyCode;

use engine::{physics::input::Input, GameObject, GameState, Scene, Transform};
use project_shmove::engine;

use self::camera::CameraController;

mod camera;

pub struct GameScene {
  camera_controller: CameraController,
  camera_speed: f32,
  object: GameObject,
  timer: f32,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController { sensitivity: 1.0 },
      camera_speed: 5.0,
      object: GameObject {
        transform: Transform::default(),
        color: [1.0, 0.0, 0.0],
      },
      timer: 0.0,
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, _game: &mut GameState) {
    self.object.transform.scale = Vector3 {
      x: 5.0,
      y: 0.5,
      z: 5.0,
    };
    self.object.color = [0.5, 1.0, 0.75];
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

    self.object.transform.scale.x = 0.5 + 2.0 * f32::abs(f32::sin(self.timer / 50.0));
    self.object.transform.scale.y = 0.5 + 2.0 * f32::abs(f32::cos(self.timer / 75.0));
    self.object.transform.scale.z = 0.5 + 2.0 * f32::abs(f32::sin(self.timer / 100.0));

    self.object.color = [
      f32::abs(f32::sin(self.timer / 200.0 + 0.25)),
      f32::abs(f32::sin(self.timer / 200.0 + 0.5)),
      f32::abs(f32::sin(self.timer / 200.0 + 0.75)),
    ];

    self.camera_controller.update(
      &mut game.camera,
      input.get_mouse_speed(),
      direction * self.camera_speed,
      dt,
    );
  }

  fn get_active_game_objects(&mut self) -> Vec<&GameObject> {
    let mut objects = Vec::<&GameObject>::new();
    objects.push(&self.object);
    objects
  }
}
