use evdev_rs::AbsInfo;

#[derive(Debug)]
pub struct TouchpadDimenstions {
    x: AbsInfo,
    y: AbsInfo,
}

impl TouchpadDimenstions {
    pub fn new(x: AbsInfo, y: AbsInfo) -> Self {
        Self { x: (x), y: (y) }
    }
    pub fn get_max_x(&self) -> i32 {
        self.x.maximum
    }
    pub fn get_max_y(&self) -> i32 {
        self.y.maximum
    }
}
