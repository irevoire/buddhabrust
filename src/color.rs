use rayon::prelude::*;
use std::collections::BTreeMap;

pub fn hue_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    assert!((0.0..=360.0).contains(&hue), "bad hue: {}", hue);
    assert!(
        (0.0..=1.0).contains(&saturation),
        "bad saturation: {}",
        saturation
    );
    assert!((0.0..=1.0).contains(&value), "bad value: {}", value);

    let c: f32 = saturation * value;
    let x: f32 = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()) as f32;
    let m: f32 = value - c;
    let (r, g, b) = match hue as u32 {
        0..=59 | 360 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => panic!("called with wrong value for hue"),
    };
    let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn scale(window: &[u32]) -> Vec<f32> {
    let division_value = 3;

    // DISTRIBUTION LAND
    let mut sorted_window = window.to_vec();
    sorted_window.sort();
    let retain_value = sorted_window
        .windows(2)
        .map(|window| (window[1] as f32 + 1.) / (window[0] as f32 + 1.))
        .position(|slope| slope > 2.)
        .unwrap_or(sorted_window.len() - 1);
    let retain_value = sorted_window[retain_value];
    let mut sorted_distribution = sorted_window
        .iter()
        .fold(BTreeMap::new(), |mut hash, value| {
            *hash.entry(value).or_insert(0) += 1;
            hash
        });
    sorted_distribution.retain(|_, v| *v > retain_value);

    // RATIO LAND
    let first_tier = (sorted_distribution.len() as f32 / division_value as f32).floor();
    let first_ratio = 0.5 / first_tier;
    let second_tier = sorted_distribution.len() as f32 - first_tier as f32;
    let second_ratio = 0.5 / second_tier;

    let itter_ratio: BTreeMap<u32, f32> = sorted_distribution
        .iter()
        .enumerate()
        .map(|(index, (&key, _))| {
            let index = index as f32;
            if index < first_tier {
                (*key, (index + 1.0) * first_ratio)
            } else {
                (*key, (index - first_tier + 1.0) * second_ratio + 0.5)
            }
        })
        .collect();

    window
        .iter()
        .map(|val| *itter_ratio.get(val).unwrap_or(&1.))
        .collect()
}

pub fn nb_iter_to_rgb(window: &mut [u32]) {
    let scale = scale(window);

    window.iter_mut().zip(scale).for_each(|(val, scale)| {
        *val = hue_to_rgb(153., 0.50, scale);
    });
}

pub fn merge_rgb_layers(window: &mut [u32], red: &[f32], green: &[f32], blue: &[f32]) {
    red.par_iter()
        .zip(green)
        .zip(blue)
        .map(|((r, g), b)| ((r * 255.) as u32, (g * 255.) as u32, (b * 255.) as u32))
        .map(|(r, g, b)| (0xff << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
        .zip(window.par_iter_mut())
        .for_each(|(color, pixel)| *pixel = color);
}
