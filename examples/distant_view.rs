use rayimg::{*, math::*, shapes::*, materials::*};
use std::rc::Rc;

fn main() {
    let mut scene = Scene::new();
    let cutty_sark_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(0.2, 0.3, 0.4))));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.8, 0.8, 0.0))));
    let outer_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5))); // RGB(camera.0, camera.0, camera.0) for clear glass
    let inner_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5)));
    let cyan_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(RGB(0.0, 1.0, 1.0), 0.88)));

    scene.add_object(cutty_sark_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(outer_glass_sphere);
    scene.add_object(inner_glass_sphere);
    scene.add_object(cyan_sphere);

    let args = std::env::args().collect::<Vec<String>>();
    let position = user_position(&args).unwrap_or(Vec3::new(-2.0, 2.0, 1.0));

    let camera = Camera::new()
        .position(position)
        .build();

    let renderer = Renderer::new(scene, camera, |r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    });

    renderer.render(P3ImageWriter::new((400, 225), std::fs::File::create("examples/output/distant_view/distant_view.ppm").expect("Failed to create output file")));
}

fn user_position(args: &[String]) -> Option<Vec3<f64>> {
    let mut coordinates = Vec::new();
    for arg in args {
        if let Ok(coordinate) = arg.parse::<f64>() {
            coordinates.push(coordinate)
        }

        if coordinates.len() >= 3 {
            return Some(Vec3::new(coordinates[0], coordinates[1], coordinates[2]));
        }
    }

    None
}