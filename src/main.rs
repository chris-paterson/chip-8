use std::fs;
use std::fs::File;

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

    fn load_game(&mut self, filename: &String) {
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

        println!("opcode: {}", opcode);
        // Decode opcode

        // Execute opcode

        // Update timers
    }

    fn set_keys(&mut self) {}
}

// Display

struct Display {
    screen: [u8; 2048], // 64x32 pixels.
    should_draw: bool,
}

impl Display {
    fn new() -> Self {
        Display {
            screen: [0; 2048],
            should_draw: false,
        }
    }

    fn draw(&mut self) {}
}

fn main() {
    let mut chip8 = Chip8::new();

    chip8.load_game(&String::from("./resources/roms/PONG"));

    print!("[");
    for a in chip8.memory.iter() {
        print!("{}, ", a);
    }
    print!("]");

    //loop {
    //    chip8.emulate_cycle();
    //    if chip8.display.should_draw {
    //        chip8.display.draw();
    //    }

    //    chip8.set_keys();
    //}
}
