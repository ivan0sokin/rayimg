mod configuration;
use configuration::*;

use rayimg::{Scene, Renderer, P3ImageWriter, Camera, RGB};

#[test]
fn create_black_image() {
    let blank_scene = Scene::new();
    let renderer = Renderer::new(blank_scene, get_camera(), |_| { return RGB::new(0.0, 0.0, 0.0); });
    let output_file = std::fs::File::create("tests/output/black.ppm").expect("Failed to create test file");
    renderer.render(&mut P3ImageWriter::new(BOUNDS, output_file));
}