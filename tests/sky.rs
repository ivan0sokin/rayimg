mod configuration;
use configuration::*;

use rayimg::{Scene, Renderer, P3ImageWriter, math::{Vec3, Ray}, RGB};

#[test]
fn create_sky_image() {
    let sky = Scene::new();
    let renderer = Renderer::new(sky, get_camera(), |r| {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        RGB::from_slice(&(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).as_slice())
    });

    let test_file = std::fs::File::create("tests/output/sky.ppm").expect("Failed to create test file");
    renderer.render(&mut P3ImageWriter::new(BOUNDS, test_file));
}