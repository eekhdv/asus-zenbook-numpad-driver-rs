use std::fs::File;
mod um433d;

use evdev_rs::{
    enums::EventCode, Device, DeviceWrapper, InputEvent, ReadFlag, TimeVal, UInputDevice,
    UninitDevice,
};

#[derive(Debug)]
enum BrightnessLevelKind {
    OFF,
    LOWEST,
    LOW,
    HIGH,
    HIGHEST,
}

#[derive(Debug)]
struct BrightnessLevel {
    level: BrightnessLevelKind,
}

impl BrightnessLevel {
    fn get_lvl(&self) -> &str {
        match self.level {
            BrightnessLevelKind::OFF => "0x0",
            BrightnessLevelKind::LOWEST => "0x2f",
            BrightnessLevelKind::LOW => "0x11",
            BrightnessLevelKind::HIGH => "0x31",
            BrightnessLevelKind::HIGHEST => "0x1",
        }
    }
}

struct NumpadBrightnessController {
    cmd: String,
    brightness: BrightnessLevel,
}

impl NumpadBrightnessController {
    fn new() -> Self {
        Self { cmd: "i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 0x0 0xad".to_string(), brightness: {BrightnessLevel { level: BrightnessLevelKind::OFF }}}
    }
    fn with_level(b: BrightnessLevel) -> Self {
        Self { cmd: format!("i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 {} 0xad", b.get_lvl()), brightness: b }
    }
}

fn main() {
    let fd_tp = File::open("/dev/input/event9").unwrap();
    let mut d_tp = Device::new_from_file(fd_tp).unwrap(); // Opens in O_NONBLOCK
                                                          //
    let abs_x_info = d_tp
        .abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_X))
        .unwrap();
    let abs_y_info = d_tp
        .abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_Y))
        .unwrap();

    println!("Touchpad AbsXInfo:\n{:#?}", abs_x_info);
    println!("Touchpad AbsYInfo:\n{:#?}", abs_y_info);

    let numpad_dev = UninitDevice::new().unwrap();
    numpad_dev.set_name("Asus UM433D Numpad/Touchpad");
    numpad_dev
        .enable_event_code(
            &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_LEFTSHIFT),
            None,
        )
        .unwrap();
    numpad_dev
        .enable_event_code(
            &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_NUMLOCK),
            None,
        )
        .unwrap();
    let numpad_keys = um433d::KeyCodes::new();

    for row in numpad_keys.rows {
        for key in row {
            numpad_dev.enable_event_code(&key, None).unwrap();
        }
    }
    let udev = UInputDevice::create_from_device(&numpad_dev).unwrap();
    // udev.events = [
    //     InputEvent(EV_KEY.KEY_NUMLOCK, 1),
    //     InputEvent(EV_SYN.SYN_REPORT, 0)
    // ]
    udev.write_event(&InputEvent::new(
        &TimeVal::new(0, 0),
        &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_NUMLOCK),
        1,
    ))
    .unwrap();
    udev.write_event(&InputEvent::new(
        &TimeVal::new(0, 0),
        &EventCode::EV_SYN(evdev_rs::enums::EV_SYN::SYN_REPORT),
        0,
    ))
    .unwrap();
    d_tp.grab(evdev_rs::GrabMode::Grab).unwrap();
    // d_tp.grab(evdev_rs::GrabMode::Ungrab).unwrap();

    loop {
        if let Ok(ev) = d_tp.next_event(ReadFlag::NORMAL) {
            if ev
                .1
                .is_code(&EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_TOOL_FINGER))
            {
                println!("{:#?}", ev.1);
            }
            if ev.1.is_code(&EventCode::EV_ABS(
                evdev_rs::enums::EV_ABS::ABS_MT_POSITION_X,
            )) {
                println!("X POS: {:#?}", ev.1.value);
            } else if ev.1.is_code(&EventCode::EV_ABS(
                evdev_rs::enums::EV_ABS::ABS_MT_POSITION_Y,
            )) {
                println!("Y POS: {:#?}", ev.1.value);
            }
        }
    }
}
