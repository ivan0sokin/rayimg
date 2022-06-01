use rayimg::{Scene, math::Vec3, Camera, shapes::Sphere, Renderer, P3ImageWriter, materials::{Lambertian, Metal}, RGB};
use std::rc::Rc;

fn main() {    
    let mut scene = Scene::new();
    let red_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(1.0, 0.0, 0.0))));
    let lime_sphere  = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.8, 0.8, 0.0))));
    let gold_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(RGB(0.831, 0.686, 0.216))));
    let silver_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(RGB(0.769, 0.792, 0.808))));
    
    scene.add_object(red_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(gold_sphere);
    scene.add_object(silver_sphere);
                                    
    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0 * aspect_ratio, 2.0), 1.0);
    let mut renderer = Renderer::new(scene, camera, |r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    });

    renderer.set_sample_count(1000);
    renderer.render(P3ImageWriter::new((400, 225), std::fs::File::create("examples/output/metal/metal.ppm").expect("Failed to create output file")));    
}
