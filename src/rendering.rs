use point::Point;
use vector::Vector3;
use scene::{Scene, Element, Sphere, Plane, Color, Intersection};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector3,
}

pub trait Intersectable {
  fn intersect(&self, ray: &Ray) -> Option<f64>;

  fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

impl Intersectable for Element {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    match *self {
      Element::Sphere(ref s) => s.intersect(ray),
      Element::Plane(ref p) => p.intersect(ray),
    }
  }

  fn surface_normal(&self, hit_point: &Point) -> Vector3 {
    match *self {
      Element::Sphere(ref s) => s.surface_normal(hit_point),
      Element::Plane(ref p) => p.surface_normal(hit_point),
    }
  }
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    let l: Vector3 = self.center - ray.origin;
    let adj = l.dot(&ray.direction);
    let d2 = l.dot(&l) - (adj * adj);
    let radius2 = self.radius * self.radius;

    if d2 > radius2 {
      return None;
    }
    
    let thc = (radius2 - d2).sqrt();
    let t0 = adj - thc;
    let t1 = adj + thc;

    if t0 < 0.0 && t1 < 0.0 {
      return None;
    }

    let distance = if t0 < t1 { t0 } else { t1 };
    Some(distance)
  }

  fn surface_normal(&self, hit_point: &Point) -> Vector3 {
    (*hit_point - self.center).normalize()
  }
}

impl Intersectable for Plane {
  fn intersect(&self, ray: &Ray) -> Option<f64> {
    let normal = &self.normal;
    let denom = normal.dot(&ray.direction);
    if denom > 1e-6 {
      let v = self.origin - ray.origin;
      let distance = v.dot(&normal) / denom;
      if distance >= 0.0 {
        return Some(distance);
      }
    }
    None
  }

  fn surface_normal(&self, _: &Point) -> Vector3 {
    -self.normal
  }
}

impl Ray {
  pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
    assert!(scene.width > scene.height);
    let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);
    let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
    let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
    
    Ray {
      origin: Point::zero(),
      direction: Vector3 {
        x: sensor_x,
        y: sensor_y,
        z: -1.0,
      }
      .normalize(),
    }
  }
}

pub const BLACK: Color = Color {
  red: 0.0,
  green: 0.0,
  blue: 0.0,
};

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
  if depth >= scene.max_recursion_depth {
    return BLACK;
  }

  let intersection = scene.trace(&ray);
  intersection.map(|i| get_color(scene, &ray, &i))
    .unwrap_or(BLACK)
}

// fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
//   let hit = ray.origin + (ray.direction * intersection.distance);
//   let normal = intersection.object.surface_normal(&hit);

//   shade_diffuse(scene, intersection.object, hit, normal)
// }

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
  let mut color = BLACK;
  for light in &scene.lights {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);
    let direction_to_light = light.direction_from(&hit_point);

    let shadow_ray = Ray {
      origin: hit_point + (surface_normal * scene.shadow_bias),
      direction: direction_to_light,
    };
    let shadow_intersection = scene.trace(&shadow_ray);
    let in_light = shadow_intersection.is_none() ||
                   shadow_intersection.unwrap().distance > light.distance(&hit_point);

    let light_intensity = if in_light {
      light.intensity(&hit_point)
    } else {
      0.0
    };
    let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                      light_intensity;
    let light_reflected = intersection.object.albedo() / PI;

    let light_color = intersection.object.color().clone() * light.color() * light_power * light_reflected;
    color = color + light_color;
  }
  color.clamp()
}

// fn shade_diffuse(scene: &Scene,
//                  element: &Element,
//                  hit_point: Point,
//                  surface_normal: Vector3)
//                  -> Color {
//   // let texture_coords = element.texture_coords(&hit_point);
//   let mut color = BLACK;
//   for light in &scene.lights {
//     let direction_to_light = light.direction_from(&hit_point);

//     let shadow_ray = Ray {
//       origin: hit_point + (surface_normal * scene.shadow_bias),
//       direction: direction_to_light,
//     };
//     let shadow_intersection = scene.trace(&shadow_ray);
//     let in_light = shadow_intersection.is_none() ||
//                    shadow_intersection.unwrap().distance > light.distance(&hit_point);
    
//     let light_intensity = if in_light {
//       light.intensity(&hit_point)
//     } else {
//       0.0
//     };
//     let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
//     let light_reflected = element.albedo() / ::std::f32::consts::PI;

//     let light_color = light.color * light_power * light_reflected;
//     color = color + light_color;
//   }
//   color.clamp()
// }
