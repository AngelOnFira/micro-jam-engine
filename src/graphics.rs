use line_drawing::Bresenham;
use vek::Vec2;

pub struct Graphics<'tick> {
    pub size: Vec2<usize>,
    pub framebuffer: &'tick mut [u32],
}

impl<'tick> Graphics<'tick> {
    // TODO: Methods for drawing shapes, sprites, perhaps even triangles, as
    // well as getting access to the framebuffer
    pub fn clear(&mut self, color: u32) {
        for pixel in self.framebuffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Draw a rectangle. This takes a starting position and a size, and fills
    /// the rectangle with the given color.
    pub fn draw_rect(&mut self, pos: Vec2<i64>, size: Vec2<i64>, color: u32) {
        for y in pos.y..pos.y + size.y {
            for x in pos.x..pos.x + size.x {
                // If this pixel is outside the framebuffer, skip it
                if x < 0 || y < 0 || x >= self.size.x as i64 || y >= self.size.y as i64 {
                    continue;
                }
                self.framebuffer[y as usize * self.size.x + x as usize] = color;
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
                    self.framebuffer[pos.y as usize * self.size.x + pos.x as usize] = color;
                }
            }
        }
    }
}
