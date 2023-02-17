use micro_jam_engine::{prelude::winit::event::VirtualKeyCode, vek::*, Console, Game};
use rand::{rngs::ThreadRng, thread_rng, Rng};

struct AvoidTheRoid {
    roids: Vec<Vec2<f32>>,
    rng: ThreadRng,
    player_x: f32,
    score: i64,
    best_score: i64,
    flame_color_index: usize,
    flame_radii_index: usize,
}

const PLAYER_SPEED: f32 = 50.0;
const PLAYER_SIZE: f32 = 10.0;

const MAX_ROIDS: usize = 20;
const MAX_ROID_START: f32 = 50.0;
const ROID_SIZE: f32 = 5f32;
const ROID_SPEED: f32 = 30.0;
const SPEED_FACTOR: f32 = 0.5;
const MAX_SCORE: i64 = 1000;

const FLAME_COLORS: &'static [u32] = &[0xffff00, 0xff0000, 0xff8800];
const FLAME_RADII: &'static [i64] = &[3, 6, 5, 2, 4];

impl Game for AvoidTheRoid {
    const TITLE: &'static str = "Avoid the 'Roid";
    type SaveData = ();

    fn init(console: &mut Console<Self>) -> Self {
        Self {
            roids: Vec::new(),
            rng: thread_rng(),
            player_x: (console.graphics.width() as f32) / 2.0,
            score: 0,
            best_score: 0,
            flame_color_index: 0,
            flame_radii_index: 0,
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        let w = console.graphics.width();
        let h = console.graphics.height();

        // Input:

        if console.input.key_held(VirtualKeyCode::A) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if console.input.key_held(VirtualKeyCode::D) {
            self.player_x += PLAYER_SPEED * dt;
        }

        self.player_x = self.player_x.clamp(0.0, w);

        // Update:

        let player_rect = Rect::new(
            self.player_x - PLAYER_SIZE / 2.0,
            h - PLAYER_SIZE - 10.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );
        // Spawn new asteroids.
        while self.roids.len() < MAX_ROIDS {
            let pos = Vec2::<f32>::new(
                self.rng.gen_range(0f32..w),
                -self.rng.gen_range(0f32..MAX_ROID_START),
            );
            self.roids.push(pos);
        }
        // Move asteroids down.
        let roid_speed = ROID_SPEED + SPEED_FACTOR * (self.score as f32);
        for roid in &mut self.roids {
            roid.y += roid_speed * dt;
        }

        // Clear out asteroids that we've passed, update score.
        let roids_before = self.roids.len();
        self.roids.retain(|roid| roid.y < h);
        let roids_after = self.roids.len();
        self.score += (roids_before - roids_after) as i64;
        self.score = self.score.clamp(0, MAX_SCORE);
        self.best_score = self.best_score.max(self.score);

        let roid_rects: Vec<_> = self
            .roids
            .iter()
            .map(|pos| {
                Rect::new(
                    pos.x - ROID_SIZE / 2.0,
                    pos.y - ROID_SIZE / 2.0,
                    ROID_SIZE,
                    ROID_SIZE,
                )
            })
            .collect();

        for roid_rect in &roid_rects {
            if player_rect.collides_with_rect(*roid_rect) {
                self.score = 0;
            }
        }

        // Render:

        // Clear the screen
        console.graphics.clear(0x000000);

        for roid_rect in &roid_rects {
            console.graphics.draw_rect(*roid_rect, 0xffffff, false);
        }

        self.flame_color_index += 1;
        self.flame_color_index %= FLAME_COLORS.len();
        self.flame_radii_index += 1;
        self.flame_radii_index %= FLAME_RADII.len();
        console.graphics.draw_circle(
            Vec2::new(
                self.player_x as i64,
                (player_rect.y + player_rect.h + 1.0) as i64,
            ),
            FLAME_RADII[self.flame_radii_index],
            FLAME_COLORS[self.flame_color_index],
        );
        console.graphics.draw_rect(player_rect, 0x888800, true);

        // Render the score as a bar, the outline shows the current best score.
        let score_height = (h - 10.0) * (self.score as f32) / (MAX_SCORE as f32);
        let best_score_height = (h - 10.0) * (self.best_score as f32) / (MAX_SCORE as f32);
        let score_rect = Rect::new(0.0, h - score_height - 5.0, 10.0, score_height);
        let best_score_rect = Rect::new(0.0, h - best_score_height - 5.0, 10.0, best_score_height);
        console.graphics.draw_rect(score_rect, 0x55cc55, true);
        console.graphics.draw_rect(best_score_rect, 0xffffff, false);
    }
}

fn main() {
    AvoidTheRoid::run();
}
