
PORT=/dev/ttyACM0

run: build
	avrdude -pm328p -carduino -b115200 -Uflash:w:target/avr-atmega328p/release/blink.elf:e -P${PORT}

build:
	cargo build -Z build-std=core --target avr-atmega328p.json --release

