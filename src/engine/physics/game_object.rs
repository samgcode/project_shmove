use cgmath::{Vector3, Zero};
use ncollide3d::pipeline::CollisionObjectSlabHandle;

use super::collision::{Collision, CollisionEvent, EventStatus, Tag};

pub struct Transform {
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub scale: Vector3<f32>,
}

impl Transform {
  pub fn default() -> Self {
    Self {
      position: Vector3::zero(),
      rotation: Vector3::zero(),
      scale: Vector3::unit(),
    }
  }

  pub fn from_position(position: Vector3<f32>) -> Self {
    Self {
      position,
      rotation: Vector3::zero(),
      scale: Vector3::unit(),
    }
  }

  pub fn from_components(
    position: Option<Vector3<f32>>,
    rotation: Option<Vector3<f32>>,
    scale: Option<Vector3<f32>>,
  ) -> Self {
    Self {
      position: if let Some(pos) = position {
        pos
      } else {
        Vector3::zero()
      },
      rotation: if let Some(rot) = rotation {
        rot
      } else {
        Vector3::zero()
      },
      scale: if let Some(scale) = scale {
        scale
      } else {
        Vector3::unit()
      },
    }
  }
}

trait Unit {
  fn unit() -> Self;
}

impl Unit for Vector3<f32> {
  fn unit() -> Self {
    Self {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    }
  }
}

pub struct GameObject {
  pub transform: Transform,
  pub color: [f32; 3],
  pub collision_handle: CollisionObjectSlabHandle,
  pub tag: Tag,
  pub collision: CollisionEvent,
}

impl GameObject {
  pub fn new(
    pos: (f32, f32, f32),
    rot: (f32, f32, f32),
    sca: (f32, f32, f32),
    color: [f32; 3],
    tag: Tag,
  ) -> Self {
    let transform = Transform {
      position: cgmath::Vector3::<f32>::new(pos.0, pos.1, pos.2),
      rotation: cgmath::Vector3::<f32>::new(rot.0, rot.1, rot.2),
      scale: cgmath::Vector3::<f32>::new(sca.0, sca.1, sca.2),
    };
    Self {
      transform,
      collision_handle: CollisionObjectSlabHandle(0),
      color,
      tag,
      collision: CollisionEvent {
        status: EventStatus::None,
        depth: 0.0,
        normal: cgmath::Vector3::zero(),
        other_handle: CollisionObjectSlabHandle(0),
        other_tag: Tag::None,
      },
    }
  }

  pub fn register_collision(&mut self, collision: &mut Collision) {
    self.collision_handle = collision.add_collider(&self.transform, &self.tag);
  }
}
