use crate::{Light, Model, Vector3f};

pub struct Scene {
  camera: Vector3f,
  width: u32,
  height: u32,
  pub models: Vec<Model>,
  pub lights: Vec<Light>,
}

impl Scene {
  pub fn new(width: u32, height: u32) -> Self {
    Scene {
      width,
      height,
      camera: Vector3f::new(0.0, 0.0, -5.0),
      models: Vec::new(),
      lights: Vec::new(),
    }
  }

  pub fn at(&self, x: u32, y: u32) -> Vector3f {
    let position = Vector3f::new(x as f32, y as f32, 0.0);
    let scale = -1.0 * 2.0 * (1.0 / self.height as f32);
    let uv = (position - Vector3f::new(1.0 / 2.0 * self.width as f32, 1.0 / 2.0 * self.height as f32, 0.0)) * scale;
    let ray = (uv - self.camera).normalize();
    let mut curr = self.camera.clone();

    let step = 64;
    let epsilon = 0.01;

    for _ in 0..step {
      let mut dist = self.models
        .iter()
        .enumerate()
        .map(|(key, it)| (key, it.distance(&curr)))
        .collect::<Vec<_>>();

      dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

      if dist[0].1 < epsilon {
        let h = 0.001_f32;
        let d = dist[0].1;
        let m = &self.models[dist[0].0];
        let normal = Vector3f::new(
          (m.distance(&(curr + Vector3f::new(h, 0.0, 0.0))) - d) / h,
          (m.distance(&(curr + Vector3f::new(0.0, h, 0.0))) - d) / h,
          (m.distance(&(curr + Vector3f::new(0.0, 0.0, h))) - d) / h,
        );

        let mut color = Vector3f::new(0.0, 0.0, 0.0);

        for light in &self.lights {
          let light_color = m.shade(light, normal, curr);
          color += light_color;
        }

        return color;
      }

      curr = curr + ray * dist[0].1;
    }

    Vector3f::new(1.0, 1.0, 1.0)
  }
}
