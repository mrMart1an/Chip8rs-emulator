use chip_8_emu::{ChipEmulator, ChipEmulatorConfig, display::ConsoleDisplay};

fn main() {
    let config = ChipEmulatorConfig {
        instruction_per_second: 700,
        ..Default::default()
    };
    let mut display = ConsoleDisplay::default();

    let mut emulator = ChipEmulator::initialize(&config, &mut display);
    emulator.load_rom("./rom/test_opcode.ch8").expect("ROM loading error");

    emulator.run();
}
