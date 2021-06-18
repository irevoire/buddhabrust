use std::sync::atomic::{AtomicU32, Ordering};

use mandelbrust::Mandel;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub struct Buddha {
    pub pos: Pos,
    pub iter: u32,
    pub zoom: f64,
}

impl Buddha {
    pub fn new(x: f64, y: f64, iter: u32, zoom: f64) -> Self {
        Buddha {
            pos: Pos { x, y },
            iter,
            zoom,
        }
    }

    pub fn compute(&self, window: &mut [u32], width: usize, height: usize) {
        let mut mandel_window = window.to_vec();
        let mandel: Mandel = self.into();
        mandel.compute(&mut mandel_window, width, height);

        let window: &mut [AtomicU32] = unsafe { std::mem::transmute(window) };

        let range: Vec<usize> = (0..width).collect();
        range.par_iter().for_each(|x| {
            for y in 0..height {
                let index = x + y * width;
                let iteration = mandel_window[index];
                if (1..self.iter).contains(&iteration) {
                    self.bouddha(window, iteration, *x, y, width, height);
                }
            }
        });
    }

    fn bouddha(&self, window: &[AtomicU32], iter: u32, orig_x: usize, orig_y: usize, width: usize, height: usize) {
        let c_y = orig_x as f64 / self.zoom + self.pos.x as f64;
        let c_x = orig_y as f64 / self.zoom + self.pos.y as f64;
        let mut z_x = c_x;
        let mut z_y = c_y;
        let mut i = 0;

        if !(-2.0..2.).contains(&z_x) || !(-2.0..2.).contains(&z_y) {
            return;
        }

        while i <= iter {
            let tmp = z_x;
            z_x = z_x * z_x - z_y * z_y + c_x;
            z_y = 2.0 * z_y * tmp + c_y;

            let x = (z_x * self.zoom).round() as isize + orig_x as isize;
            let y = (z_y * self.zoom).round() as isize + orig_y as isize;
            if (0..width as isize).contains(&x) && (0..height as isize).contains(&y) {
                window[x as usize + y as usize * width].fetch_add(1, Ordering::Relaxed);
            }
            i += 1;
        }
    }
}

impl From<&Buddha> for mandelbrust::Mandel {
    fn from(buddha: &Buddha) -> Self {
        Self::new(buddha.pos.x, buddha.pos.y, buddha.iter, buddha.zoom)
    }
}
