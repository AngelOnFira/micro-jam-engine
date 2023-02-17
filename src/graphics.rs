use line_drawing::Bresenham;
use rusttype::{point, Font, Scale};
use vek::{Rect, Vec2};

pub struct Graphics<'tick> {
    pub size: Vec2<usize>,
    pub framebuffer: &'tick mut [u32],
    pub font: &'tick Font<'tick>,
}

impl<'tick> Graphics<'tick> {
    pub fn width(&self) -> f32 {
        self.size.x as f32
    }

    pub fn height(&self) -> f32 {
        self.size.y as f32
    }

    // TODO: Methods for drawing shapes, sprites, perhaps even triangles, as
    // well as getting access to the framebuffer
    pub fn clear(&mut self, color: u32) {
        for pixel in self.framebuffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Draw a rectangle. This takes a starting position and a size, and fills
    /// the rectangle with the given color.
    pub fn draw_rect(&mut self, rect: Rect<f32, f32>, color: u32, filled: bool) {
        match filled {
            true => {
                for y in rect.y as i32..(rect.y + rect.h) as i32 {
                    for x in rect.x as i32..(rect.x + rect.w) as i32 {
                        // If this pixel is outside the framebuffer, skip it
                        if x < 0 || y < 0 || x >= self.size.x as i32 || y >= self.size.y as i32 {
                            continue;
                        }
                        self.framebuffer[y as usize * self.size.x + x as usize] = color;
                    }
                }
            }
            false => {
                // Draw the four lines that make up the rectangle
                // Top
                self.draw_line(
                    Vec2::new(rect.x as i64, rect.y as i64),
                    Vec2::new((rect.x + rect.w) as i64, rect.y as i64),
                    color,
                );

                // Bottom
                self.draw_line(
                    Vec2::new(rect.x as i64, (rect.y + rect.h) as i64),
                    Vec2::new((rect.x + rect.w) as i64, (rect.y + rect.h) as i64),
                    color,
                );

                // Left
                self.draw_line(
                    Vec2::new(rect.x as i64, rect.y as i64),
                    Vec2::new(rect.x as i64, (rect.y + rect.h) as i64),
                    color,
                );

                // Right
                self.draw_line(
                    Vec2::new((rect.x + rect.w) as i64, rect.y as i64),
                    Vec2::new((rect.x + rect.w) as i64, (rect.y + rect.h) as i64),
                    color,
                );
            }
        }
    }

    /// Draw a line. This takes a starting position and an ending position, and
    /// draws a line between them with the given color.
    /// TODO: Change this to internal implementation of Bresenham's algorithm
    pub fn draw_line(&mut self, start: Vec2<i64>, end: Vec2<i64>, color: u32) {
        for (x, y) in Bresenham::new((start.x, start.y), (end.x, end.y)) {
            // If this pixel is outside the framebuffer, skip it
            if x < 0 || y < 0 || x >= self.size.x as i64 || y >= self.size.y as i64 {
                continue;
            }
            self.framebuffer[y as usize * self.size.x + x as usize] = color;
        }
    }

    /// Draw a circle. This takes a center position and a radius, and draws a
    /// circle with the given color.
    pub fn draw_circle(&mut self, center: Vec2<i64>, radius: i64, color: u32) {
        for y in -radius..radius {
            for x in -radius..radius {
                let pos = center + Vec2::new(x, y);
                if (pos - center).magnitude_squared() <= radius * radius {
                    // Make sure this pixel is inside the framebuffer
                    if pos.x < 0
                        || pos.y < 0
                        || pos.x >= self.size.x as i64
                        || pos.y >= self.size.y as i64
                    {
                        continue;
                    }

                    self.framebuffer[pos.y as usize * self.size.x + pos.x as usize] = color;
                }
            }
        }
    }

    /// Draw the given sprite at a position and with a frame index.
    ///
    /// If the frame index is greater than the number of frames, it will be looped animation will be looped.
    ///
    /// See [`sprite`] for information about loading sprites.
    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: Vec2<i64>, frame: usize) {
        let frame = frame % sprite.frames;
        let w = sprite.img.width() / sprite.frames as u32;

        for j in 0..sprite.img.height() {
            for i in 0..w {
                let p = sprite.img.get_pixel(i + w * frame as u32, j);
                if p.0[3] == 0 {
                    continue;
                }

                let pos = Vec2::new(i as i64, j as i64) - sprite.center + pos;
                // If this pixel is outside the framebuffer, skip it
                if pos.x < 0
                    || pos.y < 0
                    || pos.x >= self.size.x as i64
                    || pos.y >= self.size.y as i64
                {
                    continue;
                }
                self.framebuffer[pos.y as usize * self.size.x + pos.x as usize] =
                    u32::from_le_bytes([p.0[2], p.0[1], p.0[0], 0]);
            }
        }
    }

    pub fn draw_text(&mut self, text: &str, y: i64, x: i64, size: f32, color: u32) {
        let scale = Scale::uniform(size);
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(x as f32, y as f32 + v_metrics.ascent);
        let glyphs = self.font.layout(text, scale, offset);

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x + bb.min.x as u32;
                    let y = y + bb.min.y as u32;
                    if x as usize >= self.size.x || y as usize > self.size.y {
                        return;
                    }

                    let mut r = color >> 16 & 0xFF;
                    let mut g = color >> 8 & 0xFF;
                    let mut b = color & 0xFF;
                    r = ((r as f32) * v) as u32;
                    g = ((g as f32) * v) as u32;
                    b = ((b as f32) * v) as u32;
                    self.framebuffer[y as usize * self.size.x + x as usize] = r << 16 | g << 8 | b;
                });
            }
        }
    }
}

/// See [`sprite`].
pub struct Sprite {
    img: image::RgbaImage,
    center: Vec2<i64>,
    frames: usize,
}

impl Sprite {
    pub fn from_image(img: image::RgbaImage, center: Vec2<i64>, frames: usize) -> Self {
        Self {
            img,
            center,
            frames,
        }
    }
}

/// A macro that can be used to load a sprite into the game at compilation time.
///
/// The `center` parameter is of type [`Vec2<i64>`] and denotes the centre of the sprite, in pixels from the top-left
/// corner.
///
/// The `frames` parameter denotes the number of animation frames in the sprite. Frames can be added by extending the
/// image file horizontally: for example, a 16x16 sprite with 10 frames of animation would be 160x16 pixels in size.
///
/// # Example
///
/// ```ignore
/// lazy_static! {
///     static ref PACMAN: Sprite = sprite!("../pacman.png", center: Vec2::new(16, 16), frames: 6);
///     static ref TREAT: Sprite = sprite!("../treat.png");
///     static ref TILESET: Sprite = sprite!("../tileset.png", frames: 16);
/// }
/// ```
#[macro_export]
macro_rules! sprite {
    (
        $s:literal
        $(, center: $center:expr)?
        $(, frames: $frames:expr)?
        $(,)?
    ) => {
        {
            let s: &str = $s;
            let center = Vec2::zero();
            $(let center = $center;)?
            let frames = 1;
            $(let frames = ($frames).max(1);)?
            Sprite::from_image(
                $crate::_image::load_from_memory_with_format(
                    ::std::include_bytes!($s),
                    $crate::_image::ImageFormat::from_path(s).unwrap(),
                ).unwrap().into_rgba8(),
                center,
                frames,
            )
        }
    };
}
