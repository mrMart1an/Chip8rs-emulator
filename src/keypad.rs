use std::cell::RefCell;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Chip key enum
#[derive(Clone, Copy)]
pub enum ChipKey {
    Key0 = 0x00,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

/// A keypad trait for a Chip-8 emulator
pub trait ChipKeypad {
    /// Get the current keypad pressed key
    fn get_key(&self) -> Option<ChipKey>;
}

/*
*
*   Sdl event based keypad Implementation
*
*/

#[derive(Default)]
pub struct SdlKeypad {
    key: RefCell<Option<ChipKey>> ,
}

/// Implement chip keypad trait for sdl keypad
impl ChipKeypad for SdlKeypad {
    fn get_key(&self) -> Option<ChipKey> {
        self.key.borrow().clone()
    }
}

/// Implement sdl keypad methods
impl SdlKeypad {
    pub fn process_sdl_event(&self, event: &Event) -> bool {
        match event {
            // Row 1
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key1);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key2);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key3);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyC);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            // Row 2
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key4);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key5);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key6);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyD);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            // Row 3
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key7);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key8);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key9);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyE);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            // Row 4
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyA);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::Key0);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyB);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                *self.key.borrow_mut() = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                *self.key.borrow_mut() = Some(ChipKey::KeyF);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                *self.key.borrow_mut() = None;
                true
            }
            _ => { false }
        }
    }
}

