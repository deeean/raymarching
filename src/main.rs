use std::time::Instant;
use cgmath::{InnerSpace, VectorSpace};
use cgmath::num_traits::clamp;
use image::{ImageBuffer};
use lazy_static::lazy_static;
use threadpool::ThreadPool;
use crate::prelude::{Material, Op, Ray, Sdf, Surface, Utils, Vector2f, Vector3f};

mod prelude;

const WIDTH: u32 = 2048;
const HEIGHT: u32 = 2048;

const MIN_DISTANCE: f32 = 0.0;
const MAX_DISTANCE: f32 = 100.0;
const PRECISION: f32 = 0.001;
const MAX_RAYMARCHING_STEPS: u32 = 256;

const EPSILON: Vector2f = Vector2f::new(0.0005, -0.0005);
const EPSILON_XYY: Vector3f = Vector3f::new(EPSILON.x, EPSILON.y, EPSILON.y);
const EPSILON_YYX: Vector3f = Vector3f::new(EPSILON.y, EPSILON.y, EPSILON.x);
const EPSILON_YXY: Vector3f = Vector3f::new(EPSILON.y, EPSILON.x, EPSILON.y);
const EPSILON_XXX: Vector3f = Vector3f::new(EPSILON.x, EPSILON.x, EPSILON.x);

lazy_static! {
  static ref LAPIS_LAZULI: Material = Material::new(
    Vector3f::new(0.0, 0.14901961, 0.25098039),
    Vector3f::new(0.14901960784313725, 0.3803921568627451, 0.611764705882353),
    Vector3f::new(1.0, 1.0, 1.0),
    64.0
  );
}

fn scene(position: &Vector3f) -> Surface {
  let floor = Surface::new(position.y + 1.0, Material::checkerboard(position));

  let gold_sphere = Surface::new(Sdf::sphere(position, 1.0), *LAPIS_LAZULI);

  Op::union(floor, gold_sphere)
}

fn raymarch(ray: &Ray) -> Surface {
  let mut depth = MIN_DISTANCE;
  let mut surface = Surface::default();

  for _ in 0..MAX_RAYMARCHING_STEPS {
    let position = ray.origin + depth * ray.direction;
    surface = scene(&position);
    depth += surface.distance;

    if surface.distance < PRECISION || depth > MAX_DISTANCE {
      break;
    }
  }

  surface.distance = depth;

  surface
}

fn calc_normal(position: &Vector3f) -> Vector3f {
  return (
    EPSILON_XYY * scene(&(position + EPSILON_XYY)).distance +
    EPSILON_YYX * scene(&(position + EPSILON_YYX)).distance +
    EPSILON_YXY * scene(&(position + EPSILON_YXY)).distance +
    EPSILON_XXX * scene(&(position + EPSILON_XXX)).distance
  ).normalize();
}

fn phong(light_dir: &Vector3f, normal: &Vector3f, ray_dir: &Vector3f, material: &Material) -> Vector3f {
  let ln = clamp(light_dir.dot(*normal), 0.0, 1.0);
  let diffuse = material.diffuse * ln;

  let rv = clamp(Utils::reflect(light_dir, normal).dot(ray_dir * -1.0), 0.0, 1.0).powf(material.shininess);
  let specular = material.specular * rv;

  material.ambient + diffuse + specular
}

fn get_pixel_color(uv: &Vector2f) -> Vector3f {
  let ray = Ray::new(Vector3f::new(0.0, 0.0, 5.0), Vector3f::new(uv.x, uv.y, -1.0).normalize());
  let surface = raymarch(&ray);
  let mut color = Vector3f::new(1.0, 0.341, 0.2)
    .lerp(
      Vector3f::new(0.0, 1.0, 1.0),
      uv.y
    ) * 1.6;

  if surface.distance > MAX_DISTANCE {
    return color;
  } else {
    let position = ray.origin + ray.direction * surface.distance;
    let normal = calc_normal(&position);

    let light_dir1 = (Vector3f::new(8.0, 2.0, -10.0) - position).normalize();
    let light_dir2 = (Vector3f::new(1.0, 1.0, 1.0) - position).normalize();
    let light_dir3 = (Vector3f::new(1.0, 2.0, 10.0) - position).normalize();

    color = 0.9 * phong(&light_dir1, &normal, &ray.direction, &surface.material);
    color += 0.5 * phong(&light_dir2, &normal, &ray.direction, &surface.material);
    color += 0.5 * phong(&light_dir3, &normal, &ray.direction, &surface.material);
  }

  color
}

fn main() {
  let now = Instant::now();
  let workers = 10;
  let side = 32;

  let pool = ThreadPool::new(workers);
  let (tx, rx) = std::sync::mpsc::channel();

  for sy in 0..HEIGHT / side {
    for sx in 0..WIDTH / side {
      let tx = tx.clone();

      pool.execute(move || {
        let mut buf = Vec::new();

        for y in sy * side..(sy + 1) * side {
          for x in sx * side..(sx + 1) * side {
            let uv = (Vector2f::new(x as f32, (HEIGHT - y) as f32) - 0.5 * Vector2f::new(WIDTH as f32, HEIGHT as f32)) / HEIGHT as f32;
            let color = get_pixel_color(&uv);
            buf.push((x, y, Utils::rgb(&color)));
          }
        }

        tx.send(buf).unwrap();
      });
    }
  }

  let tasks = (WIDTH / side * HEIGHT / side) as usize;

  let mut image = ImageBuffer::new(WIDTH, HEIGHT);

  for pixels in rx.iter().take(tasks) {
    for (x, y, c) in pixels {
      image.put_pixel(x, y, c);
    }
  }

  image.save("output.png").unwrap();

  println!("{:?}", now.elapsed());
}