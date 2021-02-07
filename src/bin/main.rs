use std::io::Error;
use std::process::Command;
use std::process::Output;

use raytrace::image::*;
use raytrace::vec3::*;

fn main() {
    image_test("output.ppm");
}

fn image_test(path: &str) {
    let mut image = Image::new(512, 512);
    let canvas_size = image.get_size();

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            image.set_pixel(
                i,
                j,
                Vec3::new_rgb(
                    i as f32 / canvas_size.0 as f32,
                    j as f32 / canvas_size.1 as f32,
                    1.0,
                ),
            );
        }
    }
    image.write_ppm(path).unwrap();
    ppm_to_png(path).expect("converting is failed ppm to png");
}

pub fn ppm_to_png(path: &str) -> Result<Output, Error> {
        Command::new("cmd")
                .arg(format!("{} {} {}.png", "magick convert", path, path))
                .output()
}