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

/*
*
*   Sdl event based keypad Implementation
*
*/

#[derive(Default)]
pub struct SdlKeypad {
    key: Option<ChipKey> ,
}

/// Implement sdl keypad methods
impl SdlKeypad {
    /// Return the current key pressed variable
    pub fn get_key(&self) -> Option<ChipKey> {
        self.key    
    }

    /// Process an sdl key event to update the key pressed variable
    /// Return true if the event was processed
    pub fn process_sdl_event(&mut self, event: &Event) -> bool {
        match event {
            // Row 1
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                self.key = Some(ChipKey::Key1);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                self.key = Some(ChipKey::Key2);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                self.key = Some(ChipKey::Key3);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                self.key = Some(ChipKey::KeyC);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
                self.key = None;
                true
            }

            // Row 2
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                self.key = Some(ChipKey::Key4);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                self.key = Some(ChipKey::Key5);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                self.key = Some(ChipKey::Key6);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.key = Some(ChipKey::KeyD);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                self.key = None;
                true
            }

            // Row 3
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                self.key = Some(ChipKey::Key7);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.key = Some(ChipKey::Key8);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.key = Some(ChipKey::Key9);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                self.key = Some(ChipKey::KeyE);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                self.key = None;
                true
            }

            // Row 4
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                self.key = Some(ChipKey::KeyA);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                self.key = Some(ChipKey::Key0);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                self.key = Some(ChipKey::KeyB);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                self.key = None;
                true
            }

            Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                self.key = Some(ChipKey::KeyF);
                true
            }
            Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                self.key = None;
                true
            }
            _ => { false }
        }
    }
}

