use std::ops::ShrAssign;

#[derive(Debug, Clone, Copy)]
pub(super) enum BrightnessLevelKind {
    OFF,
    LOWEST,
    LOW,
    HIGH,
    HIGHEST,
}

impl ShrAssign<u8> for BrightnessLevelKind {
    fn shr_assign(&mut self, _rhs: u8) {
        *self = match *self {
            Self::OFF => Self::HIGHEST,
            Self::LOWEST => Self::HIGHEST,
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

impl ToString for BrightnessLevel {
    fn to_string(&self) -> String {
        match self.level {
            BrightnessLevelKind::OFF => "OFF".to_string(),
            BrightnessLevelKind::LOW => "LOW".to_string(),
            BrightnessLevelKind::LOWEST => "LOWEST".to_string(),
            BrightnessLevelKind::HIGH => "HIGH".to_string(),
            BrightnessLevelKind::HIGHEST => "HIGHEST".to_string(),
        }
        
    }
}
