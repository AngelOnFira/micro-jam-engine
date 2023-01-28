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
    pub fn draw_rect(&mut self, pos: Vec2<usize>, size: Vec2<usize>, color: u32) {
        for y in pos.y..pos.y + size.y {
            for x in pos.x..pos.x + size.x {
                self.framebuffer[y * self.size.x + x] = color;
            }
        }
    }

    /// Draw a line. This takes a starting position and an ending position, and
    /// draws a line between them with the given color.
    pub fn draw_line(&mut self, start: Vec2<usize>, end: Vec2<usize>, color: u32) {
        let delta = end - start;
        let delta = delta.map(|x| x as f32);
        let delta = delta.map(|x| x.abs());
        let delta = delta.map(|x| x as usize);
        let delta = delta.map(|x| x.max(1));

        let step = (end - start).map(|x| x as f32 / delta.x as f32);

        for i in 0..delta.x {
            let pos = start.map(|x| x as f32) + step * i as f32;
            let pos = pos.map(|x| x as usize);
            self.framebuffer[pos.y * self.size.x + pos.x] = color;
        }
    }

    /// Draw a circle. This takes a center position and a radius, and draws a
    /// circle with the given color.
    pub fn draw_circle(&mut self, center: Vec2<usize>, radius: usize, color: u32) {
        for y in -1 * radius as isize..radius as isize {
            for x in -1 * radius as isize..radius as isize {
                let pos = center.map(|x| x as isize) + Vec2::new(x, y);
                let pos = pos.map(|x| x as usize);
                if (pos - center).map(|x| x as isize).magnitude_squared() <= radius as isize * radius as isize {
                    self.framebuffer[pos.y * self.size.x + pos.x] = color;
                }
            }
        }
    }
}
