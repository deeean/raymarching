use crate::{Vector2f, Vector3f};

type Primitive = fn(Vector3f) -> f32;

fn displacement(p: Vector3f, a: f32) -> f32 {
  (p.x * a).sin() * (p.y * a).sin() * (p.z * a).sin()
}

pub fn sphere(p: Vector3f, radius: f32) -> f32 {
  p.magnitude() - radius
}

pub fn torus(p: Vector3f, t: Vector2f) -> f32 {
  let q = Vector2f::new(p.xz().magnitude() - t.x, p.y);
  q.magnitude() - t.y
}

pub fn displace(s: Primitive, p: Vector3f, a: f32) -> f32 {
  let d1 = (s)(p);
  let d2 = displacement(p, a);

  d1 + d2
}