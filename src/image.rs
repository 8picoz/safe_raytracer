use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use num::clamp;

use crate::vec3::*;

pub struct Image {
    width: usize,
    height: usize,
    canvas: Vec<f32>,
    canvas_array_size: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let array_size = 3 * width * height;

        Image {
            width,
            height,
            canvas: vec![0.0; array_size as usize],
            canvas_array_size: array_size,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: Vec3f) {
        let target_pixel_index = 3 * x + 3 * self.width * y;

        self.canvas[target_pixel_index] = rgb.x;
        self.canvas[target_pixel_index + 1] = rgb.y;
        self.canvas[target_pixel_index + 2] = rgb.z;
    }

    pub fn write_ppm(&self, output_name: &str) -> io::Result<()> {
        let f = File::create(output_name)?;
        let mut writer = BufWriter::new(f);

        writer.write_all(b"P3\r\n")?;
        writer.write_all(&format!("{} {}\r\n", self.width, self.height).as_bytes())?;
        writer.write_all(b"255\r\n")?;

        for j in 0..self.height {
            for i in 0..self.width {
                let index = 3 * i + 3 * self.width * j;
                let r = clamp(self.canvas[index] * 255.0, 0.0, 255.0) as usize;
                let g = clamp(self.canvas[index + 1] * 255.0, 0.0, 255.0) as usize;
                let b = clamp(self.canvas[index + 2] * 255.0, 0.0, 255.0) as usize;

                writer.write_all(&format!("{} ", r).as_bytes())?;
                writer.write_all(&format!("{} ", g).as_bytes())?;
                writer.write_all(&format!("{}\r\n", b).as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn gamma_set(&mut self) {
        for index in 0..self.canvas_array_size {
            self.canvas[index] = self.canvas[index].powf(1.0 / 2.2);
        }
    }
}
