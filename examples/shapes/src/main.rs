use micro_jam_engine::{vek::*, Console, Game};

struct Shapes {}

impl Game for Shapes {
    const TITLE: &'static str = "Simple shapes";
    type SaveData = ();

    fn init(_console: &mut Console<Self>) -> Self {
        Self {}
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        // Draw a rectangle in the top left corner, with size 100x100. Make it
        // red, using the hex colour code in this case.
        console
            .graphics
            .draw_rect(Vec2::new(0, 0), Vec2::new(100, 100), 0xFF0000);

        // Draw a line from the top left corner to the bottom right corner.
        // Make it green, using the hex colour code in this case.
        console.graphics.draw_line(
            Vec2::new(0, 0),
            Vec2::new(
                console.graphics.size.x as i64,
                console.graphics.size.y as i64,
            ),
            0x00FF00,
        );

        // Draw a circle in the bottom right corner, with radius 50. Make it
        // blue, using the hex colour code in this case.
        console.graphics.draw_circle(
            Vec2::new(
                (console.graphics.size.x - 100) as i64,
                (console.graphics.size.y - 100) as i64,
            ),
            50,
            0x0000FF,
        );
    }
}

fn main() {
    Shapes::run();
}
