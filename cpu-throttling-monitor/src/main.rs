extern crate sysinfo;

use sysinfo::{ProcessorExt, System, SystemExt};

mod notification;

const CPU_USAGE_THRESHOLD: u64 = 6000;
const SEND_NOTIFICATION_SECONDS_INTERVAL: u64 = 2000;

fn main() {
    cpu_usage_loop()
}

fn cpu_usage_loop() {
    let mut notification = notification::Notification::new();

    loop {
        cpu_usage(&mut notification);
        std::thread::sleep(std::time::Duration::from_secs(
            SEND_NOTIFICATION_SECONDS_INTERVAL,
        ));
    }
}

fn cpu_usage(notification: &mut notification::Notification) {
    let mut system = System::new_all();

    system.refresh_all();
    let processors = system.get_processors();

    for processor in processors {
        if processor.get_frequency() > CPU_USAGE_THRESHOLD {
            return;
        }
    }

    notification.send();
}

