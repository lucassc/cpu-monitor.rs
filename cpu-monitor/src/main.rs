extern crate sysinfo;

use gdk::prelude::*;
use gio::prelude::*;
use glib::clone;
use glib::MainContext;
use std::sync::{Arc, Mutex};
use sysinfo::SystemExt;

fn main() {
    // Initialize the GTK and GLib libraries
    gtk::init().expect("Failed to initialize GTK.");
    gio::resources_register(include_bytes!("../resources/cpu_monitor.gresource"));

    // Create the system monitor and get the CPU information
    let system = sysinfo::System::new_all();
    system.refresh_cpu();

    // Create a shared state for CPU usage
    let cpu_usage = Arc::new(Mutex::new(0.0));

    // Create a timer to periodically update CPU usage
    let cpu_usage_clone = cpu_usage.clone();
    let update_timer = MainContext::default().spawn_local(duration::Duration::from_secs(1), move || {
        let mut cpu_usage = cpu_usage_clone.lock().unwrap();
        system.refresh_cpu();
        *cpu_usage = system.get_global_processor_info().get_cpu_usage();
        glib::Continue(true)
    }).detach();

    // Create the notification when CPU usage goes below 400MHz
    let cpu_usage_clone = cpu_usage.clone();
    let notification_timer = MainContext::default().spawn_local(duration::Duration::from_secs(1), move || {
        let cpu_usage = *cpu_usage_clone.lock().unwrap();
        if cpu_usage < 0.4 {
            show_notification();
        }
        glib::Continue(true)
    }).detach();

    // Run the GTK main loop
    gtk::main();
}


fn show_notification() {
    let app = gio::Application::new(Some("com.example.cpu_monitor"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Failed to create application");

    app.connect_activate(|_| {
        let notification = gio::Notification::new("CPU Usage Alert");
        notification.set_body(Some("CPU usage is below 400MHz."));

        let app_icon = gio::Icon::new_for_string("applications-system-symbolic");
        notification.set_icon(Some(&app_icon));

        let app = gio::Application::get_default().unwrap();
        app.send_notification(None, &notification);
    });

    app.run(&[]);
}