use evdev_rs::enums::EventCode;

#[derive(Debug)]
pub struct KeyCodes {
    pub rows: Vec<Vec<EventCode>>,
}

impl KeyCodes {
    pub fn new() -> Self {
        let mut numpad_keys = Vec::new();
        numpad_keys.push(vec![
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP7),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP8),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP9),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPSLASH),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_BACKSPACE),
        ]);
        numpad_keys.push(vec![
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP4),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP5),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP6),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPASTERISK),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_BACKSPACE),
        ]);
        numpad_keys.push(vec![
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP1),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP2),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP3),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPMINUS),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_5),
        ]);
        numpad_keys.push(vec![
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KP0),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPDOT),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPENTER),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPPLUS),
            EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_KPEQUAL),
        ]);
        KeyCodes { rows: numpad_keys }
    }
}
