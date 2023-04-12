use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub(super) enum BrightnessLevelKind {
    OFF,
    LOWEST,
    LOW,
    HIGH,
    HIGHEST,
}

impl AddAssign<i32> for BrightnessLevelKind {
    fn add_assign(&mut self, _: i32) {
        *self = match *self {
            Self::OFF => Self::HIGHEST,
            Self::LOWEST => Self::OFF,
            Self::LOW => Self::LOWEST,
            Self::HIGH => Self::LOW,
            Self::HIGHEST => Self::HIGH,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct BrightnessLevel {
    pub(super) level: BrightnessLevelKind,
}

impl BrightnessLevel {
    pub(super) fn default() -> Self {
        Self {
            level: BrightnessLevelKind::OFF,
        }
    }
    pub(super) fn get_lvl(&self) -> &str {
        match self.level {
            BrightnessLevelKind::OFF => "0x0",
            BrightnessLevelKind::LOWEST => "0x2f",
            BrightnessLevelKind::LOW => "0x11",
            BrightnessLevelKind::HIGH => "0x31",
            BrightnessLevelKind::HIGHEST => "0x1",
        }
    }
}
