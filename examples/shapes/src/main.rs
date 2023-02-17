use micro_jam_engine::input::Input;
use micro_jam_engine::prelude::winit::event::VirtualKeyCode;
use micro_jam_engine::{vek::*, Console, Game};
use rand::Rng;

struct Drifter {
    size: Vec2<usize>,
    highscore: usize,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    angle: f32,
    angular_vel: f32,
    boost: f32,
    circles: Vec<InwardCircle>,
    total_time: f32,
    rng: rand::rngs::ThreadRng,
}

struct InwardCircle {
    at: Vec2<f32>,
    size: f32,
}

impl Game for Drifter {
    const TITLE: &'static str = "Drifter";
    type SaveData = usize;

    fn init(console: &mut Console<Self>) -> Self {
        let size = console.graphics.size;
        let highscore = console.save.read();
        Self {
            size,
            highscore,
            position: Default::default(),
            velocity: Default::default(),
            angle: std::f32::consts::FRAC_PI_2,
            angular_vel: 0.0,
            boost: 0.0,
            circles: vec![],
            total_time: 0.0,
            rng: rand::rngs::ThreadRng::default(),
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.draw(console);
        self.update(dt, &console.input);
        return;
    }
}

impl Drifter {
    fn draw(&self, console: &mut Console<Self>) {
        console.graphics.clear(0x000000);

        for circle in &self.circles {
            console
                .graphics
                .draw_circle(self.to_screen_space(circle.at), (circle.size * 16.).min(24.) as i64, 0xAA4400);
            console
                .graphics
                .draw_circle(self.to_screen_space(circle.at), ((circle.size * 16.).min(24.) - 2.) as i64, 0x000000);
        }
        
        console.graphics.draw_rect(Rect {
            x: 16.,
            y: 16.,
            w: (self.size.x as f32 - 32.),
            h: (self.size.y as f32 - 32.),
        }, 0xAAAAAA, false);

        let player_col = (((4. - self.boost) * 256.) as u32).max(0).min(255);

        self.draw_poly(
            console,
            [
                0.5 * Self::point_on_circle(self.angle) + self.position,
                0.5 * Self::point_on_circle(self.angle + 2.25 * std::f32::consts::FRAC_PI_3) + self.position,
                0.5 * Self::point_on_circle(self.angle + 3.75 * std::f32::consts::FRAC_PI_3) + self.position,
                0.5 * Self::point_on_circle(self.angle) + self.position,
            ],
            0x00FF00 | player_col | player_col << 4,
        );
    }

    fn point_on_circle(theta: f32) -> Vec2<f32> {
        Vec2::new(theta.cos(), theta.sin())
    }

    fn draw_poly<T: Into<Vec2<f32>>, const N: usize>(&self, console: &mut Console<Self>, points: [T; N], color: u32) {
        let points = points.map(|p| p.into());
        for i in 0..(N - 1) {
            console
                .graphics
                .draw_line(self.to_screen_space(points[i]), self.to_screen_space(points[i + 1]), color)
        }
    }

    fn update(&mut self, dt: f32, input: &Input) {
        self.total_time += dt;
        self.boost -= 0.2 * dt;
        self.boost = self.boost.max(0.);

        let ang_accel = if self.boost > 0. { 11. } else { 5. };

        if input.key_held(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::Left) {
            self.angular_vel -= ang_accel * dt;
        }
        if input.key_held(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::Right) {
            self.angular_vel += ang_accel * dt;
        }

        self.angular_vel *= (1. - 1.5 * dt);
        self.angle += self.angular_vel * dt;

        let accel_amount = if self.boost > 0. { 11. } else { 3. };

        if input.key_held(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::Up) {
            self.velocity += accel_amount * dt * Self::point_on_circle(self.angle);
        }
        if input.key_held(VirtualKeyCode::S) || input.key_held(VirtualKeyCode::Down) {
            self.velocity -= accel_amount * dt * Self::point_on_circle(self.angle);
        }

        self.velocity *= (1. - dt);
        self.position += self.velocity * dt;

        if self.position.x <= -(self.size.x as f32 / 32.) + 1. {
            self.position.x = -(self.size.x as f32 / 32.) + 1.;
            self.velocity.x *= -0.8;
        }

        if self.position.x >= self.size.x as f32 / 32. - 1. {
            self.position.x = self.size.x as f32 / 32. - 1.;
            self.velocity.x *= -0.8;
        }

        if self.position.y <= -(self.size.y as f32 / 32.) + 1. {
            self.position.y = -(self.size.y as f32 / 32.) + 1.;
            self.velocity.y *= -0.8;
        }

        if self.position.y >= self.size.y as f32 / 32. - 1. {
            self.position.y = self.size.y as f32 / 32. - 1.;
            self.velocity.y *= -0.8;
        }

        for circle in &mut self.circles {
            circle.size -= 0.5 * dt;
            if circle.size <= 1. && (circle.at - self.position).magnitude_squared() <= 0.8 {
                self.boost += 8.;
            } else if (circle.at - self.position).magnitude_squared() <= 0.8 {
                self.boost += 2.*dt;
            }
        }
        self.circles.retain(|c| c.size > 1.);
        if self.circles.len() < 3 {
            let x = self.rng.gen_range(10..(self.size.x - 10));
            let y = self.rng.gen_range(10..(self.size.y - 10));

            self.circles.push(InwardCircle {
                at: self.to_world_space(Vec2::new(x as i64, y as i64)),
                size: self.rng.gen_range(1.4..2.4),
            })
        }
    }

    fn to_screen_space(&self, world: Vec2<f32>) -> Vec2<i64> {
        (16. * world + (self.size.map(|p| p as f32) / 2.)).map(|p| p as _)
    }

    fn to_world_space(&self, screen: Vec2<i64>) -> Vec2<f32> {
        (screen.map(|p| p as f32) - (self.size.map(|p| p as f32) / 2.)) / 16.
    }
}

fn main() {
    Drifter::run();
}
