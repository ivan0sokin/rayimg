mod configuration;
use configuration::*;

#[test]
fn create_sky_image() {
    let sky = Scene::new();
    let renderer = Renderer::new(sky, Camera::default())
        .ray_miss(sky_color())
        .build();
    let output_file = std::fs::File::create("tests/output/sky.ppm").expect("Failed to create test file");
    renderer.render(&mut P3ImageWriter::new(BOUNDS, output_file));
}
