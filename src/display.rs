pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;

/// 64 x 32 Display to present Chip-8 emulator frames
pub trait ChipDisplay {
    /// Clear the display by setting all pixel to 0
    fn clear(&mut self);
    /// Return the raw display buffer
    fn get_buffer(&mut self) -> &mut [u8; 2048];

    /// Update by drawing the buffer to the screen 
    fn update(&self);
}

/// Simple display that print a frame to the console 
/// when a screen update occurs
pub struct ConsoleDisplay {
    buffer: [u8; 2048],
}

// Implement Default for console display
impl Default for ConsoleDisplay {
    fn default() -> Self {
        Self {
            buffer: [0u8; 2048],
        }
    }
}

// Implement Chip Display for console display
impl ChipDisplay for ConsoleDisplay {
    /// Clear the display by setting all pixel to 0
    fn clear(&mut self) {
        self.buffer = [0u8; 2048];
    }
    /// Return the raw display buffer
    fn get_buffer(&mut self) -> &mut [u8; 2048] {
        &mut self.buffer
    }

    /// Update by drawing the buffer to the screen 
    fn update(&self) {
        const PIXEL_ON: &str = "▓▓";   
        const PIXEL_OFF: &str = "  ";   

        for (i, v) in self.buffer.iter().enumerate() {
            // New line if a row was printed
            if i % SCREEN_WIDTH as usize == 0 {
                println!();
            }

            // Print pixel
            if *v != 0 {
                print!("{}", PIXEL_ON);
            } else {
                print!("{}", PIXEL_OFF);
            }    
        }
        
        // New ending line
        println!();
    }
}
