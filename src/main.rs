use chip_8_emu::{ChipEmulator, ChipEmulatorConfig, display::SdlDisplay, keypad::SdlKeypad};
use sdl2::event::{Event, WindowEvent};

fn main() {
    // Initialize sdl contex and even pump
    let sdl_context = sdl2::init().expect("Couldn't initialize sdl2");
    let mut event_pump = sdl_context.event_pump().expect("Couldn't initialize event pump");

    // Initialize display and keypad
    let mut display =  SdlDisplay::new(&sdl_context, [0xFF, 0xFF, 0xFF, 0xFF], [0, 0, 0, 0]).expect("Couldn't create display");
    let mut keypad = SdlKeypad::default();

    // Initialize the emulator
    let config = ChipEmulatorConfig {
        instruction_per_second: 1000,
        ..Default::default()
    };

    let mut emulator = ChipEmulator::initialize(config);
    //emulator.load_rom("./rom/RPS.ch8").expect("ROM loading error");
    emulator.load_rom("./rom/octojam1title.ch8").expect("ROM loading error");

    // Run emulator loop
    'running: loop {
        emulator.start_cycle();

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

        emulator.update_key(keypad.get_key());

        emulator.step();

        let (video_buffer, buffer_updated) = emulator.get_video_buffer();
        if buffer_updated {
            display.update(video_buffer);
        }
    }
}
