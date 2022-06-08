use rayimg::{*, math::*, shapes::*, materials::*};
use std::rc::Rc;

fn main() {
    let mut scene = Scene::new();
    let cutty_sark_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(0.2, 0.3, 0.4))));
    let lime_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.8, 0.8, 0.0))));
    let yellow_sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(1.0, 1.0, 0.0)))); // RGB(camera.0, camera.0, camera.0) for clear glass
    let lime_outer_glass_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(RGB(0.05, 0.95, 0.05), 1.5)));
    let lime_inner_glass_sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(RGB(0.05, 0.95, 0.05), 1.5)));

    scene.add_object(cutty_sark_sphere);
    scene.add_object(lime_sphere);
    scene.add_object(yellow_sphere);
    scene.add_object(lime_outer_glass_sphere);
    scene.add_object(lime_inner_glass_sphere);

    let args = std::env::args().collect::<Vec<String>>();
    let mut targets = Vec::new();
    let mut file_name_ends = Vec::new();
    if let Some(target) = user_target(&args) {
        targets.push(target);
        file_name_ends.push("user");
    }

    if !args.contains(&"--only".into()) {
        targets.extend_from_slice(&[Vec3::new(-1.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0)]);
        file_name_ends.extend_from_slice(&["right", "middle", "left"]);
    }

    file_name_ends.reverse();

    for (i, target) in targets.into_iter().enumerate() {
        let camera = Camera::new()
            .target(target)
            .build();

        let renderer = Renderer::new(scene.clone(), camera, |r| {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
        });

        renderer.render(P3ImageWriter::new((400, 225), std::fs::File::create(format!("examples/output/targets/target_{}.ppm", file_name_ends[i])).expect("Failed to create output file")));
    }
}

fn user_target(args: &[String]) -> Option<Vec3<f64>> {
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