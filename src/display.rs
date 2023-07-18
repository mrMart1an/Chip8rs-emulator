use std::cell::RefCell;

use sdl2::{Sdl, video::{Window, WindowContext}, render::{Canvas, TextureCreator}, pixels::PixelFormatEnum, rect::Rect};

pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;

/// 64 x 32 Display to present Chip-8 emulator frames
pub trait ChipDisplay {
    /// Update by drawing the buffer to the screen
    fn update(&mut self, video_buffer: &[u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]);
}

/*
*
*   Console based display Implementation
*
*/

#[derive(Default)]
pub struct ConsoleDisplay;

// Implement Chip Display for console display
impl ChipDisplay for ConsoleDisplay {
    /// Update by drawing the buffer to the screen
    fn update(&mut self, video_buffer: &[u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) {
        const PIXEL_ON: &str = "▓▓";
        const PIXEL_OFF: &str = "  ";

        for (i, v) in video_buffer.iter().enumerate() {
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

/*
*
*   Sdl2 display Implementation
*
*/

pub struct SdlDisplay {
    canvas: Canvas<Window>,

    texture_creator: TextureCreator<WindowContext>,
    texture_buffer: [u8; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize * 4],

    pub output_rect: RefCell<Rect>,

    /// On color at index: 1,
    /// Off color at index: 0
    pixel_color: [[u8; 4]; 2],
}

impl SdlDisplay {
    /// Create the display object from a given sdl contex
    /// Take the on and off color in BGRA format
    pub fn new(contex: &Sdl, on_color: [u8; 4], off_color: [u8; 4]) -> Result<Self, String> {
        let sdl_video = contex.video()?;
        let window = sdl_video.window("Chip-8 emulator", SCREEN_WIDTH * 200, SCREEN_HEIGHT * 200)
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

        // Create texture and canvas
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        let texture_buffer = [0xFF; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize * 4];

        // Create and output the display object
        let mut display = Self {
            canvas,

            texture_creator,
            texture_buffer,

            output_rect: RefCell::new(Rect::new(0, 0, 1, 1)),
            pixel_color: [off_color, on_color],
        };

        // Generate output rect
        display.resize(display.canvas.output_size().unwrap());

        // present the clear buffer to the window
        display.present_buffer();

        Ok(display)
    }

    /// Generate output rect from the window size
    pub fn resize(&self, window_size: (u32, u32)) {
        // Calculate the texture dimensions
        let aspect_ratio = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

        let mut width = window_size.0;
        let mut height = (width as f32 / aspect_ratio) as u32;

        let mut x_pos = 0;
        let mut y_pos = ((window_size.1 / 2) - (height / 2)) as i32;

        // If height is greater that window height recalculate the dimensions
        if window_size.1 <= height {
            height = window_size.1;    
            width = (height as f32 * aspect_ratio) as u32;

            x_pos = ((window_size.0 / 2) - (width / 2)) as i32;
            y_pos = 0;
        }

        let rect = Rect::new(x_pos, y_pos, width, height);

        // Set the output rect 
        *self.output_rect.borrow_mut() = rect;
    }

    /// Present the texture_buffer to the screen
    fn present_buffer(&mut self) {
        // Create the texture and write the buffer on it 
        let mut texture = self.texture_creator.create_texture_static(PixelFormatEnum::ARGB8888, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
        texture.update(None, &self.texture_buffer, SCREEN_WIDTH as usize * 4).unwrap();
        self.canvas.copy(&texture, None, *self.output_rect.borrow()).unwrap();

        // Present the texture on the screen 
        self.canvas.present();
    }
}

impl ChipDisplay for SdlDisplay {
    fn update(&mut self, video_buffer: &[u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) {
        // Set the pixel color in the texture buffer to the on color
        // if the video buffer pixel is active
        for (i, pixel) in video_buffer.iter().enumerate() {
            self.texture_buffer[i*4..i*4 + 4].copy_from_slice(&self.pixel_color[*pixel as usize]);
        }

        // Present the texture buffer
        self.present_buffer();
    }
}
