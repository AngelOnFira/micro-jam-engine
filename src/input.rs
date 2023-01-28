use winit::event::VirtualKeyCode;

pub struct Input {
    pub pressed_keys: [bool; 256],
}

pub enum KeyState {
    Pressed,
    NotPressed,
}

impl Input {
    /// Check if a key is pressed
    pub fn key(&self, key: VirtualKeyCode) -> KeyState {
        match self.pressed_keys[key as usize] {
            true => KeyState::Pressed,
            false => KeyState::NotPressed,
        }
    }
    // pub fn key_presses(&self) -> impl Iterator<Item = Key>;
    // pub fn axis(&self, axis: Axis) -> AxisState { todo!() }
}
