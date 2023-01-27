use micro_jam_engine::{vek::*, Console, Game};

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

        // Update the player's paddle
        if console.input.key(Key::W).is_down() {
            self.player.paddle_pos += 1.0 * dt;
        }
        if console.input.key(Key::S).is_down() {
            self.player.paddle_pos -= 1.0 * dt;
        }

        // Update the AI's paddle
        self.ai.paddle_pos = self.ball_pos.y;

        // Update the ball
        self.ball_pos += self.ball_vel * dt;

        // Check if the ball has hit the paddle
        if self.ball_pos.x < -0.9 {
            if (self.ball_pos.y - self.player.paddle_pos).abs() < 0.1 {
                self.ball_vel.x = -self.ball_vel.x;
            }
        }

        // Check if the ball has hit the AI's paddle
        if self.ball_pos.x > 0.9 {
            if (self.ball_pos.y - self.ai.paddle_pos).abs() < 0.1 {
                self.ball_vel.x = -self.ball_vel.x;
            }
        }

        // Check if the ball has hit the top or bottom of the screen
        if self.ball_pos.y < -0.9 || self.ball_pos.y > 0.9 {
            self.ball_vel.y = -self.ball_vel.y;
        }

        // Check if the ball has gone off the left side of the screen
        if self.ball_pos.x < -1.0 {
            self.ball_pos = Vec2::new(0.0, 0.0);
            self.ball_vel = Vec2::new(0.0, 0.0);
            self.score += 1;
        }

        // Check if the ball has gone off the right side of the screen
        if self.ball_pos.x > 1.0 {
            self.ball_pos = Vec2::new(0.0, 0.0);
            self.ball_vel = Vec2::new(0.0, 0.0);
            self.score = 0;
        }

        // Draw the ball
        console.graphics.draw_rect(
            Rect::new(
                self.ball_pos - Vec2::new(0.01, 0.01),
                self.ball_pos + Vec2::new(0.01, 0.01),
            ),
            Color::WHITE,
        );

        // Draw the player's paddle
        console.graphics.draw_rect(
            Rect::new(
                Vec2::new(-0.9, self.player.paddle_pos - 0.1),
                Vec2::new(-0.8, self.player.paddle_pos + 0.1),
            ),
            Color::WHITE,
        );

        // Draw the AI's paddle
        console.graphics.draw_rect(
            Rect::new(
                Vec2::new(0.8, self.ai.paddle_pos - 0.1),
                Vec2::new(0.9, self.ai.paddle_pos + 0.1),
            ),
            Color::WHITE,
        );

        // Draw the score
        console.graphics.draw_text(
            &format!("Score: {}", self.score),
            Vec2::new(-0.9, 0.9),
            Color::WHITE,
        );

        // Draw the time
        console.graphics.draw_text(
            &format!("Time: {:.2}", self.time),
            Vec2::new(0.7, 0.9),
            Color::WHITE,
        );

        // Start the ball moving
        if self.ball_vel.x == 0.0 && self.ball_vel.y == 0.0 {
            self.ball_vel = Vec2::new(0.5, 0.5);
        }

        // Reset the game
        if console.input.key(Key::R).is_down() {
            self.ball_pos = Vec2::new(0.0, 0.0);
            self.ball_vel = Vec2::new(0.0, 0.0);
            self.score = 0;
            self.time = 0.0;
        }

        // Quit the game
        if console.input.key(Key::Escape).is_down() {
            console.quit();
        }
    }
}

fn main() {
    Color::run();
}
