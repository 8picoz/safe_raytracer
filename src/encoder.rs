use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use image::DynamicImage;
use image::GenericImageView;

pub struct Encoder<P: AsRef<Path>> {
    output_path: P,
    ffmpeg: std::process::Child,
    width: u32,
    height: u32,
    framerate: u32,
}

impl<P: AsRef<Path>> Encoder<P> {
    pub fn new(output_path: P, width: u32, height: u32, framerate: u32) -> Result<Encoder<P>, Box<dyn Error>> {

        let command = |width, height, framerate, output_path| {
            format!(
                "ffmpeg -framerate {framerate} -f rawvideo -pix_fmt rgba -s {width}x{height} -i - -pix_fmt yuv420p -vcodec h264_nvenc -movflags faststart {output_path:?}",
                width=width, height=height, framerate=framerate, output_path=output_path
            )
        };

        let ffmpeg = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            Command::new("sh")
        }.args(&["-c", &command(width, height, framerate, output_path.as_ref())])
        .stdin(Stdio::piped())
        .spawn()?;

        Ok(Encoder {
            output_path,
            ffmpeg,
            width,
            height,
            framerate,
        })
    }

    pub fn encode(&mut self, frame: &DynamicImage) -> Result<(), Box<dyn Error>> {
        let (width, height) = frame.dimensions();

        if (width, height) != (self.width, self.height) {
            Err(Box::<dyn Error>::from(std::io::Error::new(std::io::ErrorKind::Other, "Invalid image size")))
        } else {
            let stdin = match self.ffmpeg.stdin.as_mut() {
                Some(stdin) => Ok(stdin),
                None => Err(Box::<dyn Error>::from(std::io::Error::new(std::io::ErrorKind::Other, "Cannot start ffmpeg"))),
            }?;

            stdin.write_all(&frame.to_rgb8().as_raw())?;
            Ok(())
        }
    }

    pub fn wait(mut self) -> Result<(), Box<dyn Error>> {
        self.ffmpeg.wait()?;
        Ok(())
    }
}