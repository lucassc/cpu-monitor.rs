# CPU Throttling Monitor

This repository contains a Rust service for monitoring CPU throttling on your system.

## Description

The CPU Throttling Monitor is a Rust application that continuously checks the CPU usage and sends a notification if the CPU is being throttled. It utilizes the sysinfo crate for retrieving system information and the chrono crate for working with dates and times.


## Installation

Clone the repository:

```bash
git clone https://github.com/lucassc/cpu-throttling-monitor.rs.git
```

**You can run the entire makefile**

```bash
make
```

**Or each step**

Install the required dependencies:

```bash
make dependencies
```

Build the application:

```bash
make build
```

Install the application and set up the systemd service:

```bash
make install
```

## Usage

Once the CPU Throttling Monitor is installed and the systemd service is enabled, it will run in the background and continuously monitor the CPU usage. If the CPU usage "exceeds" the threshold defined in the main.rs file, a notification will be sent.

To view the notifications, ensure that the notify-send command-line tool is installed on your system.

## Configuration

You can adjust the CPU usage threshold and notification settings by modifying the constants defined in the main.rs or notification.rs file:

    CPU_USAGE_THRESHOLD: The CPU usage threshold in MHz.
    NOTIFICATION_TITLE: The title of the notification.
    NOTIFICATION_BODY: The body/content of the notification.
    SEND_NOTIFICATION_SECONDS_INTERVAL: The interval in seconds between sending notifications.

## Contributing

Contributions to the CPU Throttling Monitor are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.