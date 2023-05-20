extern crate sysinfo;

use chrono::{prelude::*, DateTime, Local};
use std::process::Command;
use sysinfo::{ProcessorExt, System, SystemExt};

const CPU_USAGE_THRESHOLD: u64 = 400;
const NOTIFICATION_TITLE: &str = "Warning";
const NOTIFICATION_BODY: &str = "CPU being throttled";
const SEND_NOTIFICATION_SECONDS_INTERVAL: i64 = 2000;

struct Notification {
    last: DateTime<Local>,
}

impl Notification {
    fn new() -> Self {
        Notification {
            last: Local.with_ymd_and_hms(0, 1, 1, 1, 1, 1).unwrap(),
        }
    }

    fn send(&mut self) {
        if Local::now().signed_duration_since(self.last).num_seconds()
            < SEND_NOTIFICATION_SECONDS_INTERVAL
        {
            return;
        }

        let mut cmd = Command::new("notify-send");
        cmd.arg(NOTIFICATION_TITLE)
            .arg(NOTIFICATION_BODY)
            .output()
            .expect("failed to execute process");
        println!("Notification sent");

        self.last = Local::now();
    }
}

fn main() {
    cpu_usage_loop()
}

fn cpu_usage_loop() {
    let mut notification = Notification::new();

    loop {
        cpu_usage(&mut notification);
        std::thread::sleep(std::time::Duration::from_secs(SEND_NOTIFICATION_SECONDS_INTERVAL as u64));
    }
}

fn cpu_usage(notification: &mut Notification) {
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
