mod configuration;
use configuration::*;

const ITERATIONS: usize = 10000000;

#[test]
fn random_in_unit_disk() {
    for _ in 0..ITERATIONS {
        let r = Vec3::random_in_unit_disk();
        assert!(-1.0 < r.x && r.x < 1.0);
        assert!(-1.0 < r.y && r.y < 1.0);
        assert_eq!(r.z, 0.0);
        assert!(r.len() < 1.0);
    }
}

#[test]
fn random_in_unit_sphere() {
    for _ in 0..ITERATIONS {
        let r = Vec3::random_in_unit_sphere();
        assert!(-1.0 < r.x && r.x < 1.0);
        assert!(-1.0 < r.y && r.y < 1.0);
        assert!(-1.0 < r.z && r.z < 1.0);
        assert!(r.len() < 1.0);
    }
}