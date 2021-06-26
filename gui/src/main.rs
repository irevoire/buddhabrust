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

        window.update();

        println!("refresh {:?}", now.elapsed().as_secs_f32());
    }
}
