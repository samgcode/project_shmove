use cgmath::{Vector3, Zero};

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

  pub fn from_components(position: Option<Vector3<f32>>, rotation: Option<Vector3<f32>>, scale: Option<Vector3<f32>>) -> Self {
    Self {
      position: if let Some(pos) = position { pos } else { Vector3::zero() },
      rotation: if let Some(rot) = rotation { rot } else { Vector3::zero() },
      scale: if let Some(scale) = scale { scale } else { Vector3::unit() },
    }
  }
}

trait Unit {
  fn unit() -> Self;
}

impl Unit for Vector3<f32> {
  fn unit() -> Self {
    Self { x: 1.0, y: 1.0, z: 1.0 }
  }
}
