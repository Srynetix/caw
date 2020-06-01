// Renderer module

use super::logic::GameState;

mod console;
mod image;

/// Renderer
pub trait Renderer {
    /// Render game
    fn render(&mut self, state: &GameState);
}

pub use self::console::ConsoleRenderer;
pub use self::image::ImageRenderer;
