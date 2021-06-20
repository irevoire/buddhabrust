mod window;

use buddhabrust::{color, Buddha};
use std::time::Instant;
use window::Window;

const HEIGHT: usize = 2000;
const WIDTH: usize = 2000;

fn main() {
    let iter = std::env::args().nth(1);
    let mut buddha = if let Some(iter) = iter {
        Buddha::new(-4.8461, -2.1633, iter.parse().unwrap(), 300.)
    } else {
        Buddha::new(-4.8461, -2.1633, 400, 300.)
    };
    let mut window = Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    // buddha.compute(&mut window.buffer, width, height);
    window.update();

    while window.handle_event(&mut buddha) {
        let now = Instant::now();

        window.buffer.iter_mut().for_each(|pixel| *pixel = 0);
        let (width, height) = window.dimension();
        let mut red_channel = window.buffer.clone();
        let mut red_buddha = buddha.clone();
        red_buddha.iter = buddha.colorization.r;
        red_buddha.compute(&mut red_channel, width, height);
        let red_channel = color::scale(&red_channel);

        let mut green_channel = window.buffer.clone();
        let mut green_buddha = buddha.clone();
        green_buddha.iter = buddha.colorization.g;
        green_buddha.compute(&mut green_channel, width, height);
        let green_channel = color::scale(&green_channel);

        let mut blue_channel = window.buffer.clone();
        let mut blue_buddha = buddha.clone();
        blue_buddha.iter = buddha.colorization.b;
        blue_buddha.compute(&mut blue_channel, width, height);
        let blue_channel = color::scale(&blue_channel);

        // println!("buddha {:?} for {} iter", now.elapsed(), buddha.iter);
        println!("buddha {:?} for {:?} iter", now.elapsed(), buddha.colorization);

        let now = Instant::now();
        color::merge_rgb_layers(
            &mut window.buffer,
            &red_channel,
            &green_channel,
            &blue_channel,
        );
        // color::nb_iter_to_rgb(&mut window.buffer);
        println!("Colorized the buddhabrot in {:?}", now.elapsed());

        use std::io::prelude::*;
        let mut file = std::fs::File::create("truc.bmp").unwrap();
        let mut bmp_writer = image::bmp::BmpEncoder::new(&mut file);
        let slice: &[u8] = unsafe {
            std::slice::from_raw_parts(window.buffer.as_ptr() as *const u8, window.buffer.len() * std::mem::size_of::<u32>())
        };
        bmp_writer.encode(slice, width as u32, height as u32, image::ColorType::Rgba8);

        return;


        let now = Instant::now();

        window.update();

        println!("refresh {:?}", now.elapsed().as_secs_f32());
    }
}
