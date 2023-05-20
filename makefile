all: dependencies build install

dependencies:
	os=$(shell cat /etc/os-release | grep -oP '^ID=\K(\w+)')
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
ifeq ($(os),fedora)
	sudo dnf install gcc
endif
ifeq ($(os),debian)
	sudo apt install gcc
endif

build:
	cd cpu-throttling-monitor && cargo build --release

install:
	sudo cp cpu-throttling-monitor/target/release/cpu-throttling-monitor /usr/local/bin
	chmod +x ./generate_service_file.sh
	./generate_service_file.sh
	sudo cp cpu-throttling-monitor.service /etc/systemd/system
	sudo systemctl enable cpu-throttling-monitor.service
	sudo systemctl start cpu-throttling-monitor.service
