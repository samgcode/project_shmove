use engine::{physics::input::Input, GameObject, GameState, Scene};
use project_shmove::engine::{
  self,
  physics::collision::{Collision, Tag},
  render::color::Color,
  TextObject, Time,
};

use self::camera::CameraController;

mod camera;
mod player;

pub struct GameScene {
  camera_controller: CameraController,
  player_controller: player::Controller,
  platforms: Vec<GameObject>,
  fps_text: TextObject,
}

impl GameScene {
  pub fn new() -> Self {
    Self {
      camera_controller: CameraController::new(1.0),
      player_controller: player::Controller::new(),
      platforms: Vec::<GameObject>::new(),
      fps_text: TextObject::default(),
    }
  }

  fn create_platforms(&mut self, collision: &mut Collision) {
    #[rustfmt::skip] {
      self.platforms.push(GameObject::new((0.0, 0.0, 0.0),(0.0, 0.0, 0.0),(10.0, 1.0, 500.0), [1.0, 0.0, 0.0], Tag::Platform));
      self.platforms.push(GameObject::new((30.0, 0.0, 0.0),(10.0, 0.0, 0.0),(5.0, 2.0, 5.0), [0.0, 1.0, 0.0], Tag::Platform));
      self.platforms.push(GameObject::new((30.0, 0.0, 15.0),(0.0, 0.0, 0.0),(5.0, 0.5, 5.0), [0.0, 0.0, 1.0], Tag::Platform));
      self.platforms.push(GameObject::new((35.0, 0.0, 32.0),(0.0, 0.0, 60.0),(4.0, 0.5, 5.0), [0.0, 1.0, 0.5], Tag::Platform));
      self.platforms.push(GameObject::new((30.0, 2.0, 60.0),(-20.0, 0.0, 0.0),(5.0, 0.5, 5.0), [0.0, 0.5, 1.0], Tag::Platform));
    };
    for platform in self.platforms.iter_mut() {
      platform.register_collision(collision);
    }
  }
}

impl Scene for GameScene {
  fn start(&mut self, game: &mut GameState) {
    self
      .player_controller
      .game_object
      .register_collision(&mut game.collision);

    self.create_platforms(&mut game.collision);

    self.fps_text.size = 20.0;
  }

  fn update(&mut self, game: &mut GameState, input: &Input, time: &Time) {
    game
      .background_color
      .set_hue(time.elapsed_time as f64 * 25.0);

    self
      .player_controller
      .update(game, input, &self.camera_controller, time);

    self
      .camera_controller
      .set_pos(self.player_controller.camera_position);
    self
      .camera_controller
      .update(&mut game.camera, input.get_mouse_speed(), time);

    self.fps_text.color = Color::from_hsv(time.elapsed_time as f64 * 50.0, 1.0, 1.0);
    self.fps_text.text = String::from(format!("{}", (1.0 / time.delta_time) as i32))
  }

  fn get_objects(&mut self) -> (Vec<&mut GameObject>, Vec<&engine::TextObject>) {
    let mut objects = Vec::<&mut GameObject>::new();
    objects.push(&mut self.player_controller.game_object);
    for platform in self.platforms.iter_mut() {
      objects.push(platform);
    }
    (objects, vec![&self.fps_text])
  }
}
