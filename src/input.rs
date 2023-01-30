use winit::{event::{VirtualKeyCode, KeyboardInput}, dpi::PhysicalPosition};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    KeyboardInput(KeyboardInput),
    CursorMoved(PhysicalPosition<f64>),

}

pub enum KeyState {
    Pressed,
    NotPressed,
}

impl InputEvent {
    // Check if a key is pressed
    // pub fn key(&self, key: VirtualKeyCode) -> KeyState {
    //     match self.pressed_keys[key as usize] {
    //         true => KeyState::Pressed,
    //         false => KeyState::NotPressed,
    //     }
    // }
    // pub fn key_presses(&self) -> impl Iterator<Item = Key>;
    // pub fn axis(&self, axis: Axis) -> AxisState { todo!() }
}
