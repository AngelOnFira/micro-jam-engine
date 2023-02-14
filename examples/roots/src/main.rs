use food::Food;
use hecs::World;
use micro_jam_engine::{prelude::winit::event::VirtualKeyCode, vek::*, Console, Game};
use root::{Root, RootState};

mod food;
mod root;
mod timer;

struct Roots {
    player: Player,
    world: World,
    time: f32,
    roots: Vec<Root>,
    food: Food,
}

impl Roots {
    fn add_root(&mut self) {
        self.roots.push(Root {
            // Add 10 links that start at the player pos
            links: vec![self.player.pos; 50],
            state: RootState::Exploring { search_point: None },
        });
    }

    /// Each root wants to get further from the player. To do this, the root at
    /// the end will try to move away. Each root between the player and this end
    /// root will then try to move towards the root in front of it.
    fn move_roots(&mut self) {
        for root in self.roots.iter_mut() {
            match root.state {
                RootState::Exploring { search_point } => {
                    fn random_search_distance(n: f32) -> f32 {
                        rand::random::<f32>() * n - n / 2.0
                    }

                    // If the current search point is None, or we've gotten too
                    // far from the player, create a new one
                    if search_point.is_none()
                    // || (search_point.unwrap() - self.player.pos).magnitude() > 100.0
                    {
                        root.state = RootState::Exploring {
                            search_point: Some(Vec2::new(
                                random_search_distance(100.0),
                                random_search_distance(100.0),
                            )),
                        };
                        continue;
                    }

                    // The link at the end of the root wants to try and move to the
                    // closest food
                    let link = root.links.last_mut().unwrap();

                    // Find the closest food
                    let closest_food = self.food.pieces.iter().min_by(|a, b| {
                        let dist_a = (a.pos - *link).magnitude_squared();
                        let dist_b = (b.pos - *link).magnitude_squared();
                        dist_a.partial_cmp(&dist_b).unwrap()
                    });

                    // Check if we should start chasing a food

                    if closest_food.is_some() {
                        let food = closest_food.unwrap();

                        // If the food is close enough, then we'll change
                        // our state to chase it
                        if (food.pos - *link).magnitude() < 400.0 {
                            root.state = RootState::ChasingFood { food_pos: food.pos };
                            continue;
                        }
                    }

                    // If there is no food, then keep moving towards the search point
                    let move_pos = match search_point {
                        Some(search_point) => {
                            // If the search point is too close, find
                            // another search point
                            if (search_point - *link).magnitude_squared() < 100.0 {
                                // Pick a new search location that is
                                // within a 200 radius of the player
                                let search_point = Vec2::new(
                                    self.player.pos.x + random_search_distance(100.0),
                                    self.player.pos.y + random_search_distance(100.0),
                                );
                                root.state = RootState::Exploring {
                                    search_point: Some(search_point),
                                };
                                continue;
                            } else {
                                search_point
                            }
                        }
                        None => {
                            // If there is no search point, then we'll change
                            // our state to explore
                            root.state = RootState::Exploring { search_point: None };
                            continue;
                        }
                    };

                    // Move the link towards the food with a max speed of 10
                    let dir = (move_pos - *link).try_normalized().unwrap_or(Vec2::zero());

                    // Move the link faster if it's further away from its target
                    let speed = 10.0 * (move_pos - *link).magnitude() / 100.0;

                    // Move the link
                    *link += dir * speed;
                }
                RootState::ChasingFood { food_pos } => {
                    // The link at the end of the root wants to try and move to the
                    // closest food
                    let link = root.links.last_mut().unwrap();

                    // If the food is close enough, then we'll change
                    // our state to eating it
                    if (food_pos - *link).magnitude() < 100.0 {
                        root.state = RootState::Eating;
                        *link = food_pos;
                        continue;
                    }

                    // Move the link towards the food with a max speed of 10
                    let dir = (food_pos - *link).try_normalized().unwrap_or(
                        // Pick somewhere random on the screen
                        Vec2::new(
                            rand::random::<f32>() * 1000.0,
                            rand::random::<f32>() * 1000.0,
                        ),
                    );

                    // Move the link
                    *link += dir * 10.0;
                }
                RootState::Eating => {
                    // Find the food at this location, and reduce its remaining
                    // food
                    let food =
                        self.food.pieces.iter_mut().find(|food| {
                            (food.pos - root.links.last().unwrap()).magnitude() < 10.0
                        });

                    if let Some(food) = food {
                        food.remaining -= 1.0;

                        // If the food is gone, then we'll change our state to
                        // exploring
                        if food.remaining <= 0.0 {
                            root.state = RootState::Exploring { search_point: None };
                        }
                    } else {
                        // If there is no food, then we'll change our state to
                        // exploring
                        root.state = RootState::Exploring { search_point: None };
                    }
                }
                RootState::Attacking => todo!(),
            }
        }

        // For every link between the end of a root and the player, move the
        // link to the halfway point between itself and the link in front of it
        for root in self.roots.iter_mut() {
            for i in 0..root.links.len() - 1 {
                // If this is the first link, use the player as the link in
                // front of it
                let link_in_front = if i == 0 {
                    self.player.pos
                } else {
                    root.links[i - 1]
                };

                // Get the link after this one
                let link_after = root.links[i + 1];

                // Move the link to the halfway point between the link in front
                // and the link after
                root.links[i] = (link_in_front + link_after) / 2.0;
            }
        }
    }

    /// Draw the player and all the roots. The roots are drawn as a series of
    /// circles, while the player is drawn as a square.
    fn draw(&self, console: &mut Console<Self>) {
        let graphics = &mut console.graphics;

        // Draw the roots
        for root in self.roots.iter() {
            for (i, link) in root.links.iter().enumerate() {
                graphics.draw_circle(
                    Vec2::new(link.x as i64, link.y as i64),
                    if i + 1 == root.links.len() { 20 } else { 10 },
                    0xffffff,
                );
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
            world: World::new(),
            time: 0.0,
            roots: vec![],
            food: Food::new(),
        };

        // Add 10 roots
        for _ in 0..10 {
            roots.add_root();
        }

        // Add 2 food
        for _ in 0..3 {
            roots.food.add_food(&console.graphics);
        }

        // Debug the screen resolution
        println!(
            "Screen resolution: {} x {}",
            console.graphics.width(),
            console.graphics.height()
        );

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

        // Remove any food that has been eaten
        self.food.remove_eaten_food();

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
