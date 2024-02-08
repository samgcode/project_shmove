use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

pub mod render;

pub async fn run() {
  env_logger::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Super Project Yourself At Unreasonably High Velocities Across Vast Distances Over Solid Color Abstract Shapes To Bring A Strange Creature To An Unknown Position In The Void 3D 64 (SPYAUHVAVDOSCASTBASCTAUPITV 3D64)")
    .build(&event_loop).unwrap();

  let mut state = render::State::new(window).await;
  let mut last_render_time = instant::Instant::now();

  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent {
      ref event,
      window_id,
    } if window_id == state.window().id() && !state.input(event) => match event {
      WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
      WindowEvent::Resized(physical_size) => {
        state.resize(*physical_size);
      }
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        state.resize(**new_inner_size);
      }
      _ => {}
    },
    Event::RedrawRequested(window_id) if window_id == state.window().id() => {
      let now = instant::Instant::now();
      let dt = now - last_render_time;
      last_render_time = now;
      state.update(dt);

      match state.render() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(e) => eprintln!("{:?}", e),
      }
    }
    Event::MainEventsCleared => {
      state.window().request_redraw();
    }
    _ => {}
  });
}
