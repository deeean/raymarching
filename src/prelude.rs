use cgmath::{InnerSpace, Vector2, Vector3, Zero};
use image::Rgb;

pub type Vector2f = Vector2<f32>;

pub type Vector3f = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
  pub origin: Vector3f,
  pub direction: Vector3f
}

impl Ray {
  pub fn new(origin: Vector3f, direction: Vector3f) -> Self {
    Self {
      origin,
      direction
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
  pub ambient: Vector3f,
  pub diffuse: Vector3f,
  pub specular: Vector3f,
  pub shininess: f32,
}

impl Material {
  pub fn new(ambient: Vector3f, diffuse: Vector3f, specular: Vector3f, shininess: f32) -> Self {
    Self {
      ambient,
      diffuse,
      specular,
      shininess,
    }
  }

  pub fn checkerboard(position: &Vector3f) -> Self {
    let a = (1.0 + 0.7 * ((f32::floor(position.x) + f32::floor(position.z)) % 2.0)) * 0.3;

    Self {
      ambient: Vector3f::new(a, a, a),
      diffuse: Vector3f::new(0.3, 0.3, 0.3),
      specular: Vector3::zero(),
      shininess: 1.0
    }
  }
}

impl Default for Material {
  fn default() -> Self {
    Self {
      ambient: Vector3f::zero(),
      diffuse: Vector3f::zero(),
      specular: Vector3f::zero(),
      shininess: 0.0,
    }
  }
}

#[derive(Default, Debug)]
pub struct Surface {
  pub distance: f32,
  pub material: Material,
}

impl Surface {
  pub fn new(distance: f32, material: Material) -> Self {
    Self {
      distance,
      material,
    }
  }
}

pub struct Sdf {

}

impl Sdf {
  pub fn sphere(position: &Vector3f, radius: f32) -> f32 {
    position.magnitude() - radius
  }
}

pub struct Op {

}

impl Op {
  pub fn union(a: Surface, b: Surface) -> Surface {
    if b.distance < a.distance {
      return b;
    }

    a
  }
}

pub struct Utils {

}

impl Utils {
  pub fn reflect(in_dir: &Vector3f, in_normal: &Vector3f) -> Vector3f {
    let factor = -2.0 * in_dir.dot(*in_normal);

    return Vector3f::new(
      factor * in_normal.x + in_dir.x,
      factor * in_normal.y + in_dir.y,
      factor * in_normal.z + in_dir.z,
    )
  }

  pub fn rgb(v: &Vector3f) -> Rgb<u8> {
    Rgb([(v.x * 255.0) as u8, (v.y * 255.0) as u8, (v.z * 255.0) as u8])
  }
}