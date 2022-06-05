use rayimg::{*, math::*, shapes::*, materials::*};
use std::rc::Rc;

fn main() {
    let mut scene = Scene::new();
    let cutty_sark_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(0.2, 0.3, 0.4))));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.8, 0.8, 0.0))));
    let outer_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5))); // RGB(1.0, 1.0, 1.0) for clear glass
    let inner_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5)));
    let lime_outer_glass_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(RGB(0.05, 0.95, 0.05), 1.5)));
    let lime_inner_glass_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(RGB(0.05, 0.95, 0.05), 1.5)));
    
    scene.add_object(cutty_sark_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(outer_glass_sphere);
    scene.add_object(inner_glass_sphere);
    scene.add_object(lime_outer_glass_sphere);
    scene.add_object(lime_inner_glass_sphere);
    
    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0 * aspect_ratio, 2.0), 1.0);
    let mut renderer = Renderer::new(scene, camera, |r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    });

    renderer.set_sample_count(1000);

    renderer.render(P3ImageWriter::new((1280, 720), std::fs::File::create("examples/output/dielectric/dielectric.ppm").expect("Failed to create output file")));
}
