use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use mesh::DrawModel;

mod camera;
mod mesh;
mod resources;

const ROTATION_SPEED: cgmath::Vector3<f32> = cgmath::Vector3::new(
  0.5 * std::f32::consts::PI / 120.0,
  0.0,
  0.5 * std::f32::consts::PI / 90.0,
);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
  view_position: [f32; 4],
  view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  fn new() -> Self {
    Self {
      view_position: [0.0; 4],
      view_proj: cgmath::Matrix4::identity().into(),
    }
  }

  fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
    self.view_position = camera.position.to_homogeneous().into();
    self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into()
  }
}

struct Instance {
  position: cgmath::Vector3<f32>,
  rotation: cgmath::Quaternion<f32>,
}

impl Instance {
  fn to_raw(&self) -> InstanceRaw {
    let model =
      cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
    InstanceRaw {
      model: model.into(),
      normal: cgmath::Matrix3::from(self.rotation).into(),
    }
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
  model: [[f32; 4]; 4],
  normal: [[f32; 3]; 3],
}

impl mesh::Vertex for InstanceRaw {
  fn desc() -> wgpu::VertexBufferLayout<'static> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
          shader_location: 9,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
          shader_location: 10,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
          shader_location: 11,
          format: wgpu::VertexFormat::Float32x3,
        },
      ],
    }
  }
}

pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  camera: camera::Camera,
  projection: camera::Projection,
  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  obj: mesh::Mesh,
  instances: Vec<Instance>,
  instance_buffer: wgpu::Buffer,
  clear_color: wgpu::Color,
  window: Window,
}

impl State {
  pub async fn new(window: Window) -> Self {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
          label: None,
        },
        None,
      )
      .await
      .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    let camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let projection =
      camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, &projection);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Camera Buffer"),
      contents: bytemuck::cast_slice(&[camera_uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("camera_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
      });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("camera_bind_group"),
      layout: &camera_bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    let obj = resources::load_mesh("cube.obj", &device).await.unwrap();

    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const SPACE_BETWEEN: f32 = 3.0;
    let instances = (0..NUM_INSTANCES_PER_ROW)
      .flat_map(|z| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
          let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
          let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

          let position = cgmath::Vector3 { x, y: 0.0, z };

          let rotation = if position.is_zero() {
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
          } else {
            cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
          };

          Instance { position, rotation }
        })
      })
      .collect::<Vec<_>>();

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Instance Buffer"),
      contents: bytemuck::cast_slice(&instance_data),
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    let clear_color = wgpu::Color::BLACK;

    Self {
      surface,
      device,
      queue,
      config,
      size,
      camera,
      projection,
      camera_uniform,
      camera_buffer,
      camera_bind_group,
      obj,
      instances,
      instance_buffer,
      clear_color,
      window,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.projection.resize(new_size.width, new_size.height);
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;

      self.surface.configure(&self.device, &self.config);
    }
  }

  pub fn input(&mut self, _event: &WindowEvent) -> bool {
    false
  }

  pub fn update(&mut self, _dt: instant::Duration) {
    self
      .camera_uniform
      .update_view_proj(&self.camera, &self.projection);

    self.queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );

    for instance in &mut self.instances {
      let amount_x = cgmath::Quaternion::from_angle_x(cgmath::Rad(ROTATION_SPEED.x));
      let amount_y = cgmath::Quaternion::from_angle_y(cgmath::Rad(ROTATION_SPEED.y));
      let amount_z = cgmath::Quaternion::from_angle_z(cgmath::Rad(ROTATION_SPEED.z));
      let current = instance.rotation;
      instance.rotation = amount_x * amount_y * amount_z * current;
    }

    let instance_data = self
      .instances
      .iter()
      .map(Instance::to_raw)
      .collect::<Vec<_>>();
    self.queue.write_buffer(
      &self.instance_buffer,
      0,
      bytemuck::cast_slice(&instance_data),
    );
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;

    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.clear_color),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
      });


      render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
      // render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw_mesh_instanced(
        &self.obj,
        0..self.instances.len() as u32,
        &self.camera_bind_group,
      );
    }
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
    self.size
  }
}
