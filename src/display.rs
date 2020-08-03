use crate::renderer::Renderer;

pub struct Display {
    pub screen: [u8; 2048], // 64x32 pixels.
    pub should_draw: bool,
    renderer: Renderer,
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [0; 2048],
            should_draw: false,
            renderer: Renderer::new(),
        }
    }

    pub fn draw(&mut self) {
        self.renderer.draw_screen();
    }
}

// Rendering - SDL2 stuff
impl Display {
    fn render_screen() {}
}
