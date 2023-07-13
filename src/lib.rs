#![allow(dead_code)]
pub mod display;

use display::ChipDisplay;

pub const DEFAULT_FONT: [u8; 80] = [
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

/// Chip-8 emulator configuration struct
pub struct ChipEmulatorConfig {
    font: [u8; 80],
    instruction_per_second: u32,
    instruction_time: f64,
}

// implement Default trait for config
impl Default for ChipEmulatorConfig {
    fn default() -> Self {
        Self { 
            font: DEFAULT_FONT,
            instruction_per_second: 700,
            instruction_time: 1. / 700.,
        }
    }
}

/// Store all the components of a Chip-8 emulator
pub struct ChipEmulator<'a> {
    /// Store the configuration struct
    config: &'a ChipEmulatorConfig,

    /// 4KB program memory
    memory: [u8; 4096],
    /// The pointer to the current instruction
    program_counter: u16,
    /// Register used to point at location in memory
    index_pointer: u16,
    /// Program stack
    stack: Vec<u16>,
    /// Program registers
    registers: [u8; 16],

    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,

    /// The display implementation
    display: &'a mut dyn ChipDisplay,
}

// implement constructor and methods for the Chip-8 emulator
impl<'a> ChipEmulator<'a> {
    /// Instantiate and initialize a new Chip-8 emulator
    pub fn initialize(config: &'a ChipEmulatorConfig, display: &'a mut dyn ChipDisplay) -> Self {
        let mut emulator = Self {
            // Save the config
            config,

            // Initialize memory to zeros
            memory: [0u8; 4096],
            // Set the program counter to zero
            program_counter: 0u16,
            // Set index pointer to zero
            index_pointer: 0u16,
            // Create the stack
            stack: Vec::with_capacity(32),
            // Initialize registers to 0
            registers: [0u8; 16],
            // Initialize timer to 0
            delay_timer: 0u8,
            sound_timer: 0u8,

            // Display implementation
            display,
        };

        // Store the font in the program memory during initialization
        emulator.memory[0x050..=0x09F].copy_from_slice(&config.font);

        // Return the initialized emulator
        emulator
    }

    /// Print the content of a specific memory range for debug purposes
    pub fn print_memory(&self, from: usize, to: usize, width: u32) {
        for (i, value) in self.memory[from..=to].iter().enumerate() {
            // New line if the line len is grater that width
            if i % width as usize == 0 {
                // Print address and new line
                print!("\n0x{:02X}:  ", from + i);
            }

            // Print value
            print!("0x{:02X}  ", value);
        }

        // Final end line
        println!();
    }
}
