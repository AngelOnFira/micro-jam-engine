pub trait Game {
    type SaveState: Default;
    
    fn start(state: Self::SaveState) -> Self;
    
    fn save(&mut self) -> Self::SaveState;
    
    fn tick(&mut self, ctx: &mut Ctx);
}

pub struct Ctx<'tick> {
    pub dt: f32,

    pub input: &'tick mut Input,
    pub graphics: &'tick mut Graphics,
    pub sound: &'tick mut Sound,
}

pub struct Input;
pub struct Graphics;
pub struct Sound;
