use std::{ops::Deref, fs::File, io::Write};

use webp::Encoder;
use image::open;

fn main() {
    //Opens the image file using image::open and taking path as argument.
    let image = open("src/in/test.png").unwrap();

    // Created the encoder of the image out of the image itself.
    let encoder = Encoder::from_image(&image).unwrap();

    //Encodes the encoder taking quality (float) as an argument (Take this as the level of compression that the image will have)
    let quality = encoder.encode(90.0);

    //Gets the bytes out of the encoded image.
    let image_bytes = quality.deref();

    //Creates an empty file with .webp extension.
    let mut webp_image = File::create("src/out/test.webp").unwrap();


    //Fills the empty .webp file with bytes from image_bytes.
    webp_image.write_all(image_bytes).unwrap()
}
