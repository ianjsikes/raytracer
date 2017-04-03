use point::Point;
use image::{Rgba, Pixel};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
  linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
  encoded.powf(GAMMA)
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}
impl Color {
  pub fn clamp(&self) -> Color {
    Color {
      red: self.red.min(1.0).max(0.0),
      green: self.green.min(1.0).max(0.0),
      blue: self.blue.min(1.0).max(0.0),
    }
  }

  pub fn to_rgba(&self) -> Rgba<u8> {
    Rgba::from_channels((gamma_encode(self.red) * 255.0) as u8,
                        (gamma_encode(self.green) * 255.0) as u8,
                        (gamma_encode(self.blue) * 255.0) as u8,
                        255)
  }

  pub fn from_rgba(rgba: Rgba<u8>) -> Color {
    Color {
      red: gamma_decode((rgba.data[0] as f32) / 255.0),
      green: gamma_decode((rgba.data[1] as f32) / 255.0),
      blue: gamma_decode((rgba.data[2] as f32) / 255.0),
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub sphere: Sphere,
}
impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self.spheres
      .iter()
      .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}

pub struct Intersection<'a> {
  pub distance: f64,
  pub object: &'a Sphere,
}
impl<'a> Intersection<'a> {
  pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
    distance: distance,
    object: object,
  }
}
