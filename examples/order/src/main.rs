use micro_jam_engine::{
    prelude::winit::event::VirtualKeyCode,
    vek::{*},
    Console, Game,
};

use importunate::*;

type Perm = Permutation<u8, 5>;


const TICKS: usize = 50;

struct Pong {
    /// The position of the ball
    perm_current: Perm,
    perm_prev: Perm,
    anim_ticks: usize,
}

fn interleave() -> Perm {
    Permutation::<u8, 5>::calculate_unchecked([0, 2, 4, 1, 3], |&x| x)
}

impl Game for Pong {
    const TITLE: &'static str = "Order";
    type SaveData = ();

    fn init(_console: &mut Console<Self>) -> Self {
        Self {
            perm_current: Perm::default()
                .combine(&Perm::rotate_left())
                .combine(&interleave())
                .combine(&Perm::reverse())
                .combine(&Perm::rotate_right()),
            perm_prev: Perm::default()
                .combine(&Perm::rotate_left())
                .combine(&interleave())
                .combine(&Perm::reverse())
                .combine(&Perm::rotate_right()),
            anim_ticks: TICKS,
        }
    }

    fn tick(&mut self, _dt: f32, console: &mut Console<Self>) {
        self.anim_ticks = self.anim_ticks.saturating_sub(1);
        if console.input.key_pressed(VirtualKeyCode::W) {
            self.perm_prev = self.perm_current;
            self.anim_ticks = TICKS;
            self.perm_current = self.perm_current.combine(&Perm::reverse())
        } else if console.input.key_pressed(VirtualKeyCode::A) {
            self.perm_prev = self.perm_current;
            self.anim_ticks = TICKS;
            self.perm_current = self.perm_current.combine(&Perm::rotate_left())
        } else if console.input.key_pressed(VirtualKeyCode::S) {
            self.perm_prev = self.perm_current;
            self.anim_ticks = TICKS;
            self.perm_current = self.perm_current.combine(&interleave())
        } else if console.input.key_pressed(VirtualKeyCode::D) {
            self.perm_prev = self.perm_current;
            self.anim_ticks = TICKS;
            self.perm_current = self.perm_current.combine(&Perm::rotate_right())
        }
        // console.input.input_queue.clear();

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

        for (position, size) in self.perm_current.get_array().iter().enumerate() {
            let old_position = self.perm_prev.index_of(&size, |&&x| x) as usize;
            let anim_position = ((position * (TICKS - self.anim_ticks))
                + (old_position * self.anim_ticks)) as f32
                / ((TICKS) as f32);

            let mut color = match *size {
                0 => 0xFF0000,
                1 => 0x880088,
                2 => 0x0000FF,
                3 => 0x888800,
                _ => 0x008888,
            };
            if self.perm_current == Perm::default() {
                color = 0x00FF00;
            }
            let size = (size + 1) as f32 * 4.;

            let x = (console.graphics.size.x as f32 - 25.) / 6. * (anim_position + 1.) as f32;
            let rect = Rect::new(
                x + (-size / 2.),
                console.graphics.size.y as f32 / 2.,
                size,
                size,
            );
            console.graphics.draw_rect(rect, color, true);
        }
    }
}

fn main() {
    Pong::run();
}
