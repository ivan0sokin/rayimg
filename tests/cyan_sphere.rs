mod configuration;
use configuration::*;

use rayimg::{Scene, Renderer, P3ImageWriter, math::Vec3, RGB, shapes::Sphere, materials::Lambertian};

use std::rc::Rc;

#[test]
fn cyan_on_lime() {
    let mut sky = Scene::new();
    
    let cyan_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(0.0, 1.0, 1.0))));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.8, 0.8, 0.0))));
    sky.add_object(cyan_sphere);
    sky.add_object(lime_sphere);
    let renderer = Renderer::new(sky, get_camera(), |r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    });

    let output_file = std::fs::File::create("tests/output/cyan_over_lime.ppm").expect("Failed to create test file");
    renderer.render(&mut P3ImageWriter::new(BOUNDS, output_file));
}