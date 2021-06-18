pub fn hue_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    assert!((0.0..=100.0).contains(&saturation), "saturation: {}", saturation);
    assert!((0.0..=100.0).contains(&value), "value: {}", value);
    assert!((0.0..=360.0).contains(&hue), "hue: {}", hue);

    let c: f32 = saturation * value;
    let m: f32 = value - c;
    let x: f32 = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()) as f32;

    /*
    let cm = ((c + m) * 255.).round();
    let xm = ((x + m) * 255.).round();
    */

    let (r, g, b) = match hue as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => return 0,
    };
    let (r, g, b) = ((r + m).abs() * 255.0, (g + m).abs() * 255.0, (b + m).abs() * 255.0);
    dbg!(r, g, b);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn convert_nb_to_rbg(max: u32, window: &mut [u32]) {
    window.iter_mut().for_each(|val| {
        if *val == max {
            *val = 0x0000_0000;
        } else if *val < 3 {
            *val = 0;
        } else if true {
            let hsv = palette::Hsv::new(194., 100., *val as f32 * (100.0 / max as f32));
            let rgb: palette::Srgb = hsv.into();
            let (r, g, b) = (rgb.red, rgb.green, rgb.blue);
            let rgb = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            *val = rgb;

                /*
                hue_to_rgb(
                194.,
                100.,
                *val as f32 * (100.0 / max as f32),
            );
                */
        } else {
            *val = hue_to_rgb(
                *val as f32 * (360.0 / max as f32),
                1.0,
                *val as f32 / max as f32,
            );
        }
    });
}
