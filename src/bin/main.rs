use raytrace::image::*;
use raytrace::vec3::*;


fn main() {
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
    image.write_ppm("output.ppm").unwrap();
}
