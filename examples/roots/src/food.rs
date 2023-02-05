use micro_jam_engine::{prelude::Graphics, vek::*};

use crate::timer;

pub struct Food {
    pub pieces: Vec<FoodPiece>,
    pub timer: timer::Timer,
}

impl Food {
    pub fn new() -> Self {
        Self {
            pieces: vec![],
            timer: timer::Timer::new(0.0, 10.0),
        }
    }

    pub fn draw_food(&self, graphics: &mut Graphics) {
        for piece in self.pieces.iter() {
            graphics.draw_circle(
                Vec2::new(piece.pos.x as i64, piece.pos.y as i64),
                piece.radius as i64,
                0x00ff00,
            );
        }
    }

    pub fn check_food_timer(&mut self, curr_time: f32, graphics: &Graphics) {
        if self.timer.is_complete(curr_time) {
            self.timer.start_time = curr_time;
            self.add_food(graphics);
        }
    }

    pub fn add_food(&mut self, graphics: &Graphics) {
        let pos = Vec2::new(
            rand::random::<f32>() * graphics.width() as f32,
            rand::random::<f32>() * graphics.height() as f32,
        );

        dbg!(pos);

        self.pieces.push(FoodPiece { pos, radius: 30.0 });
    }
}

pub struct FoodPiece {
    pub pos: Vec2<f32>,
    pub radius: f32,
}
