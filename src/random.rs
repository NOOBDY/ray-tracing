use cgmath::{dot, vec3, InnerSpace, Vector3};
use rand::random;

pub fn random_f64_with_min_max(min: i32, max: i32) -> f64 {
    let min = f64::from(min);
    let max = f64::from(max);
    min + (max - min) * random::<f64>()
}

#[allow(dead_code)]
pub fn random_vec3() -> Vector3<f64> {
    vec3(random::<f64>(), random::<f64>(), random::<f64>())
}

pub fn random_vec3_with_min_max(min: i32, max: i32) -> Vector3<f64> {
    vec3(
        random_f64_with_min_max(min, max),
        random_f64_with_min_max(min, max),
        random_f64_with_min_max(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vec3_with_min_max(-1, 1);

        if dot(p, p) < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
