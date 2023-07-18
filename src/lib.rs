#![allow(dead_code)]
pub mod display;
pub mod keypad;

use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use keypad::{ChipKey, ChipKeypad};
use rand::{thread_rng, Rng};

use display::{ChipDisplay, SCREEN_WIDTH, SCREEN_HEIGHT};

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

const FONT_ADDRESS: usize = 0x050;

/// Chip-8 emulator configuration struct
pub struct ChipEmulatorConfig {
    pub font: [u8; 80],
    pub instruction_per_second: u32,

    /// Compatibility setting:
    /// During a shift operation copy the value form the registers Y
    /// into the registers X before shifting
    pub copy_y_on_shift: bool,

    /// Compatibility setting:
    /// In the BXNN instruction add the value of VX to XNN
    /// to obtain the offset value
    pub offset_jump_vx: bool,
}

// implement Default trait for config
impl Default for ChipEmulatorConfig {
    fn default() -> Self {
        Self {
            font: DEFAULT_FONT,
            instruction_per_second: 700,

            // Compatibility
            copy_y_on_shift: false,
            offset_jump_vx: false,
        }
    }
}

/// Chip-8 instruction struct
#[derive(Clone, Copy)]
struct ChipInstruction {
    pub raw: [u8; 2],

    pub op_code: u8,
    pub parameter: [u8; 3],
}

// Instruction constructor
impl ChipInstruction {
    /// Take a two byte instruction and return an instruction struct
    pub fn new(instruction: [u8; 2]) -> Self {
        Self {
            raw: instruction,

            op_code: instruction[0] >> 4,
            parameter: [
                instruction[0] & 0x0F,
                instruction[1] >> 4,
                instruction[1] & 0x0F,
            ],
        }
    }
}

// Implement Debug for chip instruction
impl Debug for ChipInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "op_code: 0x{:02X}, parameter: ", self.op_code)?;

        for v in self.parameter {
            write!(f, "0x{:02X}, ", v)?;
        }

        Ok(())
    }
}

/// Store all the components of a Chip-8 emulator
pub struct ChipEmulator<'a> {
    /// 4KB program memory
    memory: [u8; 4096],
    /// Video buffer to send to the screen implement on update
    video_buffer: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
    /// Program registers
    registers: [u8; 16],
    /// The pointer to the current instruction
    program_counter: u16,
    /// Register used to point at location in memory
    index_pointer: u16,
    /// Program stack
    stack: Vec<u16>,

    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,

    /// The key currently being pressed
    key: Option<ChipKey>,

    /// The display implementation
    display: &'a mut dyn ChipDisplay,
    /// The keypad implementation
    keypad: &'a dyn ChipKeypad,

    /// Clock used to keep the timer update at 60 Hz
    last_timer_update: Instant,
    /// Cycle duration timer
    cycle_timer: Instant,

    /// Store the configuration struct
    config: ChipEmulatorConfig,
}

// implement constructor and methods for the Chip-8 emulator
impl<'a> ChipEmulator<'a> {
    /// Instantiate and initialize a new Chip-8 emulator
    pub fn initialize(
        config: ChipEmulatorConfig, 
        display: &'a mut dyn ChipDisplay,
        keypad: &'a dyn ChipKeypad,
    ) -> Self {
        let mut emulator = Self {

            // Initialize memory to zeros
            memory: [0u8; 4096],
            // Initialize video buffer
            video_buffer: [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            // Set the program counter to 0x200
            program_counter: 0x200u16,
            // Set index pointer to zero
            index_pointer: 0u16,
            // Create the stack
            stack: Vec::with_capacity(32),
            // Initialize registers to 0
            registers: [0u8; 16],

            // Initialize timer to 0
            delay_timer: 0u8,
            sound_timer: 0u8,

            // Initialize input key to None
            key: None,

            // Display and keypad implementation
            display,
            keypad,

            // Set last timer update to now
            last_timer_update: Instant::now(),
            // Set the cycle timer to now
            cycle_timer: Instant::now(),

            // Save the config
            config,
        };

        // Store the font in the program memory during initialization
        emulator.memory[FONT_ADDRESS..FONT_ADDRESS + emulator.config.font.len()]
            .copy_from_slice(&emulator.config.font);

        // Return the initialized emulator
        emulator
    }

    /// Load a chip-8 rom from a file
    pub fn load_rom(&mut self, file_path: &str) -> Result<()> {
        const START_ADDRESS: usize = 0x200;

        // Open the rom file
        let path = Path::new(file_path);
        let mut f = File::open(path)?;
        let size = f.metadata()?.len() as usize;

        // Create memory slice and read the buffer in it
        let memory_slice = &mut self.memory[START_ADDRESS..START_ADDRESS + size];
        f.read_exact(memory_slice)?;

        // Set the program counter to the rom start address
        self.program_counter = START_ADDRESS as u16;

        Ok(())
    }

    /// Start the cycle timer
    pub fn start_cycle(&mut self) {
        self.cycle_timer = Instant::now();
    }

    /// Wait for the cycle end
    pub fn finish_cycle(&self) {
        // Loop duration
        let cycle_duration =
            Duration::from_secs_f64(1. / self.config.instruction_per_second as f64);

        // Calculate how long to wait before starting the next cycle
        // to maintain a constant loop speed
        let time_taken = self.cycle_timer.elapsed();

        if cycle_duration > time_taken {
            let wait_time = cycle_duration - time_taken;
            thread::sleep(wait_time);
        }
    }

    /// Run the emulator loop
    pub fn step(&mut self) {
        // Decrements the timers
        self.update_timer();

        // Read the pressed key from the key implementation
        self.key = self.keypad.get_key();

        // Fetch, decode and execute the instruction
        let instruction = self.fetch();
        self.decode_execute(instruction);
    }

    /// Decrements the delay and sound timer 60 times per seconds
    fn update_timer(&mut self) {
        if self.last_timer_update.elapsed() >= Duration::from_secs_f64(1. / 60.) {
            // Decrements timers if they are greater that 0
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            // Update last update timer
            self.last_timer_update = Instant::now();
        }
    }

    /// Fetch an 16 bit instruction at the program counter address
    /// and increment it by 2
    /// Return a Chip Instruction struct
    fn fetch(&mut self) -> ChipInstruction {
        // Read the instruction from memory
        let instruction_array = [
            self.memory[self.program_counter as usize],
            self.memory[self.program_counter as usize + 1],
        ];
        // Increment the program counter
        self.program_counter += 2;

        ChipInstruction::new(instruction_array)
    }

    /// Decode and execute the given instruction
    fn decode_execute(&mut self, instruction: ChipInstruction) {
        // Decode the instruction with a match statement
        match (instruction.op_code, instruction.parameter) {
            // Clear the screen
            (0x00, [0x00, 0x0E, 0x00]) => {
                self.video_buffer = [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
                self.display.update(&self.video_buffer);
            }

            // Jump instruction
            (0x01, [x, _, _]) => {
                let address = u16::from_be_bytes([x, instruction.raw[1]]);

                self.program_counter = address;
            }
            // Jump and push current PC to stack
            (0x02, [x, _, _]) => {
                let address = u16::from_be_bytes([x, instruction.raw[1]]);

                self.stack.push(self.program_counter);
                self.program_counter = address;
            }
            // Jump with offset register
            (0x0B, [x, _, _]) => {
                let address = u16::from_be_bytes([x, instruction.raw[1]]);

                // If the offset_jump_vx is enable use the value of VX for the offset
                // otherwise use the value of V0
                let reg_offset = if self.config.offset_jump_vx {
                    self.registers[x as usize]
                } else {
                    self.registers[0]
                };

                self.program_counter = address + reg_offset as u16;
            }
            // Pop an address from the stack and set the PC to it
            (0x00, [0x00, 0x0E, 0x0E]) => {
                self.program_counter = self.stack.pop().expect("Tried to pop an empty stack");
            }

            // 3XNN Skip instruction if value in registers X is equal to NN
            (0x03, [x, _, _]) => {
                let register_value = self.registers[x as usize];

                if register_value == instruction.raw[1] {
                    self.program_counter += 2;
                }
            }
            // 3XNN Skip instruction if value in registers X is not equal to NN
            (0x04, [x, _, _]) => {
                let register_value = self.registers[x as usize];

                if register_value != instruction.raw[1] {
                    self.program_counter += 2;
                }
            }
            // 5XY0 Skip instruction if value in registers X is equal to
            // the one in register Y
            (0x05, [x, y, 0]) => {
                let register_value_x = self.registers[x as usize];
                let register_value_y = self.registers[y as usize];

                if register_value_x == register_value_y {
                    self.program_counter += 2;
                }
            }
            // 9XY0 Skip instruction if value in registers X is not equal to
            // the one in register Y
            (0x09, [x, y, 0]) => {
                let register_value_x = self.registers[x as usize];
                let register_value_y = self.registers[y as usize];

                if register_value_x != register_value_y {
                    self.program_counter += 2;
                }
            }

            // Logical and mathematical instructions
            (0x08, parameter) => {
                self.alu(parameter);
            }

            // Set the register in parameter 0 to the value in raw 1
            (0x06, [x, _, _]) => {
                self.registers[x as usize] = instruction.raw[1];
            }
            // Add the value in raw 1 to the registers in parameter 0
            (0x07, [x, _, _]) => {
                self.registers[x as usize] =
                    instruction.raw[1].wrapping_add(self.registers[x as usize]);
            }

            // Set the index pointer to the value given by the instruction
            (0x0A, [x, _, _]) => {
                let address = u16::from_be_bytes([x, instruction.raw[1]]);

                self.index_pointer = address;
            }

            // Generate a random number and mask it
            (0x0C, [x, _, _]) => {
                let random_number: u8 = thread_rng().gen();

                self.registers[x as usize] = random_number & instruction.raw[1];
            }

            // Set the register X to the value of delay timer
            (0x0F, [x, 0x00, 0x07]) => {
                self.registers[x as usize] = self.delay_timer;
            }
            // Set the delay timer to the value in the register X
            (0x0F, [x, 0x01, 0x05]) => {
                self.delay_timer = self.registers[x as usize];
            }
            // Set the sound timer to the value in the register X
            (0x0F, [x, 0x01, 0x08]) => {
                self.sound_timer = self.registers[x as usize];
            }

            // Add the value in register X to the index register
            // In case of overflow (value fall outside of address range) set VF to 1
            (0x0F, [x, 0x01, 0x0E]) => {
                let value_x = self.registers[x as usize];

                // Set index pointer and VF register
                self.index_pointer += value_x as u16;
                self.registers[0x0F] = if self.index_pointer >= 0x1000 { 1 } else { 0 };
            }

            // FX55 Store the value of all the register from 0 to X in
            // continuous memory starting from the address in the index pointer
            (0x0F, [x, 0x05, 0x05]) => {
                for i in 0..=x {
                    let i = i as usize;
                    self.memory[self.index_pointer as usize + i] = self.registers[i];
                }
            }
            // FX65 Load the value of all the register from 0 to X from
            // continuous memory starting from the address in the index pointer
            (0x0F, [x, 0x06, 0x05]) => {
                for i in 0..=x {
                    let i = i as usize;
                    self.registers[i] = self.memory[self.index_pointer as usize + i];
                }
            }

            // Block the execution until a key press occur
            // and save the value in register X
            (0x0F, [x, 0x00, 0x0A]) => {
                if let Some(key) = self.key {
                    self.registers[x as usize] = key as u8;
                } else {
                    self.program_counter -= 2;
                }
            }
            // Skip the next instruction if the key in the register VX is being press
            (0x0E, [x, 0x09, 0x0E]) => {
                if let Some(key) = self.key {
                    if self.registers[x as usize] == key as u8 {
                        self.program_counter += 2;
                    }
                }
            }
            // Skip the next instruction if the key in the register VX is not being press
            (0x0E, [x, 0x0A, 0x01]) => {
                if let Some(key) = self.key {
                    if self.registers[x as usize] != key as u8 {
                        self.program_counter += 2;
                    }
                } else {
                    self.program_counter += 2;
                }
            }

            // Set the index register at the font address of the char in VX
            (0x0F, [x, 0x02, 0x09]) => {
                let char = self.registers[x as usize] & 0x0F;

                self.index_pointer = FONT_ADDRESS as u16 + (char as u16) * 5;
            }
            // Store each digit of the decimal number stored in the VX register
            // in 3 byte of continuous memory starting from the index pointer
            (0x0F, [x, 0x03, 0x03]) => {
                let number = self.registers[x as usize];

                let digits = [
                    number / 100,
                    number / 10 % 10,
                    number % 10,
                ];

                // Write digit into memory
                for (i, digit) in digits.iter().enumerate() {
                    self.memory[self.index_pointer as usize + i] = *digit;
                }
            }

            // Display draw instruction
            (0x0D, _) => {
                self.draw(instruction.parameter);
            }

            _ => {
                println!("Unrecognized instruction: {:?}", instruction);
            }
        }
    }

    /// Perform logical and mathematical functions
    fn alu(&mut self, parameter: [u8; 3]) {
        // Match the alu instruction
        match parameter[2] {
            // XY0 Set register X to the value of register Y
            0x00 => {
                self.registers[parameter[0] as usize] = self.registers[parameter[1] as usize];
            }
            // XY1 Set register X to (register Y | registers X)
            0x01 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                self.registers[parameter[0] as usize] = value_x | value_y;
            }
            // XY2 Set register X to (register Y & registers X)
            0x02 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                self.registers[parameter[0] as usize] = value_x & value_y;
            }
            // XY3 Set register X to (register Y ^ registers X)
            0x03 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                self.registers[parameter[0] as usize] = value_x ^ value_y;
            }
            // XY4 Set register X to (register Y + registers X)
            // set register F to 1 if an overflow occur to 0 if it doesn't
            0x04 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                let add_result = value_x.overflowing_add(value_y);
                self.registers[parameter[0] as usize] = add_result.0;

                // Set overflow flag
                self.registers[0x0F] = if add_result.1 { 1 } else { 0 };
            }
            // XY5 Set register X to (register X - registers Y)
            // set register F to 0 if an underflow occur to 1 if it doesn't
            0x05 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                let sub_result = value_x.overflowing_sub(value_y);
                self.registers[parameter[0] as usize] = sub_result.0;

                // Set underflow flag
                self.registers[0x0F] = if sub_result.1 { 0 } else { 1 };
            }
            // XY7 Set register X to (register Y - registers X)
            // set register F to 0 if an underflow occur to 1 if it doesn't
            0x07 => {
                let value_x = self.registers[parameter[0] as usize];
                let value_y = self.registers[parameter[1] as usize];

                let sub_result = value_y.overflowing_sub(value_x);
                self.registers[parameter[0] as usize] = sub_result.0;

                // Set underflow flag
                self.registers[0x0F] = if sub_result.1 { 0 } else { 1 };
            }

            // XY6 Set register X to register Y if config require it
            // then shift X to the right and set the register F to the value
            // of the shifted out bit
            0x06 => {
                if self.config.copy_y_on_shift {
                    self.registers[parameter[0] as usize] = self.registers[parameter[1] as usize];
                }
                let value_x = self.registers[parameter[0] as usize];

                self.registers[parameter[0] as usize] = value_x >> 1;

                // Set register F
                self.registers[0x0F] = value_x & 0b00000001;
            }
            // XYE Set register X to register Y if config require it
            // then shift X to the right and set the register F to the value
            // of the shifted out bit
            0x0E => {
                if self.config.copy_y_on_shift {
                    self.registers[parameter[0] as usize] = self.registers[parameter[1] as usize];
                }
                let value_x = self.registers[parameter[0] as usize];

                self.registers[parameter[0] as usize] = value_x << 1;

                // Set register F
                self.registers[0x0F] = value_x >> 7;
            }

            _ => {
                println!("Unrecognized alu instruction");
            }
        }
    }

    /// Draw the sprite to the index pointer address to the screen with an xor operation
    fn draw(&mut self, parameter: [u8; 3]) {
        // Decode the parameter
        let rows = parameter[2] as usize;

        let sprite_x = (self.registers[parameter[0] as usize] % 64) as usize;
        let sprite_y = (self.registers[parameter[1] as usize] % 32) as usize;

        // Get sprite and display buffer slices
        let sprite = &self.memory[self.index_pointer as usize..self.index_pointer as usize + rows];

        // Set VF register to 0
        self.registers[0x0F] = 0;

        for (row, sprite_row) in sprite.iter().enumerate() {
            // Calculate y and check for overflow
            let y = sprite_y + row;
            if y >= 32 {
                break;
            }

            // For every bit in one of the sprite byte update one pixel
            for bit_index in 0..8 {
                // Calculate x and check for overflow
                let x = (sprite_x + bit_index) % 64;

                // Get sprite and screen pixel values
                let sprite_pixel = (sprite_row << bit_index) & 0b10000000;
                let pixel = &mut self.video_buffer[SCREEN_WIDTH as usize * y + x];

                // If the sprite and screen pixel are both on
                // turn off the screen pixel and set VF to 1
                // If the sprite pixel is on and the screen pixel is off
                // turn on the screen pixel
                if sprite_pixel != 0 && *pixel != 0 {
                    // Set VF register to 0
                    self.registers[0x0F] = 1;
                    *pixel = 0;
                } else if sprite_pixel != 0 && *pixel == 0 {
                    *pixel = 1;
                }
            }
        }

        // Present video buffer to the screen
        self.display.update(&self.video_buffer);
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
