const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pub screen: [u8; 2048],
}

impl Display {
    pub fn new() -> Display {
        Display { screen: [0; 2048] }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.screen[x + y * WIDTH] = if on { 255 } else { 0 };
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.screen[x + y * WIDTH] == 1
    }

    pub fn clear(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                if new_value == 1 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;
                    let old_value = self.get_pixel(xi, yj);
                    if old_value {
                        collision = true;
                    }
                    self.set_pixel(xi, yj, (new_value == 1) ^ old_value);
                }
            }
        }
        return collision;
    }
}
