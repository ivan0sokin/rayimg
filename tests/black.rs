mod configuration;
use configuration::*;

#[test]
fn create_black_image() {
    let renderer = Renderer::new(Scene::new(), Camera::default()).build();
    let output_file = std::fs::File::create("tests/output/black.ppm").expect("Failed to create test file");
    renderer.render_multithreaded(&mut P3ImageWriter::new(BOUNDS, output_file));
}