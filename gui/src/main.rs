mod window;

use buddhabrust::{color, Buddha};
use std::{collections::HashMap, time::Instant};
use window::Window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut buddha = Buddha::new(-2.175, -0.9, 1000, 800.);
    let mut window = Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    // buddha.compute(&mut window.buffer, width, height);
    color::convert_nb_to_rbg(buddha.iter, &mut window.buffer);
    window.update();

    while window.handle_event(&mut buddha) {
        let now = Instant::now();

        window.buffer.iter_mut().for_each(|pixel| *pixel = 0);
        let (width, height) = window.dimension();
        buddha.compute(&mut window.buffer, width, height);
        let max = *window.buffer.iter().max().unwrap();
        let sum: u32 = window.buffer.iter().copied().sum();
        let average = sum as f64 / window.buffer.len() as f64;
        // let average = max as f64 / 2.;
        let median = *window.buffer.iter().nth(window.buffer.len() / 2).unwrap();
        let max = *window.buffer.iter().max().unwrap();
        // dbg!(window.buffer.iter().enumerate().filter(|(_, i)| **i != 0).collect::<Vec<_>>());
        color::convert_nb_to_rbg(buddha.iter, &mut window.buffer);

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