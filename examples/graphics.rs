use std::path::Path;

fn main() {
    neutuino::enable_ansi().unwrap();
    let img = neutuino::graphics::image::load_image(
        Path::new("/home/lip/Downloads/Screenshot_20250403_141742.png"),
        image::ImageFormat::Png,
    )
    .unwrap();

    println!("{}", neutuino::graphics::image::show_image(img).unwrap());
}
