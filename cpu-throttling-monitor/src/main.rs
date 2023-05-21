use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

pub mod configuration;
mod notification;

fn main() {
    cpu_usage_loop()
}

fn cpu_usage_loop() {
    let config = configuration::Configuration::default();
    let notification_seconds_interval = config.notification_seconds_interval;
    let threshold = config.threshold;

    let mut notification = notification::Notification::new(config);

    loop {
        cpu_usage(&mut notification, threshold);
        std::thread::sleep(std::time::Duration::from_secs(
            notification_seconds_interval,
        ));
    }
}

fn cpu_usage(notification: &mut notification::Notification, threshold: u64) {
    let mut system = System::new_with_specifics(RefreshKind::new().with_cpu());

    system.refresh_cpu();
    let processors = system.get_processors();

    for processor in processors {
        if processor.get_frequency() > threshold {
            return;
        }
    }

    notification.send();
}
