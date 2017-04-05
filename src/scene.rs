use point::Point;
use vector::Vector3;
use rendering::{Intersectable, Ray, TextureCoords};
use std::ops::{Mul, Add};
use image::{Rgba, Pixel, DynamicImage, GenericImage};
use serde::{Deserialize, Deserializer};
use image;
use std::fmt;
use std::path::PathBuf;


// 
// COLOR
//

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


//
// LIGHT
//

#[derive(Deserialize, Debug)]
pub enum Light {
  Directional(DirectionalLight),
  Spherical(SphericalLight),
}
impl Light {
  pub fn color(&self) -> Color {
    match *self {
      Light::Directional(ref d) => d.color,
      Light::Spherical(ref s) => s.color,
    }
  }

  pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
    match *self {
      Light::Directional(ref d) => -d.direction,
      Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
    }
  }

  pub fn intensity(&self, hit_point: &Point) -> f32 {
    match *self {
      Light::Directional(ref d) => d.intensity,
      Light::Spherical(ref s) => {
        let r2 = (s.position - *hit_point).norm() as f32;
        s.intensity / (4.0 * ::std::f32::consts::PI * r2)
      }
    }
  }

  pub fn distance(&self, hit_point: &Point) -> f64 {
    match *self {
      Light::Directional(_) => ::std::f64::INFINITY,
      Light::Spherical(ref s) => (s.position - *hit_point).length(),
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct DirectionalLight {
  #[serde(deserialize_with="Vector3::deserialize_normalized")]
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32,
}

#[derive(Deserialize, Debug)]
pub struct SphericalLight {
  pub position: Point,
  pub color: Color,
  pub intensity: f32,
}


//
// MATERIAL
//

#[derive(Deserialize, Debug)]
pub enum SurfaceType {
  Diffuse,
  Reflective { reflectivity: f32 },
  Refractive { index: f32, transparency: f32 },
}

#[derive(Deserialize, Debug)]
pub struct Material {
  pub coloration: Coloration,
  pub albedo: f32,
  pub surface: SurfaceType,
}

pub fn load_texture<D>(deserializer: D) -> Result<DynamicImage, D::Error>
  where D: Deserializer
{
  let path = PathBuf::deserialize(deserializer)?;
  Ok(image::open(path).expect("Unable to open texture file"))
}

#[derive(Deserialize)]
pub enum Coloration {
  Color(Color),
  Texture(#[serde(deserialize_with="load_texture")]
            DynamicImage),
}
impl fmt::Debug for Coloration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Coloration::Color(ref c) => write!(f, "Color({:?})", c),
      Coloration::Texture(_) => write!(f, "Texture"),
    }
  }
}
impl Coloration {
  pub fn color(&self, texture_coords: &TextureCoords) -> Color {
    match *self {
      Coloration::Color(c) => c,
      Coloration::Texture(ref texture) => {
        let tex_x = wrap(texture_coords.x, texture.width());
        let tex_y = wrap(texture_coords.y, texture.height());

        Color::from_rgba(texture.get_pixel(tex_x, tex_y))
      }
    }
  }
}

fn wrap(val: f32, bound: u32) -> u32 {
  let signed_bound = bound as i32;
  let float_coord = val * bound as f32;
  let wrapped_coord = (float_coord as i32) % signed_bound;
  if wrapped_coord < 0 {
    (wrapped_coord + signed_bound) as u32
  } else {
    wrapped_coord as u32
  }
}


//
// ELEMENTS
//

#[derive(Deserialize, Debug)]
pub struct Plane {
  pub origin: Point,
  #[serde(deserialize_with="Vector3::deserialize_normalized")]
  pub normal: Vector3,
  pub material: Material,
}


#[derive(Deserialize, Debug)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub material: Material,
}


#[derive(Deserialize, Debug)]
pub enum Element {
  Sphere(Sphere),
  Plane(Plane),
}
impl Element {
  pub fn material(&self) -> &Material {
    match *self {
      Element::Sphere(ref s) => &s.material,
      Element::Plane(ref p) => &p.material,
    }
  }

  // pub fn material_mut(&mut self) -> &mut Material {
  //   match *self {
  //     Element::Sphere(ref s) => &mut s.material,
  //     Element::Plane(ref p) => &mut p.material,
  //   }
  // }
}


//
// SCENE
//

#[derive(Deserialize, Debug)]
pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub max_recursion_depth: u32,
  pub shadow_bias: f64,
  pub elements: Vec<Element>,
  pub lights: Vec<Light>,
}
impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self.elements
      .iter()
      .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}


//
// INTERSECTION
//

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
