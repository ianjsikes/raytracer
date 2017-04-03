extern crate image;

pub mod scene;
pub mod vector;
pub mod point;
mod rendering;

use scene::Scene;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

use rendering::{Ray, cast_ray};

pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.width, scene.height)
  let black = Rgba::from_channels(0, 0, 0, 0);
  for x in 0..scene.width {
    for y in 0..scene.height {
      let ray = Ray::create_prime(x, y, scene);

      if scene.sphere.intersect(&ray) {
        image.put_pixel(x, y, to_rgba(&scene.sphere.color))
      } else {
        image.put_pixel(x, y, black);
      }
    }
  }
  image
}

pub fn render_into(scene: &Scene, image: &mut ImageBuffer<Rgba<u8>, &mut [u8]>) {
  for y in 0..scene.height {
    for x in 0..scene.width {
      let ray = Ray::create_prime(x, y, scene);
      image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
    }
  }
}

#[test]
fn test_can_render_scene() {
  let scene = Scene {
    width: 800,
    height: 600,
    fov: 90.0,
    sphere: Sphere {
      center: Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
      },
      radius: 1.0,
      color: Color {
        red: 0.4,
        green: 1.0,
        blue: 0.4,
      },
    },
  };

  let img: DynamicImage = render(&scene);
  assert_eq!(scene.width, img.width());
  assert_eq!(scene.height, img.height());
}
