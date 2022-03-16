use crate::{Light, Shader, Vector3f};

type Sdf = Box<dyn Fn(Vector3f) -> f32>;

pub struct Model {
  sdf: Sdf,
  shader: Shader,
  position: Vector3f,
}

impl Model {
  pub fn new(
    sdf: Sdf,
    shader: Shader,
    position: Vector3f
  ) -> Self {
    Model {
      sdf,
      shader,
      position,
    }
  }

  pub fn distance(&self, position: &Vector3f) -> f32 {
    self.sdf.as_ref()(position - self.position)
  }

  pub fn shade(&self, light: &Light, normal: Vector3f, position: Vector3f) -> Vector3f {
    self.shader.as_ref()(light, normal, position - self.position)
  }
}