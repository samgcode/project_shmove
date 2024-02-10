use cgmath::Zero;
use ncollide3d::{
  na::{self, Isometry3, Translation3, UnitQuaternion},
  query,
  shape::Cuboid,
};

use crate::engine::GameObject;

pub struct CollisionEvent {
  pub colliding: bool,
  pub depth: f32,
  pub normal: cgmath::Vector3<f32>,
}

pub fn colliding(object1: &GameObject, object2: &GameObject) -> CollisionEvent {
  let object1_collider = Cuboid::new(convert_vector(object1.transform.scale));
  let object2_collider = Cuboid::new(convert_vector(object2.transform.scale));

  let object1_iso = get_isometry(object1);
  let object2_iso = get_isometry(object2);

  let contact_query = query::contact(
    &object1_iso,
    &object1_collider,
    &object2_iso,
    &object2_collider,
    0.5,
  );

  if let Some(contact) = contact_query {
    return CollisionEvent {
      colliding: contact.depth > 0.0,
      depth: contact.depth,
      normal: cgmath::Vector3 {
        x: contact.normal.x,
        y: contact.normal.y,
        z: contact.normal.z,
      },
    };
  }
  return CollisionEvent {
    colliding: false,
    depth: 0.0,
    normal: cgmath::Vector3::zero(),
  };
}

fn convert_vector(v: cgmath::Vector3<f32>) -> na::Vector3<f32> {
  let cgmath::Vector3 { x, y, z } = v;
  na::Vector3::<f32>::new(x, y, z)
}

fn get_isometry(obj: &GameObject) -> Isometry3<f32> {
  let position = Translation3::from(convert_vector(obj.transform.position));
  let cgmath::Vector3 { x, y, z } = obj.transform.rotation;
  let rotation = UnitQuaternion::from_euler_angles(x, y, z);

  Isometry3::from_parts(position, rotation)
}
