use super::brightness::BrightnessLevel;
use evdev_rs::{enums::EventCode, Device, InputEvent, TimeVal, UInputDevice};
use subprocess::Exec;

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

    fn run_cmd(&self) {
        Exec::shell(self.get_cmd()).join().unwrap();
    }

    pub fn get_brightness(&self) -> String {
        self.brightness.to_string()
    }

    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }

    pub fn change_brightness(&mut self) {
        self.brightness.level >>= 1;
        self.build_cmd();
        self.run_cmd();
    }

    pub fn turn_off(&mut self, tp: &mut Device, udev: &UInputDevice) {
        Self::send_key_numlock_event(0, udev);
        tp.grab(evdev_rs::GrabMode::Ungrab).unwrap();
        self.brightness.level = BrightnessLevel::default().level;
        self.build_cmd();
        self.run_cmd();
    }

    pub fn turn_on(&mut self, tp: &mut Device, udev: &UInputDevice) {
        Self::send_key_numlock_event(1, udev);
        tp.grab(evdev_rs::GrabMode::Grab).unwrap();
        self.change_brightness();
        self.run_cmd();
    }

    pub fn send_event(&self, udev: &UInputDevice, ec: &EventCode) {
        let ie = InputEvent::new(&TimeVal::new(0, 0), ec, 1);
        let syn_report = InputEvent::new(
            &TimeVal::new(0, 0),
            &EventCode::EV_SYN(evdev_rs::enums::EV_SYN::SYN_REPORT),
            0,
        );
        udev.write_event(&ie).unwrap();
        udev.write_event(&syn_report).unwrap();
        Self::kill_event(udev, ec, &syn_report);
    }

    fn kill_event(udev: &UInputDevice, ec: &EventCode, syn_report: &InputEvent) {
        let ie = InputEvent::new(&TimeVal::new(0, 0), ec, 0);
        udev.write_event(&ie).unwrap();
        udev.write_event(&syn_report).unwrap();
    }

    fn send_key_numlock_event(n: i32, udev: &UInputDevice) {
        let key_numlock = InputEvent::new(
            &TimeVal::new(0, 0),
            &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_NUMLOCK),
            n,
        );
        let syn_report = InputEvent::new(
            &TimeVal::new(0, 0),
            &EventCode::EV_SYN(evdev_rs::enums::EV_SYN::SYN_REPORT),
            0,
        );
        udev.write_event(&key_numlock).unwrap();
        udev.write_event(&syn_report).unwrap();
    }
}

impl From<BrightnessLevel> for NumpadBrightnessController {
    fn from(value: BrightnessLevel) -> Self {
        Self { cmd: format!("i2ctransfer -f -y 0 w13@0x15 0x05 0x00 0x3d 0x03 0x06 0x00 0x07 0x00 0x0d 0x14 0x03 {} 0xad", value.get_lvl()), brightness: value }
    }
}
