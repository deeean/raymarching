use image::{ImageBuffer, Rgb};
use raymarching::{Vector3f, Scene, Model, Light, sdf, op, new_shader};

fn shape(position: Vector3f) -> f32 {
  let d1 = sdf::sphere(position + Vector3f::new(-0.5, 0.0, 0.0), 0.5);
  let d2 = sdf::sphere(position + Vector3f::new(0.0, 0.5, 0.75), 0.7);
  let d3 = op::smooth_subtraction(d2, d1, 0.1);
  let d4 = sdf::sphere(position + Vector3f::new(0.5, 0.0, 0.0), 0.5);
  let d5 = op::smooth_union(d3, d4, 0.4);

  d5
}

fn main() {
  let width = 64_u32;
  let height = 64_u32;

  let mut scene = Scene::new(width, height);

  scene.models.push(Model::new(Box::new(shape), new_shader(Vector3f::new(1.0, 1.0, 1.0), 0.8, 0.2), Vector3f::new(0.0, 0.0, 1.0)));
  scene.lights.push(Light::new(Vector3f::new(0.0, -1.0, 1.0), 0.8, Vector3f::new(1.0, 0.0, 0.0)));

  let mut image = ImageBuffer::new(width, height);

  for y in 0..height {
    for x in 0..width {
      let color = scene.at(x, y).iter().map(|it| (it * 255.0) as u8).collect::<Vec<_>>();

      image.put_pixel(x, y, Rgb([color[0], color[1], color[2]]));
    }
  }

  image.save("output.png").unwrap();
}