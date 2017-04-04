use point::Point;
use vector::Vector3;
use rendering::{Intersectable, Ray};
use std::ops::{Mul, Add};
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
impl Mul for Color {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    Color {
      red: self.red * other.red,
      green: self.green * other.green,
      blue: self.blue * other.blue,
    }
  }
}
impl Mul<f32> for Color {
  type Output = Color;

  fn mul(self, other: f32) -> Color {
    Color {
      red: self.red * other,
      green: self.green * other,
      blue: self.blue * other,
    }
  }
}
impl Mul<Color> for f32 {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    other * self
  }
}
impl Add for Color {
  type Output = Color;

  fn add(self, other: Color) -> Color {
    Color {
      red: self.red + other.red,
      green: self.green + other.green,
      blue: self.blue + other.blue,
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Light {
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32,
}


#[derive(Deserialize, Debug)]
pub struct Plane {
  pub origin: Point,
  #[serde(deserialize_with="Vector3::deserialize_normalized")]
  pub normal: Vector3,
  pub color: Color,
  pub albedo: f32,
}


#[derive(Deserialize, Debug)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub color: Color,
  pub albedo: f32,
}


#[derive(Deserialize, Debug)]
pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}
impl Element {
  pub fn color(&self) -> &Color {
    match *self {
      Element::Sphere(ref s) => &s.color,
      Element::Plane(ref p) => &p.color,
    }
  }

  pub fn albedo(&self) -> f32 {
    match *self {
      Element::Sphere(ref s) => s.albedo,
      Element::Plane(ref p) => p.albedo,
    }
  }
}


#[derive(Deserialize, Debug)]
pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub elements: Vec<Element>,
  pub light: Light,
}
impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self.elements
      .iter()
      .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}


pub struct Intersection<'a> {
  pub distance: f64,
  pub object: &'a Element,

  _secret: (),
}
impl<'a> Intersection<'a> {
  pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
    if !distance.is_finite() {
      panic!("Intersection must have a finite distance.");
    }

    Intersection {
      distance: distance,
      object: object,
      _secret: (),
    }
  }
}
