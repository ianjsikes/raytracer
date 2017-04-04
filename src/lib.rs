#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate serde;

pub mod scene;
pub mod vector;
pub mod point;
mod rendering;

use scene::{Scene};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

use rendering::{Ray, cast_ray};

pub fn render(scene: &Scene) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
  for x in 0..scene.width {
    for y in 0..scene.height {
      let ray = Ray::create_prime(x, y, scene);
      image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
      // match scene.trace(&ray) {
      //   Option::Some(intersection) => image.put_pixel(x, y, get_color(scene, &ray, &intersection).to_rgba()),
      //   Option::None => image.put_pixel(x, y, BLACK.to_rgba()),
      // }
    }
  }
  image
}

pub fn render_into(scene: &Scene, image: &mut ImageBuffer<Rgba<u8>, &mut [u8]>) {
  for y in 0..scene.height {
    for x in 0..scene.width {
      let ray = Ray::create_prime(x, y, scene);
      image.put_pixel(x, y, cast_ray(scene, &ray, 0).to_rgba());
      // match scene.trace(&ray) {
      //   Option::Some(intersection) => image.put_pixel(x, y, get_color(scene, &ray, &intersection).to_rgba()),
      //   Option::None => image.put_pixel(x, y, BLACK.to_rgba()),
      // }
    }
  }
}

// fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
//   let hit_point = ray.origin + (ray.direction * intersection.distance);
//   let surface_normal = intersection.object.surface_normal(&hit_point);
//   let direction_to_light = -scene.light.direction.normalize();
//   let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * scene.light.intensity;
//   let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

//   let color = intersection.object.color().clone() * scene.light.color.clone() * light_power * light_reflected;
//   color.clamp()
// }

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
