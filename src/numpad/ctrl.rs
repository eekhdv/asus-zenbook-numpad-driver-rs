use super::brightness::BrightnessLevel;

#[derive(Debug)]
pub struct NumpadBrightnessController {
    cmd: String,
    brightness: BrightnessLevel,
}

impl NumpadBrightnessController {
    pub fn new() -> Self {
        Self::from(BrightnessLevel::default())
    }
    fn build_cmd(&mut self) {
        *self = Self::from(self.brightness);
    }

    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }

    pub fn change_brightness(&mut self) {
        self.brightness.level += 1;
        self.build_cmd();
    }
}

impl From<BrightnessLevel> for NumpadBrightnessController {
    fn from(value: BrightnessLevel) -> Self {
        Self { cmd: format!("i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 {} 0xad", value.get_lvl()), brightness: value }
    }
}
