# counter

This is a small program I wrote while learning some of the basics of programming an
AVR-based chip using Rust.

If you're interested in learning the basics, check out the
[AWESOME list](https://github.com/avr-rust/awesome-avr-rust) about where to start.

## Hardware

The setup for this project consists of
 - an ATmega328P (or ATmega328, I guess)

   I'm using an Arduino, but this should work without one. Just lookup the PIN mapping from the
   mcu to the Arduino pins and change the circuit accordingly.
 - a seven segment LED (or seven leds, if that's what you have on hand)
 - a pushbutton
 - one capacitor, I think mine is 1nF
 - some (9) resistors, 220Ω should do

   I also used a 1kΩ resistor combined with the capacitor to create a simple debouncer. I think
   it was mainly luck that mine worked out of the box. There are a number of different
   techniques for creating the same effect.
 - some wires/jumpers and
 - a breadboard

## Wiring

Here is an image showing the wiring. But feel free to use the schematic below, if you can't
make out the connections.
![Foto of the finished circuit by me][1]

![Schematics made in Fritzing by me 1][2]


[1]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/foto_of_the_result.JPG
[2]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/Counter_bb.svg

License: MIT
