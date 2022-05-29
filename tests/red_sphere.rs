mod configuration;
use configuration::*;

use rayimg::{Scene, Renderer, P3ImageWriter, math::Vec3, RGB, shapes::Sphere, materials::Lambertian};

use std::rc::Rc;

#[test]
fn create_red_sphere() {
    let mut sky = Scene::new();
    let red_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(1.0, 0.0, 0.0))));
    sky.add_object(red_sphere);
    let renderer = Renderer::new(sky, get_camera(), |r| {
        RGB(1.0, 1.0, 1.0)
    });

    let output_file = std::fs::File::create("tests/output/red_sphere.ppm").expect("Failed to create test file");
    renderer.render(&mut P3ImageWriter::new(BOUNDS, output_file));
}
