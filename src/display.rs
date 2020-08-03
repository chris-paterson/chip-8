use std::u8;

pub struct Display {
    pub screen: [u8; 2048], // 64x32 pixels.
    pub should_draw: bool,
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [0; 2048],
            should_draw: false,
        }
    }

    // DEBUG
    pub fn set_pixel_on(&mut self, pixel: usize) {
        for i in 0..2048 {
            self.screen[i] = 0;
        }
        self.screen[pixel] = u8::MAX;
    }
}
