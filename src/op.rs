use nalgebra::clamp;
use crate::math;

pub fn union(d1: f32, d2: f32) -> f32 {
  d1.min(d2)
}

pub fn subtraction(d1: f32, d2: f32) -> f32 {
  -d1.max(d2)
}

pub fn intersection(d1: f32, d2: f32) -> f32 {
  d1.max(d2)
}

pub fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
  math::mix(d2, d1, h) - k * h * (1.0 - h)
}

pub fn smooth_subtraction(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 - 0.5 * (d2 + d1) / k, 0.0, 1.0);
  math::mix(d2, -d1, h ) + k * h * (1.0 - h)
}

pub fn smooth_intersection(d1: f32, d2: f32, k: f32) -> f32 {
  let h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0.0, 1.0);
  math::mix(d2, d1, h) + k * h * (1.0 - h)
}