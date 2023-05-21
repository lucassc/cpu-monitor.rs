use chrono::{prelude::*, DateTime, Local};
use std::process::Command;

use crate::configuration::Configuration;

pub struct Notification {
    last: DateTime<Local>,
    config: Configuration,
}

impl Notification {
    pub(crate) fn new(config: Configuration) -> Self {
        Notification {
            last: Local.with_ymd_and_hms(0, 1, 1, 1, 1, 1).unwrap(),
            config,
        }
    }

    pub fn send(&mut self) {
        if Local::now().signed_duration_since(self.last).num_seconds()
            < self.config.notification_seconds_interval as i64
        {
            return;
        }
        let notification_title = self.config.notification_title.as_str();
        let notification_body = self.config.notification_body.as_str();

        let mut cmd = Command::new("notify-send");
        cmd.arg(notification_title)
            .arg(notification_body)
            .output()
            .expect("failed to execute process");
        println!("Notification sent");

        self.last = Local::now();
    }
}
