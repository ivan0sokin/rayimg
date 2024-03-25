use rayimg::{*, math::*, shapes::*, materials::*};

fn main() {
    let mut scene = Scene::new();
    let cutty_sark_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(RGB(0.2, 0.3, 0.4)));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(RGB(0.8, 0.8, 0.0)));
    let outer_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5)); // RGB(camera.0, camera.0, camera.0) for clear glass
    let inner_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5));
    let cyan_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::new(RGB(0.0, 1.0, 1.0), 0.88));

    scene.add_object(cutty_sark_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(outer_glass_sphere);
    scene.add_object(inner_glass_sphere);
    scene.add_object(cyan_sphere);

    let camera = Camera::new()
        .position(Vec3::new(-2.0, 2.0, 1.0))
        .defocus_angle(user_aperture(&std::env::args().collect::<Vec<String>>()).unwrap_or(10.0)) // 0.0 aperture by default and everything in focus
        .focus_distance(3.4)
        .vertical_fov(20.0)
        .build();

    let renderer = Renderer::new(scene, camera)
        .ray_miss(|r| {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
        })
        .sample_count(100)
        .build();

    renderer.render_multithreaded(P3ImageWriter::new((1920, 1080), std::fs::File::create("examples/output/lens/lens.ppm").expect("Failed to create output file")));
}

fn user_aperture(args: &[String]) -> Option<f64> {
    for arg in args {
        if let Ok(aperture) = arg.parse::<f64>() {
            return Some(aperture);
        }
    }

    None
}