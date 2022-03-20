use nalgebra::clamp;
use crate::Vector3f;

fn mix(x: f32, y: f32, t: f32) -> f32 {
  x + (y - x) * t
}

fn mix_vector3f(lhs: Vector3f, rhs: Vector3f, t: f32) -> Vector3f {
  Vector3f::new(
    mix(rhs.x, lhs.x, t),
    mix(rhs.y, lhs.y, t),
    mix(rhs.z, lhs.z, t),
  )
}

#[derive(Default)]
pub struct Surface {
  pub distance: f32,
  pub color: Vector3f,
}

impl Surface {
  pub fn new(distance: f32, color: Vector3f) -> Self {
    Self {
      distance,
      color
    }
  }

  pub fn union(lhs: Surface, rhs: Surface) -> Self {
    if rhs.distance < lhs.distance {
      return rhs;
    }

    lhs
  }

  pub fn smooth_union(lhs: Surface, rhs: Surface, smoothness: f32) -> Self {
    let interpolation = clamp(0.5 + 0.5 * (rhs.distance - lhs.distance) / smoothness, 0.0, 1.0);

    Surface::new(
      mix(rhs.distance, lhs.distance, interpolation),
      mix_vector3f(rhs.color, lhs.color, interpolation)
    )
  }

  pub fn smooth_subtraction(lhs: Surface, rhs: Surface, smoothness: f32) -> Self {
    let interpolation = clamp(0.5 - 0.5 * (rhs.distance + lhs.distance) / smoothness, 0.0, 1.0);

    Surface::new(
      mix(rhs.distance, -lhs.distance, interpolation),
      mix_vector3f(rhs.color, lhs.color, interpolation)
    )
  }
}