use engine::{physics::input::Input, GameObject, GameState, Scene};
use project_shmove::engine::{self, physics::collision::Tag};

use self::camera::CameraController;

mod camera;
mod player;

pub struct GameScene {
  camera_controller: CameraController,
  player_controller: player::Controller,
  ground: GameObject,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController::new(1.0),
      player_controller: player::Controller::new(),
      ground: GameObject::new(
        (0.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        (5.0, 2.0, 5.0),
        [0.0, 1.0, 0.0],
        Tag::Platform,
      ),
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, game: &mut GameState) {
    self
      .player_controller
      .game_object
      .register_collision(&mut game.collision);
    self.ground.register_collision(&mut game.collision);
  }

  fn update(&mut self, game: &mut GameState, input: &Input, dt: f32) {
    self
      .player_controller
      .update(game, input, &self.camera_controller, dt);

    self
      .camera_controller
      .set_pos(self.player_controller.game_object.transform.position);
    self
      .camera_controller
      .update(&mut game.camera, input.get_mouse_speed(), dt);
  }

  fn get_active_game_objects(&mut self) -> Vec<&mut GameObject> {
    let mut objects = Vec::<&mut GameObject>::new();
    objects.push(&mut self.player_controller.game_object);
    objects.push(&mut self.ground);
    objects
  }
}
