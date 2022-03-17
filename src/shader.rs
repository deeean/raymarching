use crate::{Light, Vector3f};

pub type Shader = Box<dyn Fn(&Light, Vector3f, Vector3f) -> Vector3f + Send + 'static>;

pub fn new_shader(color: Vector3f, diffuse_factor: f32, ambient_factor: f32) -> Shader {
  Box::new(move |light, normal, position| {
    let light_dir = ((light.position - position) * -1.0).normalize();
    let brightness = light_dir.dot(&normal) * light.intensity;
    let light_color = light.color * brightness;
    let diffuse = Vector3f::new(color.x * light_color.x, color.y * light_color.y, color.z * light_color.z) * diffuse_factor;

    diffuse + color * ambient_factor
  })
}