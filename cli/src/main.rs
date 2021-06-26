use buddhabrust::{color, Buddha};
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "The cli to generate the buddhabrot fractal.")]
struct Opt {
    /// the width of your image in pixel
    #[structopt(long, default_value = "2000")]
    width: usize,

    /// the height of your image in pixel
    #[structopt(long, default_value = "2000")]
    height: usize,

    /// the zoom level in the fractal
    #[structopt(long, default_value = "300.")]
    zoom: f64,

    /// the x position of the buddhabrot
    #[structopt(long, default_value = "-1.")]
    x: f64,

    /// the y position of the buddhabrot
    #[structopt(long, default_value = "0.")]
    y: f64,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let Opt {
        width,
        height,
        zoom,
        x: base_pos_x,
        y: base_pos_y,
        output,
    } = dbg!(Opt::from_args());
    let x = (width as f64 / 2.) / zoom ;
    let y = (height as f64 / 2.) / zoom;
    let pos = (base_pos_x - x, base_pos_y - y);
    let buddha = Buddha::new(pos.0, pos.1, 400, zoom);
    let mut window = vec![0u32; width * height];

    let mut red_channel = window.clone();
    let mut red_buddha = buddha.clone();
    red_buddha.iter = buddha.colorization.r;
    red_buddha.compute(&mut red_channel, width, height);
    let red_channel = color::scale(&red_channel);

    let mut green_channel = window.clone();
    let mut green_buddha = buddha.clone();
    green_buddha.iter = buddha.colorization.g;
    green_buddha.compute(&mut green_channel, width, height);
    let green_channel = color::scale(&green_channel);

    let mut blue_channel = window.clone();
    let mut blue_buddha = buddha.clone();
    blue_buddha.iter = buddha.colorization.b;
    blue_buddha.compute(&mut blue_channel, width, height);
    let blue_channel = color::scale(&blue_channel);

    color::merge_rgb_layers(&mut window, &red_channel, &green_channel, &blue_channel);

    let mut writer = if let Some(output) = output {
        Box::new(std::fs::File::create(output).unwrap()) as Box<dyn Write>
    } else {
        Box::new(std::io::stdout()) as Box<dyn Write>
    };
    let mut bmp_writer = image::bmp::BmpEncoder::new(&mut writer);
    let slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            window.as_ptr() as *const u8,
            window.len() * std::mem::size_of::<u32>(),
        )
    };
    bmp_writer.encode(slice, width as u32, height as u32, image::ColorType::Rgba8).unwrap();
}
