# counter

This is a small program I wrote while learning some of the basics of programming an
AVR-based chip using Rust.

If you're interested in learning the basics, check out the
[AWESOME list](https://github.com/avr-rust/awesome-avr-rust) about where to start.

## Hardware

The setup for this project consists of
 - an **ATmega328P** (or ATmega328, I guess)

   I'm using an Arduino, but this should work without one. Just lookup the PIN mapping from the
   mcu to the Arduino pins and change the circuit accordingly.
 - a **seven segment LED** (or seven leds, if that's what you have on hand)
 - a **pushbutton**
 - one **capacitor**, I think mine is 1nF
 - some (8) **resistors**, 220Ω should do

   I also used a 1kΩ resistor combined with the capacitor to create a simple debouncer. I think
   it was mainly luck that mine worked out of the box. There are a number of different
   techniques for creating the same effect.
 - some **wires/jumpers** and
 - a **breadboard**

## Wiring

Here is an image showing the wiring. But feel free to use the schematic below, if you can't
make out the connections.
![Foto of the finished circuit by me][1]

Note that the seven segment LED part in Fritzing seems broken. The pins on the breadboard do
not match up with those shown in [this schematic][3].

If you're having trouble mapping your segments correctly, have a look at the [`Segments`
struct][4]. It's a mapping from each pin of the port to the name of the segment.

![Schematics made in Fritzing by me 1][2]

## Prerequisites
 - A recent version of the nightly Rust compiler. Anything including or greater than rustc 1.47.0-nightly (0820e54a8 2020-07-23) can be used.
 - The rust-src rustup component - `$ rustup component add rust-src`
 - AVR-GCC on the system for linking

## Compiling

To compile this program for the ATmega328p you can simply run `make build`.

## Uploading

I'm using [avrdude][5] to upload the compiled ELF-file, simply run `make`.
If your serial port differs from `/dev/ttyACM0` overwrite it like this:
```console
$ make PORT=/dev/yourport
```

[1]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/foto_of_the_result.JPG
[2]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/Counter_bb.svg
[3]: https://github.com/MalteT/counter-avr/blob/main/static/Counter_schema.svg
[4]: https://github.com/MalteT/counter-avr/blob/0ad680ff392639b7e11c5dfc12527a8bcf817132/src/main.rs#L63
[5]: https://www.nongnu.org/avrdude/

License: MIT
