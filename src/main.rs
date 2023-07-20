use std::time::{Instant, Duration};
use std::thread;

use chip_8_emu::{ChipEmulator, ChipEmulatorConfig, display::SdlDisplay, keypad::SdlKeypad};
use sdl2::event::{Event, WindowEvent};

const MAX_FRAME_RATE: f64 = 60.;

fn main() {
    // Initialize sdl contex and even pump
    let sdl_context = sdl2::init().expect("Couldn't initialize sdl2");
    let mut event_pump = sdl_context.event_pump().expect("Couldn't initialize event pump");

    // Initialize display and keypad
    let mut display =  SdlDisplay::new(&sdl_context, [0xFF, 0xFF, 0xFF, 0xFF], [0, 0, 0, 0]).expect("Couldn't create display");
    let mut keypad = SdlKeypad::default();

    // Initialize the emulator
    let config = ChipEmulatorConfig {
        instruction_per_second: 500,
        ..Default::default()
    };

    let mut emulator = ChipEmulator::initialize(config);
    //emulator.load_rom("./rom/RPS.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/octojam6title.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/glitchGhost.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/1dcell.ch8").expect("ROM loading error");
    emulator.load_rom("./rom/snake.ch8").expect("ROM loading error");

    let mut timer = Instant::now();

    // Run emulator loop
    'running: loop {
        // Run the loop at a given frame rate
        let last_frame_time = timer.elapsed();
        timer = Instant::now();
        if Duration::from_secs_f64(1. / MAX_FRAME_RATE) >= last_frame_time {
            thread::sleep(Duration::from_secs_f64(1. / MAX_FRAME_RATE) - last_frame_time);
        }

        // Run all the instruction for the last frame as quickly as possible
        let instructions = last_frame_time.as_micros() / emulator.get_cycle_duration().as_micros();
        for _ in 0..=instructions {
            emulator.step();
        }

        // Handle events
        for event in event_pump.poll_iter() {
            if !keypad.process_sdl_event(&event) {
                match event {
                    Event::Quit { .. } => { break 'running; }
                    Event::Window { 
                        win_event: WindowEvent::Resized(x, y), .. 
                    } => {
                            display.resize((x as u32, y as u32));
                        }
            
                    _ => {}
                }
            }
        }
        
        // Update the emulator pressed key
        emulator.update_key(keypad.get_key());

        // If the emulator video buffer was updated update the screen
        let (video_buffer, buffer_updated) = emulator.get_video_buffer();
        if buffer_updated {
            display.update(video_buffer);
        }
    }
}
