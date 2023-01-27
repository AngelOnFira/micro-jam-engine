use micro_jam_engine::{Game, Console, vek::*};
use euc::{Pipeline, Buffer2d, Target, TriangleList, IndexedVertices, Texture};

struct Cube {
    mvp: Mat4<f32>,
}

impl Pipeline for Cube {
    type Vertex = (Vec4<f32>, Rgba<f32>);
    type VertexData = Rgba<f32>;
    type Primitives = TriangleList;
    type Pixel = u32;
    type Fragment = Rgba<f32>;

    fn vertex(&self, (pos, color): &Self::Vertex) -> ([f32; 4], Self::VertexData) { ((self.mvp * *pos).into_array(), *color) }
    fn fragment(&self, color: Self::VertexData) -> Self::Fragment { color }
    fn blend(&self, _: Self::Pixel, color: Self::Fragment) -> Self::Pixel { u32::from_le_bytes((color * 255.0).as_().into_array()) }
}

const R: Rgba<f32> = Rgba::new(1.0, 0.0, 0.0, 1.0);
const Y: Rgba<f32> = Rgba::new(1.0, 1.0, 0.0, 1.0);
const G: Rgba<f32> = Rgba::new(0.0, 1.0, 0.0, 1.0);
const B: Rgba<f32> = Rgba::new(0.0, 0.0, 1.0, 1.0);

const VERTICES: &[(Vec4<f32>, Rgba<f32>)] = &[
    (Vec4::new(-1.0, -1.0, -1.0, 1.0), R),
    (Vec4::new(-1.0, -1.0,  1.0, 1.0), Y),
    (Vec4::new(-1.0,  1.0, -1.0, 1.0), G),
    (Vec4::new(-1.0,  1.0,  1.0, 1.0), B),
    (Vec4::new( 1.0, -1.0, -1.0, 1.0), B),
    (Vec4::new( 1.0, -1.0,  1.0, 1.0), G),
    (Vec4::new( 1.0,  1.0, -1.0, 1.0), Y),
    (Vec4::new( 1.0,  1.0,  1.0, 1.0), R),
];

const INDICES: &[usize] = &[
    0, 3, 2, 0, 1, 3, // -x
    7, 4, 6, 5, 4, 7, // +x
    5, 0, 4, 1, 0, 5, // -y
    2, 7, 6, 2, 3, 7, // +y
    0, 6, 4, 0, 2, 6, // -z
    7, 1, 5, 3, 1, 7, // +z
];

struct Color {
    time: f32,
    color: Buffer2d<u32>,
    depth: Buffer2d<f32>,
}

impl Game for Color {
    const TITLE: &'static str = "Spinning Cube";
    type SaveData = ();

    fn init(console: &mut Console<Self>) -> Self {
        Self {
            time: 0.0,
            color: Buffer2d::fill(console.graphics.size.into_array(), 0),
            depth: Buffer2d::fill(console.graphics.size.into_array(), 1.0),
        }
    }

    fn tick(&mut self, dt: f32, console: &mut Console<Self>) {
        self.time += dt;

        // Resize the color and depth buffers if required. Otherwise, clear them ready for the next frame.
        if self.color.size() != console.graphics.size.into_array() {
            self.color = Buffer2d::fill(console.graphics.size.into_array(), 0);
            self.depth = Buffer2d::fill(console.graphics.size.into_array(), 1.0);
        } else {
            self.color.clear(0);
            self.depth.clear(1.0);
        }

        let mvp = Mat4::perspective_fov_lh_zo(1.3, console.graphics.size.x as f32, console.graphics.size.y as f32, 0.01, 100.0)
            * Mat4::translation_3d(Vec3::new(0.0, 0.0, 3.0))
            * Mat4::rotation_x((self.time as f32 * 0.4).sin() * 8.0)
            * Mat4::rotation_y((self.time as f32 * 0.8).cos() * 4.0)
            * Mat4::rotation_z((self.time as f32 * 1.6).sin() * 2.0)
            * Mat4::scaling_3d(Vec3::new(1.0, -1.0, 1.0));

        // Render the cube to the color buffer
        Cube { mvp }.render(
            IndexedVertices::new(INDICES, VERTICES),
            &mut self.color,
            &mut self.depth,
        );

        // Copy the color buffer to the console framebuffer
        console.graphics.framebuffer.copy_from_slice(&self.color.raw());
    }
}

fn main() {
    Color::run();
}
