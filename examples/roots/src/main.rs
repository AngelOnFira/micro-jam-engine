use micro_jam_engine::{
    input::InputEvent,
    prelude::{winit::event::VirtualKeyCode, Graphics},
    vek::{num_traits::clamp, *},
    Console, Game,
};

mod timer;

struct Roots {
    player: Player,
    time: f32,
    roots: Vec<Root>,
    food: Food,
}

impl Roots {
    fn add_root(&mut self) {
        self.roots.push(Root {
            // Add 10 links that start at the player pos
            links: vec![self.player.pos; 10],
        });
    }

    /// Each root wants to get further from the player. To do this, the root at
    /// the end will try to move away. Each root between the player and this end
    /// root will then try to move towards the root in front of it.
    fn move_roots(&mut self) {
        for root in self.roots.iter_mut() {
            // The link at the end of the root wants to move away from the player
            let end_link = root.links.last_mut().unwrap();
            let dir = (self.player.pos - *end_link).normalized();

            // If the direction is NaN, then replace it with a random direction
            let dir = if dir.x.is_nan() || dir.y.is_nan() {
                Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
            } else {
                dir
            };

            *end_link += dir * 0.5;
            // println!("dir: {:?}", dir);
            // println!("end_link: {:?}", end_link);

            // // Each root between the player and the end root wants to move towards
            // // the root in front of it
            for i in (1..root.links.len() - 1).rev() {
                let dir = (root.links[i - 1] - root.links[i]).normalized();

                // If the direction is NaN, then replace it with a random
                // direction
                let dir = if dir.x.is_nan() || dir.y.is_nan() {
                    Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                } else {
                    dir
                };

                root.links[i] += dir * 0.5;
            }
        }
    }

    /// Draw the player and all the roots. The roots are drawn as a series of
    /// circles, while the player is drawn as a square.
    fn draw(&self, console: &mut Console<Self>) {
        let graphics = &mut console.graphics;

        // Draw the roots
        for root in self.roots.iter() {
            for link in root.links.iter() {
                graphics.draw_circle(Vec2::new(link.x as i64, link.y as i64), 20, 0xffffff);
            }
        }

        // Draw the player. This comes second so that the player is drawn on top
        graphics.draw_rect(
            Rect::new(self.player.pos.x - 5.0, self.player.pos.y - 5.0, 10.0, 10.0),
            0xffffff,
            false,
        );
    }
}

// A root is a number of links that try to follow the player
struct Root {
    links: Vec<Vec2<f32>>,
}

impl Root {}

struct Food {
    pieces: Vec<FoodPiece>,
    timer: timer::Timer,
}

impl Food {
    fn new() -> Self {
        Self {
            pieces: vec![],
            timer: timer::Timer::new(0.0, 10.0),
        }
    }

    fn draw_food(&self, graphics: &mut Graphics) {
        for piece in self.pieces.iter() {
            graphics.draw_circle(
                Vec2::new(piece.pos.x as i64, piece.pos.y as i64),
                piece.radius as i64,
                0x00ff00,
            );
        }
    }

    fn check_food_timer(&mut self, curr_time: f32, graphics: &Graphics) {
        if self.timer.is_complete(curr_time) {
            self.timer.start_time = curr_time;
            self.add_food(graphics);
        }
    }

    fn add_food(&mut self, graphics: &Graphics) {
        let pos = Vec2::new(
            rand::random::<f32>() * graphics.width() as f32,
            rand::random::<f32>() * graphics.height() as f32,
        );

        dbg!(pos);

        self.pieces.push(FoodPiece { pos, radius: 30.0 });
    }
}

struct FoodPiece {
    pos: Vec2<f32>,
    radius: f32,
}

const GAME_SPEED: f32 = 100.0;

struct Player {
    pos: Vec2<f32>,
}

impl Game for Roots {
    const TITLE: &'static str = "Roots";
    type SaveData = ();

    fn init(console: &mut Console<Self>) -> Self {
        let mut roots = Roots {
            player: Player {
                pos: Vec2::new(100.0, 100.0),
            },
            time: 0.0,
            roots: vec![],
            food: Food::new(),
        };

        roots.add_root();

        roots
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.time += dt;

        let dt = dt * GAME_SPEED;

        let move_multiplier = 4.5;

        // Make a player circle move with the arrow keys or WASD
        if console.input.input_helper.key_held(VirtualKeyCode::Left)
            || console.input.input_helper.key_held(VirtualKeyCode::A)
        {
            self.player.pos.x -= dt * move_multiplier;
        }
        if console.input.input_helper.key_held(VirtualKeyCode::Right)
            || console.input.input_helper.key_held(VirtualKeyCode::D)
        {
            self.player.pos.x += dt * move_multiplier;
        }
        if console.input.input_helper.key_held(VirtualKeyCode::Up)
            || console.input.input_helper.key_held(VirtualKeyCode::W)
        {
            self.player.pos.y -= dt * move_multiplier;
        }
        if console.input.input_helper.key_held(VirtualKeyCode::Down)
            || console.input.input_helper.key_held(VirtualKeyCode::S)
        {
            self.player.pos.y += dt * move_multiplier;
        }

        // Logic
        // -----

        // Check the food timer
        self.food.check_food_timer(self.time, &console.graphics);

        // Run the roots progression
        self.move_roots();

        // Graphics
        // --------

        // Clear the screen
        console.graphics.clear(0x000000);

        // Draw the food
        self.food.draw_food(&mut console.graphics);

        // Draw the player
        self.draw(console);
    }
}

fn main() {
    Roots::run();
}
