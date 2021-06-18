mod window;

use buddhabrust::{color, Buddha};
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};
use window::Window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    use palette::{FromColor, Hsl, Hue, Srgb};
    let hue_shifted = Hsl::from_color(color).shift_hue(180.0);
    Srgb::from_color(hue_shifted).into_format().into_raw()
    let rgb = Srgb::from_color(hsv).into_format().into_raw();
    println!("{:?}",rgb);

    /*
    use palette::{Pixel};
    let orangeish = Srgb::new(1.0, 0.6, 0.0).into_linear();

    // Encode the result back into sRGB and create a byte array
    let pixel: [u8; 3] = Srgb::from_linear(orangeish)
        .into_format()
        .into_raw();
    dbg!(pixel);
    */

    return;

    let mut buddha = Buddha::new(-3.1795, -3.1634, 400, 300.);
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

        let distribution = window
            .buffer
            .iter()
            .fold(BTreeMap::new(), |mut hash, value| {
                *hash.entry(value).or_insert(0) += 1;
                hash
            });
        dbg!(distribution);

        /*
        window
            .buffer
            .iter_mut()
            .for_each(|iter| *iter = (*iter as f64 / average * max as f64) as u32);
        */
        let max = *window.buffer.iter().max().unwrap();
        // dbg!(window.buffer.iter().enumerate().filter(|(_, i)| **i != 0).collect::<Vec<_>>());
        color::convert_nb_to_rbg(max, &mut window.buffer);

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
