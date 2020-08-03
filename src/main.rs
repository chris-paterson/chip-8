#[allow(dead_code)]
mod display;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use display::Display;
use std::fs;
use std::fs::File;
use std::u8;

const PIXEL_SIZE: u32 = 16;
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

// Chip-8

struct Chip8 {
    memory: [u8; 4096],
    pc: u16,
    i: u16,              // Index register pointing to locations in memory.
    stack: u16,          // Used to call subroutines and return from them.
    sp: u8,              // Current place on the stack
    delay_timer: u8,     // Decrements at 60 Hz until it reaches 0.
    sound_timer: u8,     // Like the delay timer. Plays beeping sound if not 0.
    registers: [u8; 16], // V0 -> VF. VF is used as a flag register.
    display: Display,
}

impl Chip8 {
    fn new() -> Self {
        let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let mut memory: [u8; 4096] = [0; 4096];
        for (i, value) in font_set.iter().enumerate() {
            memory[i] = *value;
        }
        Chip8 {
            memory,
            pc: 0,
            i: 0,
            stack: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            display: Display::new(),
        }
    }
    /// Loads the game at the specified path into memory.
    fn load_game(&mut self, filename: &String) {
        // TODO: Return error instead of panicking.
        match std::fs::read(filename) {
            Ok(bytes) => {
                // Load game into memory.
                for (i, value) in bytes.iter().enumerate() {
                    self.memory[512 + i] = *value;
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);
        // Decode opcode
        match (opcode) {
            0xA000...0xAFFF => {
                // ANNN: Set I to the address NNN
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }
            _ => println!("Unknown opcode: {}", opcode),
        }

        // Execute opcode

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    fn set_keys(&mut self) {}
}

fn main() {
    let mut chip8 = Chip8::new();

    // Setup window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Chip 8",
            SCREEN_WIDTH * PIXEL_SIZE,
            SCREEN_HEIGHT * PIXEL_SIZE,
        )
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    chip8.load_game(&String::from("./resources/roms/PONG"));

    let mut a: usize = 0;

    let frame_rate = 1000 / 60; // Desired FPS
    let mut timer = sdl_context.timer().unwrap();
    'running: loop {
        let frame_start = timer.ticks();

        chip8.emulate_cycle();
        chip8.set_keys();

        chip8.display.set_pixel_on(a % (64 * 32));
        a += 1;

        println!("{}", a);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Set background to black.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw pixels.
        for (i, pixel) in chip8.display.screen.iter().enumerate() {
            let x = (i as u32) % SCREEN_WIDTH;
            let y = (i as u32) / SCREEN_WIDTH;

            if *pixel == u8::MAX {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(
                    (x * PIXEL_SIZE) as i32,
                    (y * PIXEL_SIZE) as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                ));
            }
        }

        canvas.present();

        let frame_end = timer.ticks();
        let time_delta = frame_end - frame_start;

        if frame_rate > time_delta {
            let sleep_time = (frame_rate - time_delta) as u64;
            std::thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
