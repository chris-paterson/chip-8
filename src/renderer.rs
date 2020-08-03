extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode; // TODO temp
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const PIXEL_SIZE: u32 = 16;

pub struct Renderer {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
}

impl Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip 8", 64 * PIXEL_SIZE, 32 * PIXEL_SIZE) // TODO: Hardcoded
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        Renderer {
            canvas,
            sdl_context,
            event_pump,
        }
    }

    pub fn draw_screen(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }

                // Set background to black.
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.canvas.clear();

                // Draw pixels.
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.fill_rect(Rect::new(10, 10, 10, 10));
                self.canvas.present();
            }
        }
    }
}
