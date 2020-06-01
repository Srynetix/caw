//! Image renderer module

use super::{GameState, Renderer};

/// Image renderer
pub struct ImageRenderer {
    pub size: (usize, usize),
    pub data: Vec<u8>,
    alive_color: (u8, u8, u8),
    dead_color: (u8, u8, u8),
}

impl ImageRenderer {
    pub fn new() -> Self {
        Self {
            size: (0, 0),
            data: vec![],
            alive_color: (244, 84, 255), // Purple
            dead_color: (0, 0, 0),       // Black
        }
    }

    pub fn set_size(&mut self, size: (usize, usize)) {
        self.size = size;
        self.data = vec![0; size.0 * size.1 * 4]; // 4 for RGBA
    }

    pub fn draw_rect(
        &mut self,
        (x, y): (usize, usize),
        (w, h): (usize, usize),
        color: (u8, u8, u8),
    ) {
        for oy in 0..h {
            for ox in 0..w {
                let cur = self.pos_to_index((x + ox, y + oy));
                self.data[cur] = color.0;
                self.data[cur + 1] = color.1;
                self.data[cur + 2] = color.2;
                self.data[cur + 3] = 255;
            }
        }
    }

    pub fn pos_to_index(&self, (x, y): (usize, usize)) -> usize {
        x * 4 + y * (self.size.0 * 4)
    }

    pub fn darken_color(&self, color: (u8, u8, u8), amount: u8) -> (u8, u8, u8) {
        (
            color.0.saturating_sub(amount),
            color.1.saturating_sub(amount),
            color.2.saturating_sub(amount),
        )
    }

    pub fn scale(&self, state: &GameState) -> (usize, usize) {
        (
            (self.size.0 as f32 / state.width as f32) as usize,
            (self.size.1 as f32 / state.height as f32) as usize,
        )
    }
}

impl Renderer for ImageRenderer {
    fn render(&mut self, state: &GameState) {
        let (cell_width, cell_height) = self.scale(state);

        for y in 0..state.height {
            for x in 0..state.width {
                let idx = state.pos_to_index((x, y));
                let alive = state.data[idx];
                let life = state.life[idx];
                let color = if alive {
                    self.darken_color(self.alive_color, life)
                } else {
                    self.dead_color
                };
                self.draw_rect(
                    (x * cell_width, y * cell_height),
                    (cell_width, cell_height),
                    color,
                );
            }
        }
    }
}
