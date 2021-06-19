mod window;

use buddhabrust::{color, Buddha};
use std::{collections::{BTreeMap, HashMap}, time::Instant};
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
    // buddha.compute(&mut window.buffer, width, height);
    window.update();

    while window.handle_event(&mut buddha) {
        let now = Instant::now();

        window.buffer.iter_mut().for_each(|pixel| *pixel = 0);
        let (width, height) = window.dimension();
        buddha.compute(&mut window.buffer, width, height);

        // WINDOW LAND
        let max = *window.buffer.iter().max().unwrap();
        let sum: u32 = window.buffer.iter().copied().sum();
        let average = sum as f64 / window.buffer.len() as f64;
        let median = max as f64 / 2.;
        dbg!((max, sum, average, median));
        let distribution = window.buffer.iter().fold(BTreeMap::new(), |mut hash, value| {
            *hash.entry(value).or_insert(0) += 1;
            hash
        });

        // TRUC LAND
        let mut truc = window.buffer.clone();
        truc.sort();
        let median_truc = *truc.iter().nth(window.buffer.len() / 2).unwrap();
        let last_percent = *truc.iter().nth((window.buffer.len() / 99) * 98).unwrap();
        dbg!(( last_percent, median));
        let truc_distribution = truc.iter().fold(BTreeMap::new(), |mut hash, value| {
            *hash.entry(value).or_insert(0) += 1;
            hash
        });
        let max = *window.buffer.iter().max().unwrap();
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
