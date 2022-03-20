use crate::{Surface, Vector2f, Vector3f, Vector4f};

pub fn sphere(position: Vector3f, radius: f32, color: Vector3f) -> Surface {
  Surface::new(position.magnitude() - radius, color)
}

pub fn torus(position: Vector3f, size: Vector2f, color: Vector3f) -> Surface {
  let q = Vector2f::new(position.xz().magnitude() - size.x, position.y);
  Surface::new(q.magnitude() - size.y, color)
}

pub fn floor(position: Vector3f, color: Vector3f) -> Surface {
  Surface::new(position.y + 1.0, color)
}