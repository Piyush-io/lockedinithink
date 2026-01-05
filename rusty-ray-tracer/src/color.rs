use crate::common;
use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = f64::sqrt(pixel_color.x() * scale);
    let g = f64::sqrt(pixel_color.y() * scale);
    let b = f64::sqrt(pixel_color.z() * scale);
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}
