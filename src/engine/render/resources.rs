use anyhow::Ok;
use std::io::{BufReader, Cursor};
use wgpu::util::DeviceExt;

use crate::engine::render::mesh;

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
  let path = std::path::Path::new(env!("OUT_DIR"))
    .join("res")
    .join(file_name);
  let txt = std::fs::read_to_string(path)?;

  Ok(txt)
}

pub async fn load_mesh(
  file_name: &str,
  device: &wgpu::Device,
) -> anyhow::Result<mesh::Mesh> {
  let obj_text = load_string(file_name).await?;
  let obj_cursor = Cursor::new(obj_text);
  let mut obj_reader = BufReader::new(obj_cursor);

  let (models, _obj_materials) = tobj::load_obj_buf_async(
    &mut obj_reader,
    &tobj::LoadOptions {
      triangulate: true,
      single_index: true,
      ..Default::default()
    },
    |p| async move {
      let mat_text = load_string(&p).await.unwrap();
      tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
    },
  )
  .await?;

  let model = models.get(0).unwrap();
  let mesh = {
    let vertices = (0..model.mesh.positions.len() / 3)
      .map(|i| mesh::MeshVertex {
        position: [
          model.mesh.positions[i * 3],
          model.mesh.positions[i * 3 + 1],
          model.mesh.positions[i * 3 + 2],
        ],
        normal: [
          model.mesh.normals[i * 3],
          model.mesh.normals[i * 3 + 1],
          model.mesh.normals[i * 3 + 2],
        ],
      })
      .collect::<Vec<_>>();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some(&format!("{:?} Vertex Buffer", file_name)),
      contents: bytemuck::cast_slice(&vertices),
      usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some(&format!("{:?} Index Buffer", file_name)),
      contents: bytemuck::cast_slice(&model.mesh.indices),
      usage: wgpu::BufferUsages::INDEX,
    });
    mesh::Mesh {
      name: file_name.to_string(),
      vertex_buffer,
      index_buffer,
      num_elements: model.mesh.indices.len() as u32,
      color: [1.0; 3],
    }
  };

  Ok(mesh)
}
