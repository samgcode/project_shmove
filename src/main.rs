use cgmath::{InnerSpace, Rad, Vector2, Vector3};
use instant::Duration;
use std::f32::consts::FRAC_PI_2;
use winit::event::VirtualKeyCode;

use engine::{physics::input::Input, Camera, GameObject, GameState, Scene, Transform};
use project_shmove::engine;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

struct TestScene {
  speed: f32,
  sensitivity: f32,
  direction: Vector3<f32>,
  mouse_speed: Vector2<f32>,
  object: GameObject,
  timer: f32,
}

impl TestScene {
  pub fn new() -> Self {
    Self {
      speed: 10.0,
      sensitivity: 0.1,
      direction: Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      mouse_speed: Vector2 { x: 0.0, y: 0.0 },
      object: GameObject {
        transform: Transform::default(),
        color: [1.0, 0.0, 0.0],
      },
      timer: 0.0,
    }
  }
}

impl Scene for TestScene {
  fn start(&mut self, _game: &mut GameState) {
    self.object.transform.scale = Vector3 {
      x: 5.0,
      y: 0.5,
      z: 5.0,
    };
    self.object.color = [0.5, 1.0, 0.75];
  }

  fn update(&mut self, game: &mut GameState, input: &Input, dt: Duration) {
    self.timer += 1.0;
    if input.key_pressed(VirtualKeyCode::W) {
      self.direction.x = 1.0;
    } else if input.key_pressed(VirtualKeyCode::S) {
      self.direction.x = -1.0;
    } else {
      self.direction.x = 0.0;
    }

    if input.key_pressed(VirtualKeyCode::A) {
      self.direction.y = -1.0;
    } else if input.key_pressed(VirtualKeyCode::D) {
      self.direction.y = 1.0;
    } else {
      self.direction.y = 0.0;
    }

    if input.key_pressed(VirtualKeyCode::Space) {
      self.direction.z = 1.0;
    } else if input.key_pressed(VirtualKeyCode::LShift) {
      self.direction.z = -1.0;
    } else {
      self.direction.z = 0.0;
    }

    self.mouse_speed = input.get_mouse_speed();

    self.object.transform.scale.x = 0.5 + 2.0*f32::abs(f32::sin(self.timer/50.0));
    self.object.transform.scale.y = 0.5 + 2.0*f32::abs(f32::cos(self.timer/75.0));
    self.object.transform.scale.z = 0.5 + 2.0*f32::abs(f32::sin(self.timer/100.0));

    self.object.color = [
      f32::abs(f32::sin(self.timer/200.0+0.25)),
      f32::abs(f32::sin(self.timer/200.0+0.5)),
      f32::abs(f32::sin(self.timer/200.0+0.75)),
    ];

    self.update_camera(&mut game.camera, dt)
  }

  fn get_active_game_objects(&mut self) -> Vec<&GameObject> {
    let mut objects = Vec::<&GameObject>::new();
    objects.push(&self.object);
    objects
  }
}

impl TestScene {
  pub fn update_camera(&self, camera: &mut Camera, dt: Duration) {
    let dt = dt.as_secs_f32();

    let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
    let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();

    camera.position += forward * (self.direction.x) * self.speed * dt;
    camera.position += right * (self.direction.y) * self.speed * dt;
    camera.position.y += self.direction.z * self.speed * dt;

    camera.yaw += Rad(self.mouse_speed.x) * self.sensitivity * dt;
    camera.pitch += Rad(-self.mouse_speed.y) * self.sensitivity * dt;

    if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
      camera.pitch = -Rad(SAFE_FRAC_PI_2);
    } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
      camera.pitch = Rad(SAFE_FRAC_PI_2);
    }
  }
}

fn main() {
  let scene = TestScene::new();
  pollster::block_on(engine::run(scene));
}
