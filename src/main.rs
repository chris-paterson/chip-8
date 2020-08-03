mod display;
mod keypad;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand::Rng;

use display::Display;
use keypad::Keypad;
use std::u8;

const PIXEL_SIZE: u32 = 16;
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

struct Chip8 {
    memory: [u8; 4096],
    pc: u16,
    i: u16,              // Index register pointing to locations in memory.
    stack: [u16; 16],    // Used to call subroutines and return from them.
    sp: u8,              // Current place on the stack
    delay_timer: u8,     // Decrements at 60 Hz until it reaches 0.
    sound_timer: u8,     // Like the delay timer. Plays beeping sound if not 0.
    registers: [u8; 16], // V0 -> VF. VF is used as a flag register.
    display: Display,
    keypad: Keypad,
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
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }
    /// Loads the game at the specified path into memory.
    fn load_game(&mut self, filename: &String) {
        // TODO: Return error instead of panicking.
        match std::fs::read(filename) {
            Ok(bytes) => {
                // Load game into memory.
                for (i, value) in bytes.iter().enumerate() {
                    self.memory[0x200 + i] = *value;
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
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers[x];
        let vy = self.registers[y];
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        self.pc += 2;

        // println!("{}, {}, {}, {}", op_1, op_2, op_3, op_4);

        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => self.display.clear(),
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            (0x3, _, _, _) => self.pc += if vx == nn { 2 } else { 0 },
            (0x4, _, _, _) => self.pc += if vx != nn { 2 } else { 0 },
            (0x5, _, _, 0x0) => self.pc += if vx == vy { 2 } else { 0 },
            (0x6, _, _, _) => self.registers[x] = nn,
            (0x7, _, _, _) => {
                let (val, _) = self.registers[x].overflowing_add(nn);
                self.registers[x] = val;
            }
            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y],
            (0x8, _, _, 0x1) => self.registers[x] |= self.registers[y],
            (0x8, _, _, 0x2) => self.registers[x] &= self.registers[y],
            (0x8, _, _, 0x3) => self.registers[x] ^= self.registers[y],
            (0x8, _, _, 0x4) => {
                let (val, did_overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[0xF] = if did_overflow { 1 } else { 0 };
                self.registers[x] = val;
            }
            (0x8, _, _, 0x5) => {
                let (val, did_overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[0xF] = if did_overflow { 0 } else { 2 };
                self.registers[x] = val;
            }
            (0x8, _, _, 0x6) => {
                self.registers[0xF] = self.registers[x] & 0x1;
                self.registers[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                let (val, did_overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[0xF] = if did_overflow { 0 } else { 2 };
                self.registers[x] = val;
            }
            (0x8, _, _, 0xE) => {
                // TODO: Correct?
                let most_significant = self.registers[x] >> 7;
                self.registers[0xF] = most_significant;
                self.registers[x] <<= 1;
            }
            (0x9, _, _, 0x0) => self.pc += if vx != vy { 2 } else { 0 },
            (0xA, _, _, _) => self.i = nnn,
            (0xB, _, _, _) => self.pc = nnn + self.registers[0] as u16,
            (0xC, _, _, _) => {
                let mut rand = rand::thread_rng();
                self.registers[x] = rand.gen_range(0, 255) as u8 & nn
            }
            (0xD, _, _, _) => {
                let flipped = self.display.draw(
                    vx as usize,
                    vy as usize,
                    &self.memory[self.i as usize..(self.i + n as u16) as usize],
                );
                self.registers[0xF] = if flipped { 1 } else { 0 };
            }
            (0xE, _, 0x9, 0xE) => self.pc += if self.keypad.is_key_down(vx) { 2 } else { 0 },
            (0xE, _, 0xA, 0x1) => self.pc += if self.keypad.is_key_down(vx) { 0 } else { 2 },
            (0xF, _, 0x0, 0x7) => self.registers[x] = self.delay_timer,
            (0xF, _, 0x0, 0xA) => {
                // TODO: Correct?
                self.pc -= 2;
                for (i, key) in self.keypad.keys.iter().enumerate() {
                    if *key == true {
                        self.registers[x] = i as u8;
                        self.pc += 2;
                    }
                }
            }
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.registers[x],
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.registers[x],
            (0xF, _, 0x1, 0xE) => self.i += self.registers[x] as u16,
            (0xF, _, 0x2, 0x9) => self.i = vx as u16 * 5,
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = (vx % 100) % 10;
            }
            (0xF, _, 0x5, 0x5) => self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]
                .copy_from_slice(&self.registers[0..(x as usize + 1)]),
            (0xF, _, 0x6, 0x5) => self.registers[0..(x as usize + 1)]
                .copy_from_slice(&self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]),
            (_, _, _, _) => println!("Unknown opcode: {}, {}, {}, {}", op_1, op_2, op_3, op_4),
        }

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

    chip8.load_game(&String::from("./resources/roms/INVADERS"));

    let mut fps_manager = FPSManager::new();
    match fps_manager.set_framerate(60) {
        Ok(_) => println!("Framerate set"),
        Err(error) => println!("Framerate not set: {}", error),
    }

    let mut timer = sdl_context.timer().unwrap();
    'running: loop {
        let frame_start = timer.ticks();

        chip8.emulate_cycle();
        chip8.set_keys();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Num1 => chip8.keypad.key_down(0),
                    Keycode::Num2 => chip8.keypad.key_down(1),
                    Keycode::Num3 => chip8.keypad.key_down(2),
                    Keycode::Num4 => chip8.keypad.key_down(3),
                    Keycode::Q => chip8.keypad.key_down(4),
                    Keycode::W => chip8.keypad.key_down(5),
                    Keycode::E => chip8.keypad.key_down(6),
                    Keycode::R => chip8.keypad.key_down(7),
                    Keycode::A => chip8.keypad.key_down(8),
                    Keycode::S => chip8.keypad.key_down(9),
                    Keycode::D => chip8.keypad.key_down(10),
                    Keycode::F => chip8.keypad.key_down(11),
                    Keycode::Z => chip8.keypad.key_down(12),
                    Keycode::X => chip8.keypad.key_down(13),
                    Keycode::C => chip8.keypad.key_down(14),
                    Keycode::V => chip8.keypad.key_down(15),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Num1 => chip8.keypad.key_up(0),
                    Keycode::Num2 => chip8.keypad.key_up(1),
                    Keycode::Num3 => chip8.keypad.key_up(2),
                    Keycode::Num4 => chip8.keypad.key_up(3),
                    Keycode::Q => chip8.keypad.key_up(4),
                    Keycode::W => chip8.keypad.key_up(5),
                    Keycode::E => chip8.keypad.key_up(6),
                    Keycode::R => chip8.keypad.key_up(7),
                    Keycode::A => chip8.keypad.key_up(8),
                    Keycode::S => chip8.keypad.key_up(9),
                    Keycode::D => chip8.keypad.key_up(10),
                    Keycode::F => chip8.keypad.key_up(11),
                    Keycode::Z => chip8.keypad.key_up(12),
                    Keycode::X => chip8.keypad.key_up(13),
                    Keycode::C => chip8.keypad.key_up(14),
                    Keycode::V => chip8.keypad.key_up(15),
                    _ => {}
                },
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

            // We only need to draw pixels that are on since the background has
            // been cleared to black already.
            if *pixel == 1 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                match canvas.fill_rect(Rect::new(
                    (x * PIXEL_SIZE) as i32,
                    (y * PIXEL_SIZE) as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                )) {
                    Ok(_) => {}
                    Err(err) => panic!(err),
                };
            }
        }

        canvas.present();

        fps_manager.delay();

        // STATS
        let frame_end = timer.ticks();
        let time_delta = frame_end - frame_start;
        println!("fps: {}", 1000 / time_delta);
    }
}
