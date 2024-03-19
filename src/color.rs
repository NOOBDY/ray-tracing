use cgmath::Vector3;

use crate::interval::Interval;

pub type Color = Vector3<f64>;

pub fn convert_color(color: Color, samples_per_pixel: u32) -> [u8; 3] {
    let scale = 1.0 / f64::from(samples_per_pixel);

    let color = color * scale;

    let intensity = Interval::new(0.000, 0.999);

    let x = (256.0 * intensity.clamp(color.x)) as u8;
    let y = (256.0 * intensity.clamp(color.y)) as u8;
    let z = (256.0 * intensity.clamp(color.z)) as u8;

    [x, y, z]
}
