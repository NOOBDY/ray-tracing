use cgmath::{dot, AbsDiffEq, Vector3};

pub fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;
    v.x.abs_diff_eq(&0.0, s) && v.y.abs_diff_eq(&0.0, s) && v.z.abs_diff_eq(&0.0, s)
}

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - dot(r_out_perp, r_out_perp)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
