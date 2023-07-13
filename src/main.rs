use chip_8_emu::{ChipEmulator, ChipEmulatorConfig, display::ConsoleDisplay};

fn main() {
    let config = ChipEmulatorConfig::default();
    let mut display = ConsoleDisplay::default();

    let _emulator = ChipEmulator::initialize(&config, &mut display);
}
