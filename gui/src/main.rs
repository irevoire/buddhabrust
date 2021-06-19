mod window;

use buddhabrust::{color, Buddha};
use std::time::Instant;
use window::Window;

const HEIGHT: usize = 2000;
const WIDTH: usize = 2000;

fn main() {
    let iter = std::env::args().nth(1);
    let mut buddha = if let Some(iter) = iter {
        Buddha::new(-3.1795, -3.1634, iter.parse().unwrap(), 300.)
    } else {
        Buddha::new(-3.1795, -3.1634, 400, 300.)
    };
    let mut window = Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    buddha.compute(&mut window.buffer, width, height);
    window.update();

    while window.handle_event(&mut buddha) {
        let now = Instant::now();

        window.buffer.iter_mut().for_each(|pixel| *pixel = 0);
        let (width, height) = window.dimension();
        buddha.compute(&mut window.buffer, width, height);

        color::nb_iter_to_rgb(&mut window.buffer);

        println!(
            "buddha {:4} for {} iter",
            now.elapsed().as_secs_f32(),
            buddha.iter
        );
        let now = Instant::now();

        window.update();

        println!("refresh {:?}", now.elapsed().as_secs_f32());
    }
}
