mod configuration;
use configuration::*;

#[test]
fn create_sky_image() {
    let sky = Scene::new();
    let renderer = Renderer::new(sky, Camera::default())
        .ray_miss(|r| {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()}
        )
        .build();
    let output_file = std::fs::File::create("tests/output/sky.ppm").expect("Failed to create test file");
    renderer.render_multithreaded(&mut P3ImageWriter::new(BOUNDS, output_file));
}
