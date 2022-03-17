use std::sync::{Arc, mpsc, Mutex};
use image::{ImageBuffer, Rgb};
use raymarching::{Vector3f, Scene, Model, Light, sdf, op, new_shader};

fn shape(position: Vector3f) -> f32 {
  let d1 = sdf::sphere(position + Vector3f::new(-0.5, 0.0, 0.0), 0.5);
  let d2 = sdf::sphere(position + Vector3f::new(0.0, 0.5, 0.75), 0.7);
  let d3 = op::smooth_subtraction(d2, d1, 0.1);
  let d4 = sdf::sphere(position + Vector3f::new(0.5, 0.0, 0.0), 0.5);
  let d5 = op::smooth_union(d3, d4, 0.2);
  let d6 = sdf::sphere(position + Vector3f::new(0.25, 0.0, 0.25), 0.5);
  let d7 = op::smooth_subtraction(d6, d5, 0.05);

  d7
}

fn main() {
  let width = 1024_u32;
  let height = 1024_u32;

  let render_size = 64_u32;
  let end_x = width / render_size;
  let end_y = height / render_size;

  let thread_size = end_x * end_y;
  let concurrency = 10_u32;
  let mut start_positions = Vec::new();

  for y in 0..end_y {
    for x in 0..end_x {
      start_positions.push((x, y));
    }
  }

  let mut image = ImageBuffer::new(width, height);
  let mut current_thread_count = 0_u32;

  let (tx, rx) = mpsc::channel();

  loop {
    let mut is_close = false;
    let mut cursor_thread_count = 0_u32;

    for _ in 0..concurrency {
      let cloned_tx = mpsc::Sender::clone(&tx);
      let start_position = start_positions[current_thread_count as usize];

      std::thread::spawn(move || {
        let mut scene = Scene::new(width, height);

        scene.models.push(Model::new(Box::new(shape), new_shader(Vector3f::new(1.0, 1.0, 1.0), 0.8, 0.2), Vector3f::new(0.0, 0.0, 1.0)));
        scene.lights.push(Light::new(Vector3f::new(0.0, -1.0, 1.0), 0.8, Vector3f::new(1.0, 0.0, 0.0)));

        let mut result = Vec::new();

        for y in start_position.1*render_size..(start_position.1 + 1)*render_size {
          for x in start_position.0*render_size..(start_position.0 + 1)*render_size {
            let c = scene.at(x, y).iter().map(|it| (it * 255.0) as u8).collect::<Vec<_>>();

            result.push((x, y, Rgb([c[0], c[1], c[2]])));
          }
        }

        cloned_tx.send(result).unwrap();
      });

      cursor_thread_count += 1;
      current_thread_count += 1;

      if thread_size <= current_thread_count {
        is_close = true;
        break;
      }
    }

    for _ in 0..cursor_thread_count {
      for pixel in rx.recv().unwrap() {
        image.put_pixel(pixel.0, pixel.1, pixel.2);
      }
    }

    if is_close {
      break;
    }
  }

  image.save("example.png").unwrap();
}