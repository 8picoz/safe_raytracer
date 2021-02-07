use core::f32;
use std::env;
use std::io::Error;
use std::process::Command;
use std::process::Output;

use raytracer::image::*;
use raytracer::pinhole_camera::*;
use raytracer::sphere::*;
use raytracer::vec3::*;

fn main() {
    //image_test("output.ppm");
    //pinhole_camera_test("output.ppm");
    sphere_test("output.ppm");
}

fn sphere_test(path: &str) {
    let mut image = Image::new(512, 512);
    let canvas_size = image.get_size();

    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, -1.0), 1.0);

    let sphere = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        SphereMaterial::Diffuce,
        Vec3::new(1.0, 1.0, 1.0),
    );

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
            let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.1 as f32;

            let ray = camera.make_ray_to_pinhole(u, v);

            if let Some(info) = sphere.collision_detect(ray) {
                image.set_pixel(i, j, info.normal * 0.5 + Vec3::new(0.5, 0.5, 0.5));
            } else {
                image.set_pixel(i, j, Vec3::new(0.0, 0.0, 0.0));
            }
        }
    }

    image.write_ppm(path).expect("failed to write ppm");
    ppm_to_png(path).expect("converting is failed ppm to png");
}

//(2.0f * i - width) / widthは(i - width / 2.0f) / widthでも問題ない？
//widthが512の場合* 2.0f - widthで-512から512を取るようになるそれを512で割って[-1, 1]
fn pinhole_camera_test(path: &str) {
    let mut image = Image::new(512, 512);
    let canvas_size = image.get_size();

    let camera = PinholeCamera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 1.0);

    for j in 0..canvas_size.1 {
        for i in 0..canvas_size.0 {
            let u = (2.0 * i as f32 - canvas_size.0 as f32) / canvas_size.0 as f32;
            let v = (2.0 * j as f32 - canvas_size.1 as f32) / canvas_size.1 as f32;
            let ray = camera.make_ray_to_pinhole(u, v);
            //println!("u: {}, v: {}", u, v);

            image.set_pixel(i, j, (ray.direction + Vec3::new(1.0, 1.0, 1.0)) * 0.5);
        }
    }
    image.write_ppm(path).expect("failed to write ppm");
    ppm_to_png(path).expect("converting is failed ppm to png");
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
    image.write_ppm(path).expect("failed to write ppm");
    ppm_to_png(path).expect("failed to convert ppm to png");
}

//depend on imagemagick
pub fn ppm_to_png(path: &str) -> Result<Output, Error> {
    let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let command = format!(
        "{} {}\\{} {}\\{}.png",
        "magick convert", current_path, path, current_path, path
    );

    Command::new("powershell")
        .current_dir(env::current_dir()?)
        .arg(command)
        .output()
}
