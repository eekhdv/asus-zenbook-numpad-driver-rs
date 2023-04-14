use super::dim::TouchpadDimenstions;

#[derive(Debug, Default)]
pub struct CalcButton {
    active: bool,
    x_pos: i32,
    y_pos: i32,
}

impl CalcButton {
    pub fn new(tp: &TouchpadDimenstions) -> Self {
        Self {
            active: false,
            x_pos: tp.get_max_x() - 250,
            y_pos: 250,
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn pressed(&mut self, tap_pos: (i32, i32)) -> bool {
        tap_pos.0 >= self.x_pos && tap_pos.1 <= self.y_pos
    }
    pub fn change_state(&mut self) {
        self.active = !self.active();
    }
}

pub struct TriangleButton {
    x_pos: i32,
    y_pos: i32,
}

impl TriangleButton {
    pub fn new() -> Self {
        Self {
            x_pos: 250,
            y_pos: 200,
        }
    }
    pub fn pressed(&mut self, tap_pos: (i32, i32)) -> bool {
        tap_pos.0 <= self.x_pos && tap_pos.1 <= self.y_pos
    }
}
