use rayimg::{*, math::*, shapes::*, materials::*};

fn main() {
    let mut scene = Scene::new();
    let cutty_sark_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(RGB(0.1, 0.2, 0.5)));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(RGB(0.8, 0.8, 0.0)));
    let outer_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5));
    let inner_glass_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, Dielectric::new(RGB(1.0, 1.0, 1.0), 1.5));
    let cyan_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::new(RGB(0.8, 0.6, 0.2), 0.0));

    scene.add_object(cutty_sark_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(outer_glass_sphere);
    scene.add_object(inner_glass_sphere);
    scene.add_object(cyan_sphere);

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut fov_list = user_fov_list(&args);
    if !args.contains(&"--only".into()) {
        fov_list.extend_from_slice(&[20.0, 30.0, 60.0, 90.0, 120.0, 150.0]);
    }

    for fov in fov_list {
        let camera = Camera::new()
            .position(Vec3::new(-2.0, 2.0, 1.0))
            .target(Vec3::new(0.0, 0.0, -1.0))
            .vertical_fov(fov)
            .build();

        let renderer = Renderer::new(scene.clone(), camera)
            .ray_miss(|r| {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
            })
            .ray_depth(50)
            .sample_count(100)
            .build();

        renderer.render_multithreaded(P3ImageWriter::new((400, 225), std::fs::File::create(format!("examples/output/changeable_fov/fov_{:.0}.ppm", fov)).expect("Failed to create output file")));
    }
}

fn user_fov_list(args: &[String]) -> Vec<f64> {
    if args.is_empty() {
        return Vec::new();
    }

    let mut fov_list = Vec::new();
    for arg in args {
        match arg.parse::<f64>() {
            Ok(fov) => fov_list.push(fov),
            _ => ()
        };
    }

    fov_list
}