use micro_jam_engine::{
    input::InputEvent,
    prelude::winit::event::VirtualKeyCode,
    vek::{num_traits::clamp, *},
    Console, Game,
};

/// This will be an implementation of pong. It will just be drawn with
/// rectangles, and will use simple collision detection to determine if the ball
/// has hit the paddle or the wall. It will also use a simple AI to control the
/// paddle.

const GAME_SPEED: f32 = 1.0;
const AI_MAX_SPEED: f32 = 80.0;

struct Ball {
    pos: Vec2<f32>,
    vel: Vec2<f32>,
}

impl Ball {
    fn draw(&self, console: &mut Console<Pong>) {
        console.graphics.draw_rect(
            Rect::new(self.pos.x, self.pos.y, 10.0, 10.0),
            0xFFFFFFFF,
            false
        );
    }

    fn update(&mut self, dt: f32, console: &mut Console<Pong>, player_paddle_rect: Rect<f32, f32>, ai_paddle_rect: Rect<f32, f32>) {
        let ball_rect = Rect::new(self.pos.x, self.pos.y, 10.0, 10.0); 

        self.pos += self.vel * dt;

        // Update the ball
        self.pos += self.vel * dt;

        // Check if the ball has hit the left paddle and the velocity is going
        // left
        if ball_rect.collides_with_rect(player_paddle_rect) && self.vel.x < 0.0 {
            self.vel.x = self.vel.x.abs();
            self.vel.y = (self.pos.y - player_paddle_rect.center().y) * 4.0;
        }

        // Check if the ball has hit the right paddle and the velocity is going
        // right
        if ball_rect.collides_with_rect(ai_paddle_rect) && self.vel.x > 0.0 {
            self.vel.x = -self.vel.x.abs();
            self.vel.y = (self.pos.y - ai_paddle_rect.center().y) * 4.0;
        }

        // Check if the ball has hit the top or bottom of the screen
        if self.pos.y < 15.0 || self.pos.y > console.graphics.height() - 15.0 - 12.0 {
            self.vel.y = -self.vel.y.abs()
                * (self.pos.y - (console.graphics.height() / 2.0)).signum();
        }

        // Check if the ball has hit the left or right of the screen
        if self.pos.x < 15.0 || self.pos.x > console.graphics.width() - 15.0 - 12.0 {
            self.vel.x = -self.vel.x.abs()
                * (self.pos.x - (console.graphics.width() / 2.0)).signum();
        }
    }
}

struct Pong {
    // /// The position of the ball
    // ball_pos: Vec2<f32>,
    // /// The velocity of the ball
    // ball_vel: Vec2<f32>,
    balls: Vec<Ball>,
    /// The player's paddle
    player: Player,
    /// The AI's paddle
    ai: Player,
    /// The score of the player
    score: u32,
    /// The game time
    time: f32,
}

struct Player {
    /// The position of the paddle
    paddle_pos: f32,
}

impl Game for Pong {
    const TITLE: &'static str = "Pong";
    type SaveData = ();

    fn init(console: &mut Console<Self>) -> Self {
        Self {
            balls: vec![Ball {
                pos: Vec2::new(
                    console.graphics.width() / 2.0,
                    console.graphics.height() / 2.0,
                ),
                vel: Vec2::new(100.0, 100.0),
            }],
            player: Player { paddle_pos: 25.0 },
            ai: Player { paddle_pos: 25.0 },
            score: 0,
            time: 0.0,
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.time += dt;

        let dt = dt * GAME_SPEED;

        // All numbers are in pixels, based on the size of the screen

        // Check if W or S is pressed
        if console.input.key_held(VirtualKeyCode::W) || console.input.key_held(VirtualKeyCode::Up) {
            self.player.paddle_pos -= 100.0 * dt;
        }

        if console.input.key_held(VirtualKeyCode::S) || console.input.key_held(VirtualKeyCode::Down)
        {
            self.player.paddle_pos += 100.0 * dt;
        }

        // if n is pressed, spawn a new ball
        if console.input.key_pressed(VirtualKeyCode::N) {
            self.balls.push(Ball {
                pos: Vec2::new(
                    console.graphics.width() / 2.0,
                    console.graphics.height() / 2.0,
                ),
                vel: Vec2::new(100.0, 100.0),
            });
        }

        // Make sure the paddle doesn't go too high
        self.player.paddle_pos = clamp(
            self.player.paddle_pos,
            10.0,
            console.graphics.height() - 50.0 - 15.0,
        );

        // Set up the rectangles for the ball and paddles

        let player_paddle_rect = Rect::new(25.0, self.player.paddle_pos, 10.0, 50.0);

        let ai_paddle_rect = Rect::new(
            console.graphics.size.x as f32 - 35.0,
            self.ai.paddle_pos,
            10.0,
            50.0,
        );

        // Update the AI's paddle
        let rightmost_ball = self.balls.iter().max_by_key(|ball| ball.pos.x as i32).unwrap();
        if rightmost_ball.pos.y > self.ai.paddle_pos + 25.0 {
            self.ai.paddle_pos += AI_MAX_SPEED * dt;
        } else if rightmost_ball.pos.y < self.ai.paddle_pos + 25.0 {
            self.ai.paddle_pos -= AI_MAX_SPEED * dt;
        }
        

        // Make sure the paddle doesn't go too high
        self.ai.paddle_pos = clamp(
            self.ai.paddle_pos,
            10.0,
            console.graphics.height() - 50.0 - 15.0,
        );

        // // Update the ball
        for ball in &mut self.balls {
            ball.update(dt, console, player_paddle_rect, ai_paddle_rect);
        }

        // Clear the screen
        console.graphics.clear(0x000000);

        // Draw the arena
        console.graphics.draw_rect(
            Rect::new(
                10.0,
                10.0,
                console.graphics.size.x as f32 - 25.0,
                console.graphics.size.y as f32 - 25.0,
            ),
            0x0000FF,
            false,
        );

        // Draw the ball
        // console.graphics.draw_rect(ball_rect, 0xFFFFFF, true);
        for ball in &self.balls {
            ball.draw(console);
        }

        // Draw the player's paddle
        console
            .graphics
            .draw_rect(player_paddle_rect, 0x00FF00, false);

        // Draw the AI's paddle
        console.graphics.draw_rect(ai_paddle_rect, 0xFF0000, false);

        // Draw the score
    }
}

fn main() {
    Pong::run();
}
