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
        for x in 0..width {
            for y in 0..height {
                let (iteration, z_x, z_y) = self.mandel(x, y);
                // dbg!(iteration);
                // window[x + y * width] = iteration;
                // if iteration > 3 && iteration < self.iter {
                if iteration < self.iter && (z_x * z_x + z_y * z_y) > 4. {
                    self.bouddha(window, x, y, width, height);
                }
            }
        }
    }

    fn bouddha(&self, window: &mut [u32], orig_x: usize, orig_y: usize, width: usize, height: usize) {
        let c_x = orig_x as f64 / self.zoom + self.pos.x as f64;
        let c_y = orig_y as f64 / self.zoom + self.pos.y as f64;
        let mut z_x = c_x;
        let mut z_y = c_y;
        let mut i = 0;

        if !(-2.0..2.).contains(&z_x) || !(-2.0..2.).contains(&z_y) {
            return;
        }

        while (z_x * z_x + z_y * z_y <= 4.0) && i <= self.iter {
            let tmp = z_x;
            z_x = z_x * z_x - z_y * z_y + c_x;
            z_y = 2.0 * z_y * tmp + c_y;

            let x = (z_x * self.zoom).round() as isize + orig_x as isize;
            let y = (z_y * self.zoom).round() as isize + orig_y as isize;
            if (0..width as isize).contains(&x) && (0..height as isize).contains(&y) {
                window[x as usize + y as usize * width] += 1;
            }
            i += 1;
        }
    }

    fn mandel(&self, orig_x: usize, orig_y: usize) -> (u32, f64, f64) {
        let c_x = orig_x as f64 / self.zoom + self.pos.x as f64;
        let c_y = orig_y as f64 / self.zoom + self.pos.y as f64;
        let mut z_x = c_x;
        let mut z_y = c_y;
        let mut i = 0;

        while (z_x * z_x + z_y * z_y <= 4.0) && i <= self.iter {
            let tmp = z_x;
            z_x = z_x * z_x - z_y * z_y + c_x;
            z_y = 2.0 * z_y * tmp + c_y;
            i += 1;
        }

        (i, z_x, z_y)
    }
}
