use chrono::{prelude::*, DateTime, Local};
use std::process::Command;

const NOTIFICATION_TITLE: &str = "Warning";
const NOTIFICATION_BODY: &str = "CPU being throttled";
const SEND_NOTIFICATION_SECONDS_INTERVAL: i64 = 2000;

mod config;

pub struct Notification {
    last: DateTime<Local>,
    config: config::MyConfig,
}

impl Notification {
    pub(crate) fn new() -> Self {
        Notification {
            last: Local.with_ymd_and_hms(0, 1, 1, 1, 1, 1).unwrap(),
            my_config: MyConfig::default(),
        }
    }

    pub fn send(&mut self) {
        
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
