extern crate image;
extern crate tobj;

use core::f32;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use rand::{thread_rng, Rng};
use raytracer::material::Material;
use raytracer::pinhole_camera::PinholeCamera;
use raytracer::scene::Scene;
use raytracer::shapes::obj::Obj;
use raytracer::shapes::sphere::Sphere;
use raytracer::vec3::{Color, Vec3};
use raytracer::*;

fn main() {
    //raytrace_with_hard(512, 512, "output.png", 16, 100);
    raytrace(512, 512, "output.png", 16, 100);
}

fn raytrace(width: u32, height: u32, path: &str, ssaa_sampling_point: u32, sample: u32) {
    let image = image::ImageBuffer::new(width, height);

    let mut scene = Scene::new(Vec3::new(0.5, 1.0, 0.5).normalized());

     scene.add_obj(Obj::new(
        "./models/cube.obj",
        Vec3::new(2.0, -0.4, 3.0),
        Material::Diffuce,
        Vec3::from(0.9),
    ));

    //立方体
/*
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, 0.1, 3.5), Vec3::new(2.5, 0.1, 3.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, 0.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(1.5, -1.1, 3.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 3.5), Vec3::new(1.5, 0.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(2.5, 0.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 2.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(1.5, -1.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(1.5, 0.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 3.5), Vec3::new(1.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 3.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 3.5), Vec3::new(2.5, 0.1, 3.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, -1.1, 3.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(2.5, -1.1, 2.5), Material::Diffuce, Vec3::from(0.9)));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(2.5, -1.1, 3.5), Material::Diffuce, Vec3::from(0.9)));
 */
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Diffuce,
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(4.0, 1.0, 7.0),
        Vec3::from(0.0) - Vec3::new(4.0, 1.0, 7.0).normalized(),
        1.0,
    ));
    let image = Arc::new(Mutex::new(image));
    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..height {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            let rgbs = (0..width)
                .map(|i| {
                    let mut rng = thread_rng();

                    let kd: Color = (0..ssaa_sampling_point)
                        .map(|_| {
                            let u = (2.0 * (i as f32 + rng.gen_range(0.0..1.0)) - width as f32)
                                / height as f32;
                            let v = (2.0 * (j as f32 + rng.gen_range(0.0..1.0)) - height as f32)
                                / height as f32;

                            let ray = camera.make_ray_to_pinhole(u, v);
                            let raytracer = Raytracer::new(100, &scene, 0.9);

                            raytracer.pathtrace(ray, 0, 0.99, sample)
                        })
                        .fold(Vec3::from(0.0), |sum, color| sum + color)
                        / ssaa_sampling_point as f32;

                    let kd = Vec3::new(gamma(kd.x), gamma(kd.y), gamma(kd.z));

                    let r = num::clamp(kd.x * 255.0, 0.0, 255.0) as u8;
                    let g = num::clamp(kd.y * 255.0, 0.0, 255.0) as u8;
                    let b = num::clamp(kd.z * 255.0, 0.0, 255.0) as u8;

                    image::Rgb([r, g, b])
                })
                .collect::<Vec<_>>();

            let mut image = image.lock().unwrap();

            for (i, rgb) in (0_u32..).zip(rgbs) {
                let pixel = image.get_pixel_mut(i as u32, j);
                *pixel = rgb;
            }
        }));
    }

    print!("running");
    for handle in handles {
        print!(".");
        stdout().flush().unwrap();
        handle.join().unwrap();
    }

    let image = image.lock().unwrap();
    image.save(path).unwrap();
}

#[allow(dead_code)]
fn raytrace_with_hard(width: u32, height: u32, path: &str, ssaa_sampling_point: u32, sample: u32) {
    let image = image::ImageBuffer::new(width, height);

    let mut scene = Scene::new(Vec3::new(0.5, 1.0, 0.5).normalized());

    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        Material::Diffuce,
        Vec3::new(0.9, 0.9, 0.9),
    ));

    scene.add_sphere(Sphere::new(
        Vec3::new(-2.0, 0.0, 1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.8, 0.2, 0.2),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.2, 0.8, 0.2),
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(2.0, 0.0, -1.0),
        1.0,
        Material::Diffuce,
        Vec3::new(0.2, 0.2, 0.8),
    ));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(4.0, 1.0, 7.0),
        Vec3::from(0.0) - Vec3::new(4.0, 1.0, 7.0).normalized(),
        1.0,
    ));
    let image = Arc::new(Mutex::new(image));
    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..height {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            let rgbs = (0..width)
                .map(|i| {
                    let mut rng = thread_rng();

                    let kd: Color = (0..ssaa_sampling_point)
                        .map(|_| {
                            let u = (2.0 * (i as f32 + rng.gen_range(0.0..1.0)) - width as f32)
                                / height as f32;
                            let v = (2.0 * (j as f32 + rng.gen_range(0.0..1.0)) - height as f32)
                                / height as f32;

                            let ray = camera.make_ray_to_pinhole(u, v);
                            let raytracer = Raytracer::new(100, &scene, 0.9);

                            raytracer.pathtrace(ray, 0, 0.99, sample)
                        })
                        .fold(Vec3::from(0.0), |sum, color| sum + color)
                        / ssaa_sampling_point as f32;

                    let kd = Vec3::new(gamma(kd.x), gamma(kd.y), gamma(kd.z));

                    let r = num::clamp(kd.x * 255.0, 0.0, 255.0) as u8;
                    let g = num::clamp(kd.y * 255.0, 0.0, 255.0) as u8;
                    let b = num::clamp(kd.z * 255.0, 0.0, 255.0) as u8;

                    image::Rgb([r, g, b])
                })
                .collect::<Vec<_>>();

            let mut image = image.lock().unwrap();

            for (i, rgb) in (0_u32..).zip(rgbs) {
                let pixel = image.get_pixel_mut(i as u32, j);
                *pixel = rgb;
            }
        }));
    }

    print!("running");
    for handle in handles {
        print!(".");
        stdout().flush().unwrap();
        handle.join().unwrap();
    }

    let image = image.lock().unwrap();
    image.save(path).unwrap();
}
