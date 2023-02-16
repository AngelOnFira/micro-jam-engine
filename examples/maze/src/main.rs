use micro_jam_engine::{vek::*, Console, Game, prelude::winit::event::VirtualKeyCode};
use rand::Rng;

const MAP_SIZE: i32 = 64;

struct Map([u64; 64]);

fn color_cell(cell: Vec2<i32>) -> Rgb<f32> {
    let r = ((cell.x.rotate_left(3)).overflowing_mul(167).0 as f32 / 13.0).fract().abs();
    let g = ((cell.y.rotate_left(5)).overflowing_mul(237).0 as f32 / 11.0).fract().abs();
    let b = ((cell.y.overflowing_mul(cell.x).0.rotate_right(3)).overflowing_mul(37).0 as f32 / 17.0).fract().abs();

    Rgb::new(r, g, b)
}

fn color_u32(color: Rgb<f32>) -> u32 {
    let color = color * 255.0;
    let color = color.as_::<u32>();

    (color.r << 16) |  (color.g << 8) | color.b
}

impl Map {
    fn get(&self, pos: Vec2<i32>) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= MAP_SIZE || pos.y >= MAP_SIZE {
            return true;
        }

        self.0[pos.y as usize] & (1 << pos.x as u32) != 0
    }

    fn trace(&self, origin: Vec2<f32>, dir: Vec2<f32>) -> (f32, Vec2<f32>, Rgb<f32>) {
        let step = dir.map(|e| e.signum());
        // TODO: Handle 0?
        let delta = step / dir;
        let mut tmax = step.map3(delta, origin, |s, d, o| {
            if s > 0.0 {
                d * (1.0 - o.fract())
            } else {
                d * o.fract()
            }
        });

        let mut pos = origin.map(|e| e.floor());
        let mut norm = Vec2::zero();
        let mut dist = 0.0;

        loop {
            let cell_pos = pos.as_();
            let cell = self.get(cell_pos);
            if cell || pos.map(|e| e.is_nan()).reduce_or() {
                break (dist, norm, color_cell(cell_pos));
            }

            if tmax.x < tmax.y {
                pos.x += step.x;
                dist = tmax.x;
                norm = Vec2::new(step.x, 0.0);
                tmax.x += delta.x;
            } else {
                pos.y += step.y;
                dist = tmax.y;
                norm = Vec2::new(0.0, step.y);
                tmax.y += delta.y;
            }
        }
    }
}


struct Maze {
    map: Map,
    player_pos: Vec2<f32>,
    player_angle: f32,

    map_open: bool,
}

impl Game for Maze {
    const TITLE: &'static str = "Simple shapes";
    type SaveData = ();

    fn init(_console: &mut Console<Self>) -> Self {
        let mut rand = rand::thread_rng();
        let map = Map([(); 64].map(|_| rand.gen()));

        Self {
            map,
            player_pos: Vec2::broadcast((MAP_SIZE - 1) as f32 / 2.0),
            player_angle: 0.0,

            map_open: false,
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {

        if console.input.input_helper.key_pressed_os(VirtualKeyCode::M) {
            self.map_open = !self.map_open;
        }

        let rot = console.input.input_helper.key_held(VirtualKeyCode::Q) as u8 as f32 - console.input.input_helper.key_held(VirtualKeyCode::E) as u8 as f32;

        self.player_angle += dt * rot * 2.0;

        let player_forward = Vec2::new(self.player_angle.cos(), self.player_angle.sin());
        let player_right = Vec2::new(player_forward.y, -player_forward.x);

        let forward = console.input.input_helper.key_held(VirtualKeyCode::W) as u8 as f32 - console.input.input_helper.key_held(VirtualKeyCode::S) as u8 as f32;
        let right = console.input.input_helper.key_held(VirtualKeyCode::D) as u8 as f32 - console.input.input_helper.key_held(VirtualKeyCode::A) as u8 as f32;

        self.player_pos += (forward * player_forward + right * player_right).try_normalized().unwrap_or(Vec2::zero()) * dt * 2.0;

        console.graphics.clear(color_u32(Rgb::broadcast(0.2)));

        console.graphics.draw_rect(Rect { x: 0.0, y: 0.0, w: console.graphics.width(), h: console.graphics.height() / 2.0 }, color_u32(Rgb::broadcast(0.8)), true);

        for col in 0..console.graphics.size.x {
            let col = col as f32;
            let t = (2.0 * col / console.graphics.width() - 1.0) * 1.2;

            let dir = (player_forward + player_right * t).normalized();

            let (dist, norm, color) = self.map.trace(self.player_pos, dir);

            let height = console.graphics.width() / (1.0 + dist.powf(2.0)).sqrt();

            let light = (norm.dot(player_forward) + 1.0) / 2.0;

            let color = color_u32(color * light);

            console.graphics.draw_rect(Rect { x: col, y: (console.graphics.height() - height) / 2.0, w: 1.0, h: height }, color, true)
        }

        if self.map_open {
            let size = (console.graphics.width().min(console.graphics.height()) - 20.0).max(0.0);
            let cell_size = (size / (MAP_SIZE + 2) as f32).floor();
            let size = cell_size * (MAP_SIZE + 2) as f32;

            let offset = (Vec2::new(console.graphics.width(), console.graphics.height()) - size) / 2.0;


            console.graphics.draw_rect(Rect { x: offset.x - 2.0, y: offset.y - 2.0, w: size + 4.0, h: size + 4.0 }, u32::MAX, true);

            for y in -1..MAP_SIZE + 1 {
                for x in -1..MAP_SIZE + 1 {
                    let cell = Vec2::new(x, y);
                    if self.map.get(cell) {
                        let t = (cell + 1).as_();
                        let o = offset + cell_size * t;
                        console.graphics.draw_rect(Rect { x: o.x, y: o.y, w: cell_size, h: cell_size }, color_u32(color_cell(cell)), true);
                    }
                }
            }

            let centerf = (offset + (self.player_pos + 1.0) * cell_size).clamped(Vec2::zero(), console.graphics.size.as_::<f32>() - 1.0);
            let center = centerf.as_();

            let radiusf = cell_size / 2.0 - 1.0;
            let radius = radiusf as i64;
            console.graphics.draw_line((centerf + player_forward * radiusf).as_(), (centerf + player_forward * radiusf * 2.0).as_(), 0);

            console.graphics.draw_circle(center, radius, 0);
            console.graphics.draw_circle(center, radius - 1, 0xFF0000);
        }
    }
}

fn main() {
    Maze::run();
}
