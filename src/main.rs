use std::time::{Instant, Duration};
use std::thread;

use chip_8_emu::sound::RodioSound;
use chip_8_emu::{ChipEmulator, ChipEmulatorConfig, display::SdlDisplay, keypad::SdlKeypad};
use sdl2::event::{Event, WindowEvent};

const MAX_FRAME_RATE: f64 = 60.;

fn main() {
    // Initialize sdl contex and even pump
    let sdl_context = sdl2::init().expect("Couldn't initialize sdl2");
    let mut event_pump = sdl_context.event_pump().expect("Couldn't initialize event pump");

    // Initialize display and keypad
    let mut display =  SdlDisplay::new(&sdl_context, [0x00, 0xFF, 0xFF, 0xFF], [0, 0, 0, 0]).expect("Couldn't create display");
    let mut keypad = SdlKeypad::default();

    // Initialize sound system
    let sound = RodioSound::new(698., 0.3);

    // Initialize the emulator
    let config = ChipEmulatorConfig {
        instruction_per_second: 700,
        ..Default::default()
    };

    let mut emulator = ChipEmulator::initialize(config);
    emulator.load_rom("./rom/RPS.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/octojam1title.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/glitchGhost.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/1dcell.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/snake.ch8").expect("ROM loading error");
    //emulator.load_rom("./rom/test_audio.ch8").expect("ROM loading error");

    // Run emulator loop
    let mut timer = Instant::now();
    'running: loop {
        // Run the loop at a given frame rate
        let last_frame_time = timer.elapsed();
        timer = Instant::now();

        if Duration::from_secs_f64(1. / MAX_FRAME_RATE) >= last_frame_time {
            thread::sleep(Duration::from_secs_f64(1. / MAX_FRAME_RATE) - last_frame_time);
        }
        
        // Update bell status
        sound.update_bell(emulator.get_bell_status());

        // Update the emulator pressed key
        emulator.update_key(keypad.get_key());

        // If the emulator video buffer was updated update the screen
        let (video_buffer, buffer_updated) = emulator.get_video_buffer();
        if buffer_updated {
            display.update(video_buffer);
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

        // Run all the instruction for the frame as quickly as possible
        let cpu_time = timer.elapsed();

        let instructions = cpu_time.as_nanos() / emulator.get_cycle_duration().as_nanos();
        for _ in 0..=instructions {
            emulator.step();
        }
    }
}
