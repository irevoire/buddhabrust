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
                if iteration < self.iter {
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

        while (z_x * z_x + z_y * z_y <= 4.0) && i <= self.iter {
            let x = (z_x * self.zoom) as usize + orig_x;
            let y = (z_y * self.zoom) as usize + orig_y;
            if x < width && y < height {
                window[x as usize + y as usize * width] += 1;
            }

            let tmp = z_x;
            z_x = z_x * z_x - z_y * z_y + c_x;
            z_y = 2.0 * z_y * tmp + c_y;

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
