use crate::Vector3f;

#[derive(Copy, Clone)]
pub struct Light {
  pub position: Vector3f,
  pub intensity: f32,
  pub color: Vector3f,
}

impl Light {
  pub fn new(position: Vector3f, intensity: f32, color: Vector3f) -> Self {
    Light {
      position,
      intensity,
      color,
    }
  }
}