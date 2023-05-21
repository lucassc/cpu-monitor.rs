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

You can adjust the CPU usage threshold and notification settings by modifying the default constants defined in the configuration.rs or setting following environment variables

```BASH
export CTM_THRESHOLD=400
export CTM_NOTIFICATION_TITLE="Warning"
export CTM_NOTIFICATION_BODY="CPU being throttled"
export CTM_NOTIFICATION_SECONDS_INTERVAL=2000
```

Another option is define those variables at [cpu-throttling-monitor.service.TEMPLATE](./cpu-throttling-monitor.service.TEMPLATE) at [Service].Environment

## Contributing

Contributions to the CPU Throttling Monitor are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.