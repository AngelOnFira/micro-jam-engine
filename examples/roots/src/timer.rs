pub struct Timer {
    pub start_time: f32,
    pub duration: f32,
    pub looping: bool,
}

impl Timer {
    pub fn new(curr_time: f32, duration: f32) -> Self {
        Self {
            start_time: curr_time,
            duration,
            looping: false,
        }
    }

    pub fn elapsed(&self, curr_time: f32) -> f32 {
        curr_time - self.start_time
    }

    pub fn is_complete(&self, curr_time: f32) -> bool {
        self.elapsed(curr_time) >= self.duration
    }
}
