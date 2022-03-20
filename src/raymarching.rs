use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;
use nalgebra::clamp;
use threadpool::ThreadPool;
use crate::{sdf, Surface, Vector2f, Vector3f, Vector4f};

const WIDTH: u32 = 2048;
const HEIGHT: u32 = 2048;
const RESOLUTION: Vector2f = Vector2f::new(WIDTH as f32, HEIGHT as f32);

const EPSILON: Vector2f = Vector2f::new(0.0005, -0.0005);
const PRECISION: f32 = 0.001;

const MAX_DIST: f32 = 100.0;
const MIN_DIST: f32 = 0.0;
const MAX_STEPS: u32 = 256;

const LIGHT_POSITION: Vector3f = Vector3f::new(5.0, 7.5, 10.0);
const CAMERA_POSITION: Vector3f = Vector3f::new(0.0, 0.0, 3.0);

const BACKGROUND_COLOR: Vector3f = Vector3f::new(1.0, 1.0, 1.0);

const GAMMA: f32 = 1.0 / 2.2;


fn scene(position: Vector3f) -> Surface {
  let sphere1 = sdf::sphere(position + Vector3f::new(0.5, 0.0, 0.0), 0.3, Vector3f::new(1.0, 0.0, 0.0));
  let sphere2 = sdf::sphere(position + Vector3f::new(-0.5, 0.0, 0.0), 0.3, Vector3f::new(0.0, 0.0, 1.0));

  let floor_color = 1.0 + 0.5 * ((f32::floor(position.x) + f32::floor(position.z)) % 2.0);
  let floor = sdf::floor(position, Vector3f::new(floor_color, floor_color, floor_color));

  let mut shape = Surface::union(sphere1, sphere2);

  Surface::union(floor, shape)
}

fn calc_normal(position: &Vector3f) -> Vector3f {
  (
    EPSILON.xyy() * scene(position + EPSILON.xyy()).distance +
    EPSILON.yyx() * scene(position + EPSILON.yyx()).distance +
    EPSILON.yxy() * scene(position + EPSILON.yxy()).distance +
    EPSILON.xxx() * scene(position + EPSILON.xxx()).distance
  ).normalize()
}

fn ray_march(ray_origin: &Vector3f, ray_dir: &Vector3f) -> Surface {
  let mut distance = MIN_DIST;
  let mut result = Surface::default();

  for _ in 0..MAX_STEPS {
    let position = ray_origin + distance * ray_dir;
    result = scene(position);
    distance += result.distance;
    if PRECISION > result.distance || MAX_DIST < distance {
      break;
    }
  }

  result.distance = distance;

  result
}

fn at(x: f32, y: f32) -> Vector3f {
  let position = Vector2f::new(x, HEIGHT as f32 - y) - 0.5 * RESOLUTION;
  let uv = position / RESOLUTION.y;

  let ray_origin = CAMERA_POSITION.clone();
  let ray_dir = Vector3f::new(uv.x, uv.y, -1.0);

  let surface = ray_march(&ray_origin, &ray_dir);
  let mut result = BACKGROUND_COLOR.clone();

  if MAX_DIST < surface.distance {
    return result;
  } else {
    let position = ray_origin + ray_dir * surface.distance;
    let normal = calc_normal(&position);
    let light_dir = (LIGHT_POSITION - position).normalize();

    let mut diff = clamp(normal.dot(&light_dir), 0.3, 1.0);

    let new_ray_origin = position + normal * PRECISION * 2.0;
    let shadow = ray_march(&new_ray_origin, &light_dir);
    if shadow.distance < (LIGHT_POSITION - new_ray_origin).magnitude() {
      diff *= 0.2;
    }

    result = diff * surface.color;
  }

  result = Vector3f::new(
    f32::powf(result.x, GAMMA),
    f32::powf(result.y, GAMMA),
    f32::powf(result.z, GAMMA),
  );

  result
}

pub fn render() {
  let now = Instant::now();
  let workers = 10;
  let jobs = 64;

  let pool = ThreadPool::new(workers);

  let (tx, rx) = channel();
  let mut pixels = vec![0_u8; (WIDTH * HEIGHT * 4) as usize];
  let mut positions = Vec::new();

  for y in 0..WIDTH {
    for x in 0..HEIGHT {
      positions.push((x as f32, y as f32));
    }
  }

  let size = positions.len() / jobs;

  for i in 0..jobs {
    let tx = tx.clone();
    let positions = positions.clone();

    pool.execute(move || {
      println!("Start Job: {}", i);

      let mut buf = Vec::new();

      for j in i * size..(i + 1) * size {
        let (x, y) = positions[j];
        let color = at(x, y);

        buf.push((color[0] * 255.0) as u8);
        buf.push((color[1] * 255.0) as u8);
        buf.push((color[2] * 255.0) as u8);
        buf.push(255_u8);
      }

      tx.send((i, buf)).unwrap();
    });
  }

  for (i, buffer) in rx.iter().take(jobs) {
    let start_index = i * size * 4;

    for (buf_index, buf) in buffer.iter().enumerate() {
      let pixel_index = start_index + buf_index;
      pixels[pixel_index] = *buf;
    }
  }

  image::save_buffer("output.png", &pixels, WIDTH, HEIGHT, image::ColorType::Rgba8).unwrap();

  println!("{:?}", now.elapsed());
}