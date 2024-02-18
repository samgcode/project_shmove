use std::ops::{Mul, Range};

use cgmath::Vector4;
use wgpu::{util::DeviceExt, Device};

pub trait Vertex {
  fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
}

impl Vertex for MeshVertex {
  fn desc() -> wgpu::VertexBufferLayout<'static> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x3,
        },
      ],
    }
  }
}

pub struct Mesh4d {
  pub name: String,
  pub vertices: Vec<Vector4<f32>>,
  pub normals: Vec<[f32; 3]>,
  pub indices: Vec<u32>,
  pub angle: f32,
}

impl Mesh4d {
  pub fn project(&self, device: &Device) -> Mesh {
    let focal_length = 100.0;

    let angle = self.angle * 3.1415/180.0;
    #[rustfmt::skip]
    let rotation1 = cgmath::Matrix4::new(
      1.0, 0.0, 0.0, 0.0, 
      0.0, f32::cos(angle*0.5), 0.0, f32::sin(angle*0.5), 
      0.0, 0.0, 1.0, 0.0, 
      0.0, -f32::sin(angle*0.5), 0.0, f32::cos(angle*0.5)
    );
    #[rustfmt::skip]
    let rotation2 = cgmath::Matrix4::new(
      f32::cos(angle), 0.0, 0.0, f32::sin(angle), 
      0.0, 1.0, 0.0, 0.0, 
      0.0, 0.0, 1.0, 0.0, 
      -f32::sin(angle), 0.0, 0.0, f32::cos(angle)
    );
    #[rustfmt::skip]
    let rotation3 = cgmath::Matrix4::new(
      1.0, 0.0, 0.0, 0.0, 
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, f32::cos(angle * 0.75), f32::sin(angle * 0.75), 
      0.0, 0.0, -f32::sin(angle * 0.75), f32::cos(angle * 0.75), 
    );

    let vertices = self
      .vertices
      .iter()
      .enumerate()
      .map(|(i, v)| {
        // let vertex = v;
        let vertex = rotation1.mul(rotation2).mul(rotation3).mul(v);
        MeshVertex {
          position: [
            (focal_length * vertex.x) / (focal_length + vertex.w),
            (focal_length * vertex.y) / (focal_length + vertex.w),
            (focal_length * vertex.z) / (focal_length + vertex.w),
          ],
          normal: self.normals[i],
        }
      })
      .collect::<Vec<_>>();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some(&format!("{:?} Vertex Buffer", self.name)),
      contents: bytemuck::cast_slice(&vertices),
      usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some(&format!("{:?} Index Buffer", self.name)),
      contents: bytemuck::cast_slice(&self.indices),
      usage: wgpu::BufferUsages::INDEX,
    });
    Mesh {
      name: self.name.to_string(),
      vertex_buffer,
      index_buffer,
      num_elements: self.indices.clone().len() as u32,
    }
  }
}

pub struct Mesh {
  pub name: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_elements: u32,
}

pub trait DrawModel<'a> {
  fn draw_mesh_instanced(
    &mut self,
    mesh: &'a Mesh,
    instances: Range<u32>,
    camera_bind_group: &'a wgpu::BindGroup,
    light_bind_group: &'a wgpu::BindGroup,
  );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
  'b: 'a,
{
  fn draw_mesh_instanced(
    &mut self,
    mesh: &'b Mesh,
    instances: Range<u32>,
    camera_bind_group: &'b wgpu::BindGroup,
    light_bind_group: &'a wgpu::BindGroup,
  ) {
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    self.set_bind_group(0, camera_bind_group, &[]);
    self.set_bind_group(1, light_bind_group, &[]);
    self.draw_indexed(0..mesh.num_elements, 0, instances);
  }
}

pub trait DrawLight<'a> {
  fn draw_light_mesh(
    &mut self,
    mesh: &'a Mesh,
    camera_bind_group: &'a wgpu::BindGroup,
    light_bind_group: &'a wgpu::BindGroup,
  );
  fn draw_light_mesh_instanced(
    &mut self,
    mesh: &'a Mesh,
    instances: Range<u32>,
    camera_bind_group: &'a wgpu::BindGroup,
    light_bind_group: &'a wgpu::BindGroup,
  );
}

impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a>
where
  'b: 'a,
{
  fn draw_light_mesh(
    &mut self,
    mesh: &'b Mesh,
    camera_bind_group: &'b wgpu::BindGroup,
    light_bind_group: &'b wgpu::BindGroup,
  ) {
    self.draw_light_mesh_instanced(mesh, 0..1, camera_bind_group, light_bind_group);
  }
  fn draw_light_mesh_instanced(
    &mut self,
    mesh: &'b Mesh,
    instances: Range<u32>,
    camera_bind_group: &'b wgpu::BindGroup,
    light_bind_group: &'b wgpu::BindGroup,
  ) {
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    self.set_bind_group(0, camera_bind_group, &[]);
    self.set_bind_group(1, light_bind_group, &[]);
    self.draw_indexed(0..mesh.num_elements, 0, instances.clone());
  }
}
