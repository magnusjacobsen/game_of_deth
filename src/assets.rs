use crate::util;

use ggez::graphics::{self, Color, Rect, Font, Text};
use ggez::Context;

pub struct Assets {
    pub colored_cells: Vec<ggez::graphics::Mesh>,
    pub font: graphics::Font,
    pub text_start: graphics::Text,
    pub text_alive: graphics::Text,
    pub text_added: graphics::Text,
    pub text_steps: graphics::Text,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        let colored_cells = util::create_colored_cells(ctx);
        let font = Font::new(ctx, "/DejaVuSerifCondensed.ttf").unwrap();
        let text_start = Text::new(("Start: 0", font, 20.0));
        let text_alive = Text::new(("Alive: 0", font, 20.0));
        let text_added = Text::new(("Added: 0", font, 20.0));
        let text_steps = Text::new(("Time steps: 0", font, 20.0));

        Assets {
            colored_cells: colored_cells, 
            font: font, 
            text_added: text_added, 
            text_alive: text_alive,
            text_steps: text_steps,
            text_start: text_start,
        }
    }
}