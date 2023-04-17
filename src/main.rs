mod numpad;
mod touchpad;
mod um433d;
use numpad::ctrl::NumpadBrightnessController;
use subprocess::Exec;
use touchpad::{button::CalcButton, dim::TouchpadDimenstions};

use evdev_rs::{enums::EventCode, Device, DeviceWrapper, ReadFlag, UInputDevice, UninitDevice};

use std::{env, fs::File};

use crate::touchpad::button::TriangleButton;

use log::{debug, info, log_enabled};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let app_launch = if args.len() > 1 { &args[1] } else { "" };

    let fd_tp = File::open("/dev/input/event9").unwrap();
    let mut d_tp = Device::new_from_file(fd_tp).unwrap(); // Opens in O_NONBLOCK

    let tp_dim = TouchpadDimenstions::new(
        d_tp.abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_X))
            .unwrap(),
        d_tp.abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_Y))
            .unwrap(),
    );

    debug!("Touchpad AbsXInfo: {:?}", tp_dim.get_max_x());
    debug!("Touchpad AbsYInfo: {:?}", tp_dim.get_max_y());

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

    for row in &numpad_keys.rows {
        for key in row {
            numpad_dev.enable_event_code(&key, None).unwrap();
        }
    }
    let udev = UInputDevice::create_from_device(&numpad_dev).unwrap();
    let mut calc_button = CalcButton::new(&tp_dim);
    let mut trg_button = TriangleButton::new();
    let mut nctrl = NumpadBrightnessController::new();
    let mut tap_pos: (i32, i32) = (0, 0);
    let mut pressed;

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
                    if log_enabled!(log::Level::Info) {
                        if pressed {
                            info!("Pressed at position x:{} y:{}", tap_pos.0, tap_pos.1);
                        }
                    }
                    if pressed {
                        if calc_button.pressed(tap_pos) {
                            if calc_button.active() {
                                nctrl.turn_off(&mut d_tp, &udev);
                                if log_enabled!(log::Level::Info) {
                                    info!("Numpad turned off");
                                }
                            } else {
                                nctrl.turn_on(&mut d_tp, &udev);
                                if log_enabled!(log::Level::Info) {
                                    info!("Numpad turned on");
                                }
                            }
                            calc_button.change_state();
                            continue;
                        }
                        if trg_button.pressed(tap_pos) && calc_button.active() {
                            nctrl.change_brightness();
                            if log_enabled!(log::Level::Info) {
                                info!("Change numpad's brightness {:?}", nctrl.get_brightness());
                            }
                            continue;
                        }
                        if trg_button.pressed(tap_pos)
                            && !calc_button.active()
                            && !app_launch.is_empty()
                        {
                            debug!("Run application: {}", app_launch);
                            Exec::shell(format!("nohup sudo -u $(logname) \"{app_launch}\""))
                                .join()
                                .unwrap();
                            continue;
                        }
                        if calc_button.active() {
                            let col: usize =
                                (tap_pos.0 / (tp_dim.get_max_x() / 5)).try_into().unwrap();
                            let row: usize =
                                (tap_pos.1 / (tp_dim.get_max_y() / 4)).try_into().unwrap();
                            if log_enabled!(log::Level::Info) {
                                info!("Choosed tab at row {} and column {}", row, col);
                            }
                            let event = numpad_keys.get_keycode(row, col);
                            nctrl.send_event(&udev, event);
                            if log_enabled!(log::Level::Info) {
                                info!("Sent event {}", event);
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
