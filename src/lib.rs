use std::marker::PhantomData;

pub trait Game: Sized {
    type SaveData: Default;
    
    fn init() -> Self;
    
    fn tick(&mut self, ctx: &mut Ctx<Self>);
}

pub struct Ctx<'tick, G: Game> {
    pub dt: f32, // Time passed since last tick, in seconds

    pub input: &'tick mut Input,
    pub graphics: &'tick mut Graphics,
    pub audio: &'tick mut Audio,
    pub save: &'tick mut Save<G::SaveData>,
}

pub struct Input;

impl Input {
    //pub fn key(&self, key: Key) -> KeyState { todo!() }
    //pub fn key_presses(&self) -> impl Iterator<Item = Key>;
    //pub fn axis(&self, axis: Axis) -> AxisState { todo!() }
}

pub struct Graphics;

impl Graphics {
    // TODO: Methods for drawing shapes, sprites, perhaps even triangles, as well as getting access to the framebuffer
}

pub struct Audio;

impl Audio {
    //pub fn play(&mut self, sound: Sound) { todo!() }
}

pub struct Save<S> {
    phantom: PhantomData<S>,
}

impl<S> Save<S> {
    pub fn read(&mut self) -> S { todo!() }
    pub fn write(&mut self, save: S) { todo!() }
}