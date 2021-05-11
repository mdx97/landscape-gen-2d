use clap::{ App, Arg };
use image::{ ImageBuffer, ImageFormat, Rgb, RgbImage };
use rand::Rng;

const DEFAULT_FLATNESS: f64 = 1.0;
const DEFAULT_SIZE: u32 = 512;
const DEFAULT_VARIANCE: u32 = 1;

fn main() {
    let matches = App::new("landscape-gen-2d")
        .author("Mathew H. <mathewhorner456@gmail.com>")
        .about("2D Procedurally generated landscapes")
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .help("Sets the size of the landscape")
            .takes_value(true))
        .arg(Arg::with_name("flatness")
            .short("f")
            .long("flatness")
            .help("Sets the flatness factor of the landscape.\n\
                   Values can be in the range [0.0, inf) where 0.0 means there will be no flat sections, \
                   and the higher the value, the more likely flat sections are to occur.")
            .takes_value(true))
        .arg(Arg::with_name("variance")
            .short("v")
            .long("variance")
            .help("Sets the variance factor of the landscape.\n\
                   Values can be in the range [0, inf) where the value represents the maximum height differential \
                   between adjacent blocks.")
            .takes_value(true))
        .get_matches();

    let size = matches.value_of("size").unwrap_or("")
        .parse::<u32>()
        .unwrap_or(DEFAULT_SIZE);

    let flatness = matches.value_of("flatness").unwrap_or("")
        .parse::<f64>()
        .unwrap_or(DEFAULT_FLATNESS);

    let variance = ((
        matches.value_of("variance").unwrap_or("")
            .parse::<u32>()
            .unwrap_or(DEFAULT_VARIANCE)) as f64) * 2.0;

    let mut rng = rand::thread_rng();
    let mut height = (size / 2) as i32;
    let mut image: RgbImage = ImageBuffer::new(size, size);

    for i in 0..size {
        let random = rng.gen_range(0.0..(variance + flatness));
        if random < variance {
            let adjusted = random - (variance / 2.0);
            height += (if adjusted > 0.0 { adjusted.ceil() } else { adjusted.floor() }) as i32;
        }

        for j in 0..height.abs() as u32 {
            image.put_pixel(i, size - j - 1, Rgb([255, 255, 255]));
        }
    }

    image.save_with_format("render.bmp", ImageFormat::Bmp).unwrap();
}
