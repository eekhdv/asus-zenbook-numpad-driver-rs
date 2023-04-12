use std::{fs::File, ops::AddAssign};
mod um433d;

use evdev_rs::{
    enums::EventCode, AbsInfo, Device, DeviceWrapper, InputEvent, ReadFlag, TimeVal, UInputDevice,
    UninitDevice,
};

use subprocess::Exec;

#[derive(Debug)]
enum BrightnessLevelKind {
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

#[derive(Debug)]
struct BrightnessLevel {
    level: BrightnessLevelKind,
}

impl BrightnessLevel {
    fn new(level: BrightnessLevelKind) -> Self {
        Self { level: (level) }
    }
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

#[derive(Debug)]
struct NumpadBrightnessController {
    cmd: String,
    brightness: BrightnessLevel,
}

impl NumpadBrightnessController {
    fn build_cmd(&mut self) {
        self.cmd = format!("i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 {} 0xad", self.brightness.get_lvl());
    }

    fn get_cmd(&self) -> &str {
        &self.cmd
    }

    fn change_brightness(&mut self) {
        self.brightness.level += 1;
        self.build_cmd();
    }
}

impl From<BrightnessLevel> for NumpadBrightnessController {
    fn from(value: BrightnessLevel) -> Self {
        Self { cmd: format!("i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 {} 0xad", value.get_lvl()), brightness: value }
    }
    
}

#[derive(Debug)]
struct TouchpadDimenstions {
    x: AbsInfo,
    y: AbsInfo,
}

impl TouchpadDimenstions {
    fn new(x: AbsInfo, y: AbsInfo) -> Self {
        Self { x: (x), y: (y) }
    }
    fn get_max_x(&self) -> i32 {
        self.x.maximum
    }
    fn get_max_y(&self) -> i32 {
        self.y.maximum
    }
}

#[derive(Debug, Default)]
struct CalcButton {
    pub pressed: bool,
    x_pos: i32,
    y_pos: i32,
}

impl CalcButton {
    fn new(tp: &TouchpadDimenstions) -> Self {
        Self {
            pressed: false,
            x_pos: tp.get_max_x() - 250,
            y_pos: 250,
        }
    }

    fn is_pressed(&mut self, tap_pos: (i32, i32)) -> bool {
        self.pressed = tap_pos.0 >= self.x_pos && tap_pos.1 <= self.y_pos;
        self.pressed
    }
}

fn main() {
    let fd_tp = File::open("/dev/input/event9").unwrap();
    let mut d_tp = Device::new_from_file(fd_tp).unwrap(); // Opens in O_NONBLOCK

    let tp_dim = TouchpadDimenstions::new(
        d_tp.abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_X))
            .unwrap(),
        d_tp.abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_Y))
            .unwrap(),
    );

    println!("Touchpad AbsXInfo:\n{:#?}", tp_dim.get_max_x());
    println!("Touchpad AbsYInfo:\n{:#?}", tp_dim.get_max_y());

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
    let mut calc_button = CalcButton::new(&tp_dim);
    let b_lvl = BrightnessLevel::new(BrightnessLevelKind::HIGHEST);
    let mut nctrl = NumpadBrightnessController::from(b_lvl);
    let mut tap_pos: (i32, i32) = (0, 0);
    let mut pressed;

    Exec::shell(
        NumpadBrightnessController::from(BrightnessLevel {
            level: BrightnessLevelKind::OFF,
        })
        .get_cmd(),
    )
    .join()
    .unwrap();
    loop {
        if let Ok(ev) = d_tp.next_event(ReadFlag::NORMAL) {
            match ev.1.event_code {
                EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_MT_POSITION_X) => {
                    tap_pos.0 = ev.1.value;
                }
                EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_MT_POSITION_Y) => {
                    tap_pos.1 = ev.1.value;
                }
                EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_TOOL_FINGER) => {
                    pressed = ev.1.value == 1;
                    if pressed && calc_button.is_pressed(tap_pos) {
                        nctrl.change_brightness();
                        println!("{:?}", nctrl.brightness);
                        println!("{:?}", nctrl.get_cmd());
                        Exec::shell(&nctrl.get_cmd()).join().unwrap();
                    }
                }
                _ => (),
            }

            // if ev
            //     .1
            //     .is_code(&EventCode::EV_KEY(evdev_rs::enums::EV_KEY::BTN_TOOL_FINGER))
            // {
            //     println!("{:#?}", ev.1);
            // }
            // if ev.1.is_code(&EventCode::EV_ABS(
            //     evdev_rs::enums::EV_ABS::ABS_MT_POSITION_X,
            // )) {
            //     println!("X POS: {:#?}", ev.1.value);
            // }
            // if ev.1.is_code(&EventCode::EV_ABS(
            //     evdev_rs::enums::EV_ABS::ABS_MT_POSITION_Y,
            // )) {
            //     println!("Y POS: {:#?}", ev.1.value);
            // }
        }
    }
}
