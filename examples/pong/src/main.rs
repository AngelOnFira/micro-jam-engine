use micro_jam_engine::{input::InputEvent, vek::*, Console, Game};

/// This will be an implementation of pong. It will just be drawn with
/// rectangles, and will use simple collision detection to determine if the ball
/// has hit the paddle or the wall. It will also use a simple AI to control the
/// paddle.

struct Pong {
    /// The position of the ball
    ball_pos: Vec2<f32>,
    /// The velocity of the ball
    ball_vel: Vec2<f32>,
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
            ball_pos: Vec2::new(0.0, 0.0),
            ball_vel: Vec2::new(0.0, 0.0),
            player: Player { paddle_pos: 0.0 },
            ai: Player { paddle_pos: 0.0 },
            score: 0,
            time: 0.0,
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.time += dt;

        // All numbers are in pixels, based on the size of the screen

        // Handle all inputs
        for input in &console.input {
            match input {
                InputEvent::KeyboardInput(input) => match input.scancode {
                    17 => {
                        self.player.paddle_pos += 1.0 * dt;
                    }
                    31 => {
                        self.player.paddle_pos -= 1.0 * dt;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Update the AI's paddle
        self.ai.paddle_pos = self.ball_pos.y;

        // Update the ball
        self.ball_pos += self.ball_vel * dt;

        // Check if the ball has hit the left paddle
        if self.ball_pos.x < self.player.paddle_pos - 10.0
            && self.ball_pos.y > self.player.paddle_pos - 10.0
            && self.ball_pos.y < self.player.paddle_pos + 10.0
        {
            self.ball_vel.x = -self.ball_vel.x;
        }

        // Check if the ball has hit the right paddle
        if self.ball_pos.x > self.ai.paddle_pos + 10.0
            && self.ball_pos.y > self.ai.paddle_pos - 10.0
            && self.ball_pos.y < self.ai.paddle_pos + 10.0
        {
            self.ball_vel.x = -self.ball_vel.x;
        }

        // Check if the ball has hit the top or bottom of the screen
        if self.ball_pos.y < console.graphics.size.y as f32 / -2.0
            || self.ball_pos.y > console.graphics.size.y as f32 / 2.0
        {
            self.ball_vel.y = -self.ball_vel.y;
        }

        // Check if the ball has hit the left or right of the screen
        if self.ball_pos.x < console.graphics.size.x as f32 / -2.0 {
            self.ball_pos = Vec2::new(0.0, 0.0);
            self.ball_vel = Vec2::new(0.0, 0.0);
            self.score += 1;
        } else if self.ball_pos.x > console.graphics.size.x as f32 / 2.0 {
            self.ball_pos = Vec2::new(0.0, 0.0);
            self.ball_vel = Vec2::new(0.0, 0.0);
            self.score = 0;
        }
    }
}

fn main() {
    Pong::run();
}
