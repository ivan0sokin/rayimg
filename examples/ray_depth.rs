use rayimg::{*, math::*, shapes::*, materials::*};

fn main() {    
    let mut scene = Scene::new();
    let red_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(RGB(1.0, 0.0, 0.0)));
    let blue_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(RGB(0.0, 0.0, 1.0)));

    scene.add_object(red_sphere);
    scene.add_object(blue_sphere);

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut depths = user_ray_depths(&args);
    if !args.contains(&"--only".into()) {
        depths.extend_from_slice(&[1, 5, 10, 50]);   
    }

    for ray_depth in depths {
        let renderer = Renderer::new(scene.clone(), Camera::new().build())
            .ray_miss(|r| {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
            })
            .ray_depth(ray_depth)
            .build();

        renderer.render_multithreaded(P3ImageWriter::new((1280, 720), std::fs::File::create(format!("examples/output/ray_depth/ray_depth_{}.ppm", ray_depth)).expect("Failed to create output file")));
    }    
}

fn user_ray_depths(args: &[String]) -> Vec<usize> {
    let mut ray_depths = Vec::new();
    for arg in args {
        match arg.parse::<usize>() {
            Ok(depth) if depth > 0 => ray_depths.push(depth),
            _ => ()
        };
    }

    ray_depths
}