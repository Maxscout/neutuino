use std::{
    io::{BufReader, Cursor},
    path::Path,
};

use image::{DynamicImage, ImageFormat, ImageResult};

use crate::graphics::base64::base64_encode;

pub fn load_image(path: &Path, fmt: ImageFormat) -> ImageResult<image::DynamicImage> {
    image::load(
        BufReader::new(Cursor::new(std::fs::read(path)?.as_slice())),
        fmt,
    )
}

pub fn show_image(image: DynamicImage) -> ImageResult<String> {
    let _ = std::fs::remove_dir_all(Path::new("/tmp/neutuino-graphics"));
    std::fs::create_dir(Path::new("/tmp/neutuino-graphics"))?;
    image.save_with_format(
        Path::new("/tmp/neutuino-graphics/tmp.png"),
        ImageFormat::Png,
    )?;

    Ok(format!(
        "{}a=T,f=100;{}{}",
        super::ANSI_GRAPHICS_PROTOCOL_ESCAPE_START,
        base64_encode("/tmp/neutuino-graphics/tmp.png".as_bytes()),
        super::ANSI_GRAPHICS_PROTOCOL_ESCAPE_END
    ))
}
