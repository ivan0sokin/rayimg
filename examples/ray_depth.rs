use rayimg::{Scene, math::Vec3, Camera, shapes::Sphere, Renderer, P3ImageWriter, materials::Lambertian, RGB};
use std::rc::Rc;

fn main() {    
    let mut scene = Scene::new();
    let red_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(RGB(1.0, 0.0, 0.0))));
    let blue_sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(RGB(0.0, 0.0, 1.0))));

    scene.add_object(red_sphere);
    scene.add_object(blue_sphere);

    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0 * aspect_ratio, 2.0), 1.0);
    let mut renderer = Renderer::new(scene, camera, |r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    });

    let bounds = (400, (400 as f64 / aspect_ratio) as usize);
    
    let mut depths = user_ray_depths();
    depths.extend_from_slice(&[1, 5, 10, 50]);

    for ray_depth in depths {
        renderer.set_ray_depth(ray_depth);
        renderer.render(P3ImageWriter::new(bounds, std::fs::File::create(format!("examples/output/ray_depth/ray_depth_{}.ppm", ray_depth)).expect("Failed to create output file")));
    }    
}

fn user_ray_depths() -> Vec<usize> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        return Vec::new();
    }

    let mut ray_depths = Vec::new();
    for arg in args {
        match arg.parse::<usize>() {
            Ok(depth) if depth > 0 => ray_depths.push(depth),
            _ => ()
        };
    }

    ray_depths
}
