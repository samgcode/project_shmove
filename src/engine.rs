use instant::Duration;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

pub use camera::Camera;
pub use game_state::GameState;

use self::physics::game_object::{GameObject, Transform};

pub mod camera;
mod game_state;
pub mod physics;
pub mod render;

pub trait Scene {
  fn update(&mut self, sgame: &mut GameState, input: &physics::input::Input, dt: Duration);
}

pub async fn run(mut game: impl Scene + 'static) {
  env_logger::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Super Project Yourself At Unreasonably High Velocities Across Vast Distances Over Solid Color Abstract Shapes To Bring A Strange Creature To An Unknown Position In The Void 3D 64 (SPYAUHVAVDOSCASTBASCTAUPITV 3D64)")
    .build(&event_loop).unwrap();

  let mut physics_state = physics::State::new();
  let mut last_render_time = instant::Instant::now();
  let mut game_state = GameState::new();
  let mut render_state = render::State::new(window, &game_state.camera).await;

  let game_object = GameObject {
    transform: Transform::default(),
    color: [1.0, 0.0, 0.75]
  };
  
  game_state.game_objects = vec![game_object];

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
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          render_state.resize(**new_inner_size);
        }
        _ => {}
      },
      Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
        let now = instant::Instant::now();
        let dt = now - last_render_time;
        last_render_time = now;
        render_state.update(&game_state.camera, dt, &game_state.game_objects);

        match render_state.render() {
          Ok(_) => {}
          Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size()),
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
          Err(e) => eprintln!("{:?}", e),
        }

        physics_state.input.update();
        game.update(&mut game_state, &physics_state.input, dt);
      }
      Event::MainEventsCleared => {
        render_state.window().request_redraw();
      }
      _ => {}
    }
  });
}
