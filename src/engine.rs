use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{Fullscreen, WindowBuilder},
};

pub use self::physics::game_object::{GameObject, Transform};
pub use camera::Camera;
pub use game_state::GameState;
pub use render::color::Color;
pub use render::ui::TextObject;
pub use time::Time;

pub mod camera;
mod game_state;
pub mod physics;
pub mod render;
mod time;

const TITLE: &'static str = "Super Project Yourself At Unreasonably High Velocities Across Vast Distances Over Solid Color Abstract Shapes To Bring A Strange Creature To An Unknown Position In The Void 3D 64 (SPYAUHVAVDOSCASTBASCTAUPITV 3D64)";

pub trait Scene {
  fn start(&mut self, game: &mut GameState);
  fn update(&mut self, game: &mut GameState, input: &physics::input::Input, time: &Time);
  fn get_objects(&mut self) -> (Vec<&mut GameObject>, Vec<&TextObject>);
}

pub async fn run(mut game: impl Scene + 'static) {
  env_logger::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title(TITLE)
    .with_fullscreen(Some(Fullscreen::Borderless(None)))
    .with_theme(Some(winit::window::Theme::Dark))
    .build(&event_loop)
    .unwrap();

  let mut physics_state = physics::State::new();
  let mut game_state = GameState::new();
  let mut render_state = render::State::new(window, &game_state.camera).await;
  let mut time = Time::create();

  game.start(&mut game_state);
  physics_state
    .input
    .updated_window_size(render_state.window());

  event_loop.run(move |event, _, control_flow| {
    physics_state.input.handle_event(&event);
    match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == render_state.window().id() => match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::Resized(physical_size) => {
          render_state.resize(*physical_size);
          physics_state
            .input
            .updated_window_size(render_state.window());
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          render_state.resize(**new_inner_size);
          physics_state
            .input
            .updated_window_size(render_state.window());
        }
        _ => {}
      },
      Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
        time.update();

        physics_state.input.update();
        game.update(&mut game_state, &physics_state.input, &time);

        let (game_objects, text_objects) = game.get_objects();
        render_state.update_clear_color(&game_state.background_color);
        render_state.update(
          &game_state.camera,
          time.delta_time,
          game_objects,
          text_objects,
        );

        match render_state.render() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size()),
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
          Err(e) => eprintln!("{:?}", e),
        }

        if physics_state.input.key_pressed(VirtualKeyCode::Escape) {
          if let Some(_) = render_state.window().fullscreen() {
            render_state.window().set_fullscreen(None);
          } else {
            render_state
              .window()
              .set_fullscreen(Some(Fullscreen::Borderless(None)));
          }
        }
      }
      Event::MainEventsCleared => {
        render_state.window().request_redraw();
      }
      _ => {}
    }
  });
}
