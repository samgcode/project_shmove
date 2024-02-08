use std::ops::Range;

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

pub struct Mesh {
  pub name: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_elements: u32,
  pub color: [f32; 3],
}

pub trait DrawModel<'a> {
  fn draw_mesh_instanced(
    &mut self,
    mesh: &'a Mesh,
    instances: Range<u32>,
    camera_bind_group: &'a wgpu::BindGroup,
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
  ) {
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    self.set_bind_group(0, camera_bind_group, &[]);
    self.draw_indexed(0..mesh.num_elements, 0, instances);
  }
}
