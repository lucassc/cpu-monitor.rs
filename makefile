all: install

install:
	os=$(shell cat /etc/os-release | grep -oP '^ID=\K(\w+)')
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
ifeq ($(os),fedora)
	sudo dnf install gtk3-devel glib2-devel
endif
