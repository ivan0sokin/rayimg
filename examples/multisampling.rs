use rayimg::{*, math::*, shapes::*, materials::*};

fn main() {
    let mut scene = Scene::new();
    let blue_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(RGB(0.0, 0.0, 1.0)));
    let grey_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(RGB(0.5, 0.5, 0.5)));

    scene.add_object(blue_sphere);
    scene.add_object(grey_sphere);

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut sample_counts = user_sample_counts(&args);
    if !args.contains(&"--only".into()) {
        sample_counts.extend_from_slice(&[1, 100, 1000]);        
    }

    for sample_count in sample_counts {
        let renderer = Renderer::new(scene.clone(), Camera::default())
            .ray_miss(|r| {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
            })
            .sample_count(sample_count)
            .build();

        renderer.render(P3ImageWriter::new((400, 225), std::fs::File::create(format!("examples/output/multisampling/{}_sample_per_pixel.ppm", sample_count)).expect("Failed to create output file")));
    }
}

fn user_sample_counts(args: &[String]) -> Vec<usize> {
    let mut sample_counts = Vec::new();
    for arg in args {
        match arg.parse::<usize>() {
            Ok(sample_count) if sample_count > 0 => sample_counts.push(sample_count),
            _ => ()
        };
    }

    sample_counts
}
