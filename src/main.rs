use std::fs::File;

use evdev_rs::{
    enums::EventCode, Device, DeviceWrapper, InputEvent, ReadFlag, TimeVal, UInputDevice,
    UninitDevice,
};

fn main() {
    todo!("get event from /proc/bus/input/devices");
    let fd_tp = get_touchpad_event_from_proc();
    let fd_tp = File::open("/dev/input/event9").unwrap();
    let mut d_tp = Device::new_from_file(fd_tp).unwrap(); // Opens in O_NONBLOCK
    let abs_x_info = d_tp
        .abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_X))
        .unwrap();
    let abs_y_info = d_tp
        .abs_info(&EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_Y))
        .unwrap();
    println!("Touchpad AbsXInfo:\n{:#?}", abs_x_info);
    println!("Touchpad AbsYInfo:\n{:#?}", abs_y_info);

    let fd_kb = File::open("/dev/input/event3").unwrap();
    let d_kb = Device::new_from_file(fd_kb).unwrap();

    let numpad_dev = UninitDevice::new().unwrap();
    numpad_dev.set_name("Asus UM433D Numpad/Touchpad");
    numpad_dev
        .enable_event_code(
            &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_LEFTSHIFT), None,
        )
        .unwrap();
    numpad_dev
        .enable_event_code(
            &EventCode::EV_KEY(evdev_rs::enums::EV_KEY::KEY_NUMLOCK),
            None,
        )
        .unwrap();
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

    for row in numpad_keys {
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
