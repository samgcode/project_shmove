use wgpu::util::DeviceExt;

use crate::engine::render::{
  create_render_pipeline,
  mesh::{self, Vertex},
  texture,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
  pub position: [f32; 3],
  _padding: u32,
  pub color: [f32; 3],
  _padding2: u32,
}

pub struct Light {
  pub uniform: LightUniform,
  pub buffer: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub render_pipeline: wgpu::RenderPipeline,
}

impl Light {
  pub fn new(
    device: &wgpu::Device,
    camera_layout: &wgpu::BindGroupLayout,
    shader_source: wgpu::ShaderSource,
    tex_format: wgpu::TextureFormat,
    position: [f32; 3],
    color: [f32; 3],
  ) -> Self {
    let uniform = LightUniform {
      position,
      _padding: 0,
      color,
      _padding2: 0,
    };

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Light VB"),
      contents: bytemuck::cast_slice(&[uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
      label: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
      label: None,
    });

    let render_pipeline = {
      let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Light Pipeline Layout"),
        bind_group_layouts: &[camera_layout, &bind_group_layout],
        push_constant_ranges: &[],
      });
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Light Shader"),
        source: shader_source,
      };
      create_render_pipeline(
        &device,
        &layout,
        tex_format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[mesh::MeshVertex::desc()],
        shader,
      )
    };

    Self {
      uniform,
      buffer,
      bind_group,
      bind_group_layout,
      render_pipeline,
    }
  }
}
