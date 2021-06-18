use std::collections::HashMap;

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

pub fn convert_nb_to_rbg(max: u32, window: &mut [u32]) {
    let mut truc = window.to_vec();
    truc.sort();
    truc.dedup();
    dbg!(truc.iter().sum::<u32>() / truc.len() as u32);
    dbg!(truc.get(truc.len() / 2));
    let ratio = 1. / truc.len() as f32;
    let map: HashMap<u32, usize> = truc.into_iter().enumerate().map(|(i, v)| (v, i + 1)).collect();


    window.iter_mut().for_each(|val| {
        let index = map[val];

        *val = hue_to_rgb(220., 0.30, index as f32 * ratio);
    });
}
