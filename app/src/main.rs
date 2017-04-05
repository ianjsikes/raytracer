extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate raytracer;
extern crate image;

use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use raytracer::scene::*;
use image::ImageFormat;

fn main() {
  let app = App::new("raytracer")
    .version("0.1")
    .author("ianjsikes <ianjsikes@gmail.com>")
    .about("Basic Raytracer")
    .arg(Arg::with_name("scene")
      .help("Sets the scene file to use")
      .required(true)
      .index(1))
    .arg(Arg::with_name("image")
      .help("Sets the output image file")
      .required(true)
      .index(2));
  
  let matches = app.get_matches();

  let scene_path = matches.value_of("scene").unwrap();
  let scene_file = File::open(scene_path).expect("File not found");

  let image_path = matches.value_of("image").unwrap();

  let scene: Scene = if scene_path.ends_with(".yml") || scene_path.ends_with(".yaml") {
    serde_yaml::from_reader(scene_file).unwrap()
  } else if scene_path.ends_with(".json") {
    serde_json::from_reader(scene_file).unwrap()
  } else {
    panic!("Invalid scene file type! Must be .json, .yml, or .yaml");
  };

  let image = raytracer::render(&scene);

  let mut image_file =
    OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
  image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
