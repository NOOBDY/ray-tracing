use cgmath::{vec3, Vector3};

use crate::interval::Interval;

pub type Color = Vector3<f64>;

fn linear_to_gamma(linear_component: Vector3<f64>) -> Vector3<f64> {
    vec3(
        linear_component.x.sqrt(),
        linear_component.y.sqrt(),
        linear_component.z.sqrt(),
    )
}

pub fn convert_color(color: Color, samples_per_pixel: u32) -> [u8; 3] {
    let scale = 1.0 / f64::from(samples_per_pixel);

    let color = color * scale;
    let color = linear_to_gamma(color);

    let intensity = Interval::new(0.000, 0.999);

    let x = (256.0 * intensity.clamp(color.x)) as u8;
    let y = (256.0 * intensity.clamp(color.y)) as u8;
    let z = (256.0 * intensity.clamp(color.z)) as u8;

    [x, y, z]
}
