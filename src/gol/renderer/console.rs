//! Console renderer module

use super::{GameState, Renderer};

/// Console renderer
pub struct ConsoleRenderer;

impl ConsoleRenderer {
    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn draw_header(&self, state: &GameState) {
        let tick_len = state.current_tick.to_string().len();
        let header_width = state.width + 2;

        print!("╔═({})═", state.current_tick);

        for _ in 0..header_width - tick_len - 6 {
            print!("═");
        }

        println!("╗");
    }

    pub fn draw_footer(&self, state: &GameState) {
        print!("╚");

        for _ in 0..state.width {
            print!("═");
        }

        println!("╝");
    }

    pub fn draw_line(&self, state: &GameState, y: usize) {
        print!("║");
        for x in 0..state.width {
            let cell = state.data[state.pos_to_index((x, y))];
            print!("{}", if cell { "█" } else { " " });
        }

        println!("║");
    }
}

impl Renderer for ConsoleRenderer {
    fn render(&mut self, state: &GameState) {
        self.clear_screen();
        self.draw_header(state);

        for y in 0..state.height {
            self.draw_line(state, y);
        }

        self.draw_footer(state);
    }
}
