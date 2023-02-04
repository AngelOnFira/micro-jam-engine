use micro_jam_engine::{
    input::InputEvent, prelude::winit::event::VirtualKeyCode, vek::*, Console, Game,
};

/// This will be an implementation of pong. It will just be drawn with
/// rectangles, and will use simple collision detection to determine if the ball
/// has hit the paddle or the wall. It will also use a simple AI to control the
/// paddle.

const GAME_SPEED: f32 = 1.0;
const AI_MAX_SPEED: f32 = 500.0;

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
            ball_pos: Vec2::new(
                console.graphics.width() / 2.0,
                console.graphics.height() / 2.0,
            ),
            ball_vel: Vec2::new(500.0, 500.0),
            player: Player { paddle_pos: 100.0 },
            ai: Player { paddle_pos: 100.0 },
            score: 0,
            time: 0.0,
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.time += dt;

        let dt = dt * GAME_SPEED;

        console.graphics.clear(0x000000);

        // All numbers are in pixels, based on the size of the screen

        // Handle all inputs
        for input in &console.input.input_queue {
            match input {
                _ => {}
            }
        }

        // Check if W or S is pressed
        if console.input.input_helper.key_held(VirtualKeyCode::W)
            || console.input.input_helper.key_held(VirtualKeyCode::Up)
        {
            self.player.paddle_pos -= 500.0 * dt;
        }

        if console.input.input_helper.key_held(VirtualKeyCode::S)
            || console.input.input_helper.key_held(VirtualKeyCode::Down)
        {
            self.player.paddle_pos += 500.0 * dt;
        }

        // Set up the rectangles for the ball and paddles
        let ball_rect = Rect::new(self.ball_pos.x, self.ball_pos.y, 50.0, 50.0);

        let player_paddle_rect = Rect::new(100.0, self.player.paddle_pos, 40.0, 200.0);

        let ai_paddle_rect = Rect::new(
            console.graphics.size.x as f32 - 140.0,
            self.ai.paddle_pos,
            40.0,
            200.0,
        );

        // Update the AI's paddle
        if self.ball_pos.y > self.ai.paddle_pos + 100.0 {
            self.ai.paddle_pos += AI_MAX_SPEED * dt;
        } else if self.ball_pos.y < self.ai.paddle_pos + 100.0 {
            self.ai.paddle_pos -= AI_MAX_SPEED * dt;
        }

        // Update the ball
        self.ball_pos += self.ball_vel * dt;

        // Check if the ball has hit the left paddle and the velocity is going
        // left
        if ball_rect.collides_with_rect(player_paddle_rect) && self.ball_vel.x < 0.0 {
            self.ball_vel.x *= -1.0;
        }

        // Check if the ball has hit the right paddle and the velocity is going
        // right
        if ball_rect.collides_with_rect(ai_paddle_rect) && self.ball_vel.x > 0.0 {
            self.ball_vel.x *= -1.0;
        }

        // Check if the ball has hit the top or bottom of the screen
        if self.ball_pos.y < 0.0 || self.ball_pos.y > console.graphics.size.y as f32 {
            self.ball_vel.y *= -1.0;
        }

        // Check if the ball has hit the left or right of the screen
        if self.ball_pos.x < 0.0 || self.ball_pos.x > console.graphics.size.x as f32 {
            self.ball_vel.x *= -1.0;
        }

        // Draw the ball
        console.graphics.draw_rect(ball_rect, 0xFFFFFF);

        // Draw the player's paddle
        console.graphics.draw_rect(player_paddle_rect, 0x00FF00);

        // Draw the AI's paddle
        console.graphics.draw_rect(ai_paddle_rect, 0xFF0000);

        // Draw the score
    }
}

fn main() {
    Pong::run();
}
