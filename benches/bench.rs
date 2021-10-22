#[macro_use]
extern crate criterion;

use std::io::Write;
use std::io::stdout;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use criterion::Criterion;

use rand::Rng;
use rand::thread_rng;
use safe_raytracer::renderer::raytracer::Raytracer;
use safe_raytracer::gamma;
use safe_raytracer::pinhole_camera::PinholeCamera;
use safe_raytracer::renderer::raytracer::bvh::BVH;
use safe_raytracer::renderer::raytracer::shapes::bsdf::BSDF;
use safe_raytracer::renderer::raytracer::shapes::bsdf::lambert::Lambert;
use safe_raytracer::renderer::raytracer::shapes::sphere::Sphere;
use safe_raytracer::renderer::raytracer::shapes::triangle::Triangle;
use safe_raytracer::vec3::Color;
use safe_raytracer::vec3::Vec3;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const SSAA_SAMPLING_POINT: u32 = 16;

fn pathtrace() {
    let image = image::ImageBuffer::new(WIDTH, HEIGHT);

    let mut scene = BVH::new(Vec3::new(0.5, 1.0, 0.5).normalized());

    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, 0.1, 3.5), Vec3::new(2.5, 0.1, 3.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, 0.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(1.5, -1.1, 3.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 3.5), Vec3::new(1.5, 0.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(2.5, 0.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 2.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(1.5, -1.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(2.5, 0.1, 2.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(1.5, 0.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 3.5), Vec3::new(1.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 3.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, 0.1, 3.5), Vec3::new(2.5, -1.1, 3.5), Vec3::new(2.5, 0.1, 3.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, -1.1, 3.5), Vec3::new(1.5, -1.1, 2.5), Vec3::new(2.5, -1.1, 2.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
    scene.add_triangle(Triangle::new(Vec3::new(1.5, -1.1, 3.5), Vec3::new(2.5, -1.1, 2.5), Vec3::new(2.5, -1.1, 3.5), BSDF::Lambert(Lambert::new(Vec3::from(0.9)))));
 
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -1001.0, 0.0),
        1000.0,
        BSDF::Lambert(Lambert::new(Vec3::from(0.9)))
    ));

    let camera = Arc::new(PinholeCamera::new(
        Vec3::new(4.0, 1.0, 7.0),
        Vec3::from(0.0) - Vec3::new(4.0, 1.0, 7.0).normalized(),
        1.0,
    ));
    let image = Arc::new(Mutex::new(image));
    let scene = Arc::new(scene);
    let mut handles = vec![];

    for j in 0..HEIGHT {
        let (image, camera, scene) = (image.clone(), camera.clone(), scene.clone());
        handles.push(thread::spawn(move || {
            let rgbs = (0..WIDTH)
                .map(|i| {
                    let mut rng = thread_rng();

                    let kd: Color = (0..SSAA_SAMPLING_POINT)
                        .map(|_| {
                            let u = (2.0 * (i as f32 + rng.gen_range(0.0..1.0)) - WIDTH as f32)
                                / HEIGHT as f32;
                            let v = (2.0 * (j as f32 + rng.gen_range(0.0..1.0)) - HEIGHT as f32)
                                / HEIGHT as f32;

                            let ray = camera.make_ray_to_pinhole(u, v);
                            let raytracer = Raytracer::new(100, &scene);

                            raytracer.pathtrace(ray, 0, 0.99, 100)
                        })
                        .fold(Vec3::from(0.0), |sum, color| sum + color)
                        / SSAA_SAMPLING_POINT as f32;

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
}

fn pathtrace_bench(c: &mut Criterion) {
    c.bench_function("pathtrace 100", |b| b.iter(pathtrace));
}

criterion_group!(benches, pathtrace_bench);
criterion_main!(benches);