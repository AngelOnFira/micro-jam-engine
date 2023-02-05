use micro_jam_engine::vek::Vec2;

// A root is a number of links that try to follow the player
pub struct Root {
    pub links: Vec<Vec2<f32>>,
    pub state: RootState,
}

impl Root {}

pub enum RootState {
    Exploring,
    Eating,
    Attacking,
}