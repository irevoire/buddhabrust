use std::{collections::{BTreeMap, HashMap}};


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

pub fn convert_nb_to_rbg(iter: u32, window: &mut [u32]) {

    let retain_value = 30;
    let division_value = 3;
    
    // DISTRIBUTION LAND
    let mut sorted_window = window.to_vec();
    sorted_window.sort();
    let mut sorted_distribution = sorted_window.iter().fold(BTreeMap::new(), |mut hash, value| {
        *hash.entry(value).or_insert(0) += 1;
        hash
    });
    sorted_distribution.retain(|_, v| *v > retain_value);
    dbg!(&sorted_distribution);
    


    // RATIO LAND 
    let first_tier = (sorted_distribution.len() as f32 / division_value as f32).floor();
    let first_ratio = 0.5 / first_tier;
    let second_tier = sorted_distribution.len() as f32 - first_tier as f32;
    let second_ratio = 0.5 / second_tier;
    // println!("len: {}",sorted_distribution.len());
    // dbg!((first_tier, first_ratio, second_tier, second_ratio));

    // RATIO MAP LAND
    let mut itter_ratio = BTreeMap::new();
    let mut index = 0.0;
    for (key, value) in &sorted_distribution {
        if (index < first_tier) {
            itter_ratio.insert(*key, (index + 1.0) * first_ratio);
        }
        else {
            itter_ratio.insert(*key, (index - first_tier + 1.0) * second_ratio + 0.5);
        }
        index = index + 1.0;
    }
    dbg!(&itter_ratio);

    // PIXEL LAND
    window.iter_mut().for_each(|val| {
        if let Some(_) = itter_ratio.get(val) {
            *val = hue_to_rgb(220., 0.30, *itter_ratio.get(val).unwrap());
        } else {
        *val = u32::MAX;
            *val = 0;
        }
    });
}
