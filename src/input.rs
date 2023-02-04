use winit::{
    dpi::PhysicalPosition,
    event::{KeyboardInput, VirtualKeyCode},
};
use winit_input_helper::WinitInputHelper;

#[derive(Clone)]
pub struct Input {
    pub input_queue: Vec<InputEvent>,
    pub input_helper: WinitInputHelper,
}

impl Input {
    //pub fn key(&self, key: Key) -> KeyState { todo!() }
    //pub fn key_presses(&self) -> impl Iterator<Item = Key>;
    //pub fn axis(&self, axis: Axis) -> AxisState { todo!() }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    KeyboardInput(KeyboardInput),
    CursorMoved(PhysicalPosition<f64>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyboardState {
    key: VirtualKeyCode,
}

impl From<KeyboardInput> for KeyboardState {
    fn from(input: KeyboardInput) -> Self {
        Self {
            key: input.virtual_keycode.unwrap(),
        }
    }
}

pub enum KeyState {
    Pressed,
    NotPressed,
}

impl InputEvent {
    // Check if a key is pressed
    // pub fn key(&self, key: VirtualKeyCode) -> KeyState { todo!() }
    // pub fn key_presses(&self) -> impl Iterator<Item = Key>;
    // pub fn axis(&self, axis: Axis) -> AxisState { todo!() }
}
