use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::window::Window;

use mesh::{DrawModel, Vertex};

use crate::engine::camera;
use light::Light;

use super::physics::game_object::Transform;

mod light;
mod mesh;
mod resources;
mod texture;

const MAX_INSTANCES: u64 = 100;

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

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
  model: [[f32; 4]; 4],
  normal: [[f32; 3]; 3],
}

impl InstanceRaw {
  pub fn from_transform(transform: &Transform) -> Self {
    use cgmath::{Quaternion, Rad};
    let amount_x = Quaternion::from_angle_x(Rad(transform.rotation.x));
    let amount_y = Quaternion::from_angle_y(Rad(transform.rotation.y));
    let amount_z = Quaternion::from_angle_z(Rad(transform.rotation.z));
    let rotation = amount_x * amount_y * amount_z;

    let model = cgmath::Matrix4::from_translation(transform.position)
      * cgmath::Matrix4::from(rotation)
      * cgmath::Matrix4::from_nonuniform_scale(
        transform.scale.x,
        transform.scale.y,
        transform.scale.z,
      );
    Self {
      model: model.into(),
      normal: cgmath::Matrix3::from(rotation).into(),
    }
  }
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
  render_pipeline: wgpu::RenderPipeline,
  projection: camera::Projection,
  camera_uniform: CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  obj: mesh::Mesh,
  instance_buffer: wgpu::Buffer,
  instance_count: u32,
  clear_color: wgpu::Color,
  depth_texture: texture::Texture,
  light: Light,
  window: Window,
}

impl State {
  pub async fn new(window: Window, camera: &camera::Camera) -> Self {
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

    let light = Light::new(
      &device,
      &camera_bind_group_layout,
      wgpu::ShaderSource::Wgsl(include_str!("render/shader/light.wgsl").into()),
      config.format,
      [2.0, 5.0, 2.0],
      [1.0, 1.0, 1.0],
    );

    let obj = resources::load_mesh("cube.obj", &device, [0.0, 1.0, 0.5])
      .await
      .unwrap();

    let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("Instance Buffer"),
      size: (std::mem::size_of::<[f32; 25]>() as u64) * MAX_INSTANCES,
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let clear_color = wgpu::Color::BLACK;

    let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

    let render_pipeline_layout: wgpu::PipelineLayout =
      device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout, &light.bind_group_layout],
        push_constant_ranges: &[],
      });

    let render_pipeline = {
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("render/shader/shader.wgsl").into()),
      };
      create_render_pipeline(
        &device,
        &render_pipeline_layout,
        config.format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[mesh::MeshVertex::desc(), InstanceRaw::desc()],
        shader,
      )
    };

    Self {
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      projection,
      camera_uniform,
      camera_buffer,
      camera_bind_group,
      obj,
      instance_count: 0,
      instance_buffer,
      clear_color,
      depth_texture,
      light,
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
      self.depth_texture =
        texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
    }
  }

  pub fn update(
    &mut self,
    camera: &camera::Camera,
    dt: instant::Duration,
    objects: &Vec<Transform>,
  ) {
    self
      .camera_uniform
      .update_view_proj(camera, &self.projection);

    self.queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );

    let instance_data = objects
      .iter()
      .map(InstanceRaw::from_transform)
      .collect::<Vec<_>>();

    self.queue.write_buffer(
      &self.instance_buffer,
      0,
      bytemuck::cast_slice(&instance_data),
    );
    self.instance_count = instance_data.len() as u32;

    let old_position: cgmath::Vector3<_> = self.light.uniform.position.into();
    self.light.uniform.position = (cgmath::Quaternion::from_axis_angle(
      (0.0, 1.0, 0.0).into(),
      cgmath::Deg(60.0 * dt.as_secs_f32()),
    ) * old_position)
      .into();
    self.queue.write_buffer(
      &self.light.buffer,
      0,
      bytemuck::cast_slice(&[self.light.uniform]),
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
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &self.depth_texture.view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
          }),
          stencil_ops: None,
        }),
        occlusion_query_set: None,
        timestamp_writes: None,
      });

      render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

      use mesh::DrawLight;
      render_pass.set_pipeline(&self.light.render_pipeline);
      render_pass.draw_light_mesh(&self.obj, &self.camera_bind_group, &self.light.bind_group);

      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw_mesh_instanced(
        &self.obj,
        0..self.instance_count,
        &self.camera_bind_group,
        &self.light.bind_group,
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

fn create_render_pipeline(
  device: &wgpu::Device,
  layout: &wgpu::PipelineLayout,
  color_format: wgpu::TextureFormat,
  depth_format: Option<wgpu::TextureFormat>,
  vertex_layouts: &[wgpu::VertexBufferLayout],
  shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
  let shader = device.create_shader_module(shader);

  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(layout),
    vertex: wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",
      buffers: vertex_layouts,
    },
    fragment: Some(wgpu::FragmentState {
      module: &shader,
      entry_point: "fs_main",
      targets: &[Some(wgpu::ColorTargetState {
        format: color_format,
        blend: Some(wgpu::BlendState {
          alpha: wgpu::BlendComponent::REPLACE,
          color: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
      })],
    }),
    primitive: wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw,
      cull_mode: Some(wgpu::Face::Back),
      polygon_mode: wgpu::PolygonMode::Fill,
      unclipped_depth: false,
      conservative: false,
    },
    depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
      format,
      depth_write_enabled: true,
      depth_compare: wgpu::CompareFunction::Less,
      stencil: wgpu::StencilState::default(),
      bias: wgpu::DepthBiasState::default(),
    }),
    multisample: wgpu::MultisampleState {
      count: 1,
      mask: !0,
      alpha_to_coverage_enabled: false,
    },
    multiview: None,
  })
}
