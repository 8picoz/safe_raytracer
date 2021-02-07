use std::env;
use std::io::Error;
use std::process::Command;
use std::process::Output;

use raytracer::image::*;
use raytracer::vec3::*;

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
    let output = ppm_to_png(path).expect("converting is failed ppm to png");
}

//depend on imagemagick
pub fn ppm_to_png(path: &str) -> Result<Output, Error> {

    let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let command = format!("{} {}\\{} {}\\{}.png", "magick convert", current_path, path, current_path, path);

    Command::new("powershell")
            .current_dir(env::current_dir()?)
            .arg(command)
            .output()
}