// Create a virtual keyboard, just while this is running.
// Generally this requires root.

use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_A);

    let mut device = VirtualDeviceBuilder::new()?
        .name("Fake Keyboard")
        .with_keys(&keys)?
        .build()
        .unwrap();

    for path in device.enumerate_dev_nodes_blocking()? {
        let path = path?;
        println!("Available as {}", path.display());
    }

    let type_ = EventType::KEY;
    // Note this will ACTUALLY PRESS the button on your computer.
    // Hopefully you don't have BTN_DPAD_UP bound to anything important.
    let code = Key::KEY_A.code();

    println!("Waiting for Ctrl-C...");
    loop {
        let down_event = InputEvent::new(type_, code, 1);
        device.emit(&[down_event]).unwrap();
        println!("Pressed.");
        sleep(Duration::from_secs(2));

        let up_event = InputEvent::new(type_, code, 0);
        device.emit(&[up_event]).unwrap();
        println!("Released.");
        sleep(Duration::from_secs(2));
    }
}
