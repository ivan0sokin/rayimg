use rayimg::{materials::{Dielectric, Lambertian, Metal}, math::Vec3, shapes::Sphere, BVHNode, Camera, P3ImageWriter, Renderer, Scene, RGB};

fn random_in_zero_to_one() -> f64 {
    Vec3::<f64>::random_in_unit_segment().x.abs()
}

fn main() {
    let mut scene = Scene::new();

    let ground_material = Lambertian::new(RGB::new(0.5, 0.5, 0.5));
    scene.add_object(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = Vec3::<f64>::random_in_unit_segment().x.abs();
            let center = Vec3::new(a as f64 + 0.9 * random_in_zero_to_one(), 0.2, b as f64 + 0.9 * random_in_zero_to_one());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuses
                    let color1 = RGB::new(random_in_zero_to_one(), random_in_zero_to_one(), random_in_zero_to_one());
                    let color2 = RGB::new(random_in_zero_to_one(), random_in_zero_to_one(), random_in_zero_to_one());
                    let mat = Lambertian::new(color1 * color2);
                scene.add_object(Sphere::new(center, 0.2, mat));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = RGB::new(0.5 + 0.5 * random_in_zero_to_one(), 0.5 + 0.5 * random_in_zero_to_one(), 0.5 + 0.5 * random_in_zero_to_one());
                    let fuzz = 0.5 * random_in_zero_to_one();
                    let mat = Metal::new(albedo, fuzz);
                scene.add_object(Sphere::new(center, 0.2, mat));

                } else {
                    // glass
                    let mat = Dielectric::new(RGB::new(1.0, 1.0, 1.0), 1.5);
                    scene.add_object(Sphere::new(center, 0.2, mat));

                }

            }
        }
    }

    let material1 = Dielectric::new(RGB::new(1.0, 1.0, 1.0), 1.5);
    scene.add_object(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let  material2 = Lambertian::new(RGB::new(0.4, 0.2, 0.1));
    scene.add_object(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0);
    scene.add_object(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let camera = Camera::new().aspect_ratio(16.0 / 9.0)
                              .vertical_fov(20.0)
                              .position(Vec3::new(13.0, 2.0, 3.0))
                              .target(Vec3::new(0.0, 0.0, 0.0))
                              .up(Vec3::new(0.0, 1.0, 0.0))
                              .focus_distance(10.0)
                              .build();
    
    let renderer = Renderer::new(BVHNode::from_scene(scene), camera)
                                               .ray_depth(50)
                                               .sample_count(500)
                                               .ray_miss(|r| {
                                                   let unit_direction = r.direction().normalize();
                                                   let t = 0.5 * (unit_direction.y + 1.0);
                                                   (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
                                               })
                                               .build();

    let file = std::fs::File::create("examples/output/result/result.ppm").expect("Failed to create test file");
    renderer.render_multithreaded(P3ImageWriter::new((1920, 1080), file));
}