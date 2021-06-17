use buddhabrust::Buddha;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode};
use std::time;

pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
    pub buffer: Vec<u32>, // color
}

impl Window {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let window = minifb::Window::new(
            "Buddhabrot",
            width,
            height,
            minifb::WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };
        let mut window = window.unwrap();
        window.limit_update_rate(Some(time::Duration::from_secs(1) / 30));

        Ok(Window {
            // if the window creation fail we exit everything
            window,
            width,
            height,
            buffer: vec![0; width * height],
        })
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap_or_else(|e| eprintln!("Window update failed: {}", e));
    }

    /// Update the buddha struct with the fetched event
    /// The user want to exit if this function return false
    pub fn handle_event(&mut self, buddha: &mut Buddha) -> bool {
        loop {
            self.window.update(); // needed in order to fetch the new events

            if !self.window.is_open() {
                return false;
            }

            let (width, height) = self.window.get_size();
            if width != self.width || height != self.height {
                self.width = width;
                self.height = height;
                self.buffer.resize(self.width * self.height, 0);
                return true;
            }

            if let Some(b) = self.handle_event_key(buddha) {
                return b;
            }
        }
    }

    fn handle_event_key(&self, buddha: &mut Buddha) -> Option<bool> {
        let mut res = None;

        if let Some(keys) = self.window.get_keys_pressed(KeyRepeat::Yes) {
            for k in &keys {
                match k {
                    Key::Escape => return Some(false),
                    Key::W | Key::Z | Key::Up => {
                        buddha.pos.y -= 100.0 / buddha.zoom;
                    }
                    Key::S | Key::Down => {
                        buddha.pos.y += 100.0 / buddha.zoom;
                    }
                    Key::A | Key::Q | Key::Left => {
                        buddha.pos.x -= 100.0 / buddha.zoom;
                    }
                    Key::D | Key::Right => {
                        buddha.pos.x += 100.0 / buddha.zoom;
                    }
                    Key::Space => {
                        buddha.pos.x += self.width as f64 * 0.25 / buddha.zoom;
                        buddha.pos.y += self.height as f64 * 0.25 / buddha.zoom;
                        buddha.zoom *= 2.0;
                    }
                    Key::X => {
                        buddha.zoom /= 2.0;
                        buddha.pos.x -= self.width as f64 * 0.25 / buddha.zoom;
                        buddha.pos.y -= self.height as f64 * 0.25 / buddha.zoom;
                    }
                    Key::I => {
                        buddha.iter += 1;
                    }
                    Key::U => {
                        buddha.iter -= 2;
                        if buddha.iter == 0 {
                            buddha.iter = 1;
                        }
                    }
                    _ => (),
                }
            }
            if !keys.is_empty() {
                res = Some(true);
            }
        };

        if self.window.get_mouse_down(MouseButton::Left) {
            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                buddha.pos.x += mouse.0 as f64 * 0.5 / buddha.zoom;
                buddha.pos.y += mouse.1 as f64 * 0.5 / buddha.zoom;
            });
            buddha.zoom *= 2.0;
            res = Some(true);
        }
        if self.window.get_mouse_down(MouseButton::Right) {
            self.window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                buddha.pos.x -= mouse.0 as f64 * 0.75 / buddha.zoom;
                buddha.pos.y -= mouse.1 as f64 * 0.75 / buddha.zoom;
            });
            res = Some(true);
            buddha.zoom /= 2.0;
        }
        if let Some((x, y)) = self.window.get_scroll_wheel() {
            buddha.pos.x -= x as f64 * 10. / buddha.zoom;
            buddha.pos.y -= y as f64 * 10. / buddha.zoom;
            res = Some(true);
        }
        res
    }

    /// return the dimensions of the window (width, height)
    pub fn dimension(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
