use micro_jam_engine::prelude::*;

lazy_static! {
    static ref PACMAN: Sprite = sprite!("../pacman.png", count: 6);
    static ref TREAT: Sprite = sprite!("../treat.png");
}

struct Shapes {}

impl Game for Shapes {
    const TITLE: &'static str = "Sprites";
    type SaveData = ();

    fn init(_console: &mut Console<Self>) -> Self {
        Self {}
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        console.graphics.clear(0x00808080);

        console.graphics.draw_sprite(&PACMAN, Vec2::zero(), console.tick() / 6);

        console.graphics.draw_sprite(&TREAT, Vec2::new(32, 32), 0);
    }
}

fn main() {
    Shapes::run();
}
