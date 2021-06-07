use core::str;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use image::ImageBuffer;
use image::Rgb;
pub struct Encoder<P: AsRef<Path>> {
    output_path: P,
    output_temp_path: &'static str,
    width: u32,
    height: u32,
    framerate: u32,
    writer: BufWriter<File>,
}

impl<P: AsRef<Path>> Encoder<P> {
    pub fn new(output_path: P, width: u32, height: u32, framerate: u32) -> Result<Encoder<P>, Box<dyn Error>> {
        let output_temp_path = "./temp";


        let writer = BufWriter::new(File::create(output_temp_path)?);

        Ok(Encoder {
            output_path,
            output_temp_path,
            width,
            height,
            framerate,
            writer,
        })
    }

    pub fn write(&mut self, frame: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let (width, height) = frame.dimensions();

        if (width, height) != (self.width, self.height) {
            Err(Box::<dyn Error>::from(std::io::Error::new(std::io::ErrorKind::Other, "Invalid image size")))
        } else {
            self.writer.write_all(frame.as_raw())?;
            Ok(())
        }
    }

    pub fn encode(&mut self) -> Result<(), Box<dyn Error>> {

        let command = |width, height, framerate, output_temp_path, output_path| {
            format!(
                "ffmpeg -framerate {framerate} -f rawvideo -pix_fmt rgba -s {width}x{height} -i {output_temp_path} -pix_fmt yuv420p -vcodec libx264 -movflags faststart {output_path:?}",
                width=width, height=height, framerate=framerate, output_temp_path=output_temp_path, output_path=output_path
            )
        };

        let mut ffmpeg = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            Command::new("sh")
        }.args(&["-c", &command(self.width, self.height, self.framerate, self.output_temp_path, self.output_path.as_ref())])
        .stdin(Stdio::piped())
        .spawn()?;

        drop(ffmpeg.stdin.take());
        drop(ffmpeg.stdout.take());
        ffmpeg.wait()?;

        Ok(())
    }
}