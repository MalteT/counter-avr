#![feature(llvm_asm, lang_items, abi_avr_interrupt)]
#![no_std]
#![no_main]
//! This is a small program I wrote while learning some of the basics of programming an
//! AVR-based chip using Rust.
//!
//! If you're interested in learning the basics, check out the
//! [AWESOME list](https://github.com/avr-rust/awesome-avr-rust) about where to start.
//!
//! # Hardware
//!
//! The setup for this project consists of
//!  - an **ATmega328P** (or ATmega328, I guess)
//!
//!    I'm using an Arduino, but this should work without one. Just lookup the PIN mapping from the
//!    mcu to the Arduino pins and change the circuit accordingly.
//!  - a **seven segment LED** (or seven leds, if that's what you have on hand)
//!  - a **pushbutton**
//!  - one **capacitor**, I think mine is 1nF
//!  - some (8) **resistors**, 220Ω should do
//!
//!    I also used a 1kΩ resistor combined with the capacitor to create a simple debouncer. I think
//!    it was mainly luck that mine worked out of the box. There are a number of different
//!    techniques for creating the same effect.
//!  - some **wires/jumpers** and
//!  - a **breadboard**
//!
//! # Wiring
//!
//! Here is an image showing the wiring. But feel free to use the schematic below, if you can't
//! make out the connections.
//! ![Foto of the finished circuit by me][1]
//!
//! Note that the seven segment LED part in Fritzing seems broken. The pins on the breadboard do
//! not match up with those shown in [this schematic][3].
//!
//! If you're having trouble mapping your segments correctly, have a look at the [`Segments`
//! struct][4]. It's a mapping from each pin of the port to the name of the segment.
//!
//! ![Schematics made in Fritzing by me 1][2]
//!
//! # Prerequisites
//!  - A recent version of the nightly Rust compiler. Anything including or greater than rustc 1.47.0-nightly (0820e54a8 2020-07-23) can be used.
//!  - The rust-src rustup component - `$ rustup component add rust-src`
//!  - AVR-GCC on the system for linking
//!
//! # Compiling
//!
//! To compile this program for the ATmega328p you can simply run `make build`.
//!
//! # Uploading
//!
//! I'm using [avrdude][5] to upload the compiled ELF-file, simply run `make`.
//! If your serial port differs from `/dev/ttyACM0` overwrite it like this:
//! ```console
//! $ make PORT=/dev/yourport
//! ```
//!
//! # The progam
//!
//! I tried to do some heavy commenting, to enable everyone to learn from this example. Now,
//! there are some really ugly parts and some of the unsafe code could, and might, be rewritten
//! in a safer way, but if you want to have a look, follow the white rabbit [🐇][6]
//!
//! # Contribution
//!
//! If your having trouble with the way I explained something or found a bug, feel free to contact
//! me or open an issue!
//!
//! [1]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/foto_of_the_result.JPG
//! [2]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/Counter_bb.svg
//! [3]: https://github.com/MalteT/counter-avr/blob/main/static/Counter_schema.svg
//! [4]: https://github.com/MalteT/counter-avr/blob/0ad680ff392639b7e11c5dfc12527a8bcf817132/src/main.rs#L63
//! [5]: https://www.nongnu.org/avrdude/
//! [6]: https://github.com/MalteT/counter-avr/blob/main/src/main.rs
use bitflags::bitflags;
use core::ptr::{read_volatile, write_volatile};
use ruduino::cores::current as avr_core;
use ruduino::{Pin, Register, RegisterBits};

use avr_core::{port, DDRB, DDRD, PCICR, PCMSK0, PORTB, PORTD, SREG};

static mut NUMBER: u8 = 0;
static mut TOGGLE_SWITCH: bool = false;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Let's initialize some stuff.
    // We are using every single bit of PORTD for output, not input.
    // DDRD actually stands for 'Data Direction Register D' afaik.
    DDRD::set(RegisterBits::new(0b1111_1111));
    // We are also using a single pin of PORTB for output, the rest I don't care about.
    DDRB::set(RegisterBits::new(0b0010_0000));
    // Now, about interrupts. I've gone trough the datasheet for the ATmega328p and found the
    // following. We have to enable interrupts in the Status Register (SREG). SREG::I is the bit
    // that we need to use here, I as in Interrupt.
    SREG::set(SREG::I);
    // Next, we want to execute an interrupt, iff a pin changes. the Pin Change Interrupt Control
    // Register (PCICR) seems like the way to go. PCIE0 enables the group of pins we're interested
    // in (Port B). If you want another pin to cause interrupts, you might want to change this.
    PCICR::set(PCICR::PCIE0);
    // At last, we can finally enable the single pin we want to cause interrupts. To control
    // which pins can cause an interrupt in the PCIE0 group, we have to set the Pin Change Mask 0
    // (PCMSK0) accordingly. The documentation hints, that Pin 0 of PORTB corresponds to the
    // Pin Change Interrupt 0, so let's enable that one.
    PCMSK0::set(PCMSK0::PCINT0);
    // We don't want our program to end.. ever. Only the spiritual figure of your choice might
    // know what happens when our main function exits. An assortment of guesses (and scientific
    // explanations) can be found [here](https://electronics.stackexchange.com/questions/30830/what-happens-when-an-embedded-program-finishes).
    loop {}
}

bitflags! {
    /// This struct keeps track of the enabled segments of the seven segment LED.
    ///
    /// Every one of these segments will be mapped to PORTD for display.
    ///
    /// I also added some combinations like [`Segments::NINE`], those contain the segments
    /// that need to be lit to display the corresponding character, in this case a `9`.
    pub struct Segments: u8 {
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const C = 0b0000_0100;
        const D = 0b0000_1000;
        const E = 0b0001_0000;
        const F = 0b0010_0000;
        const G = 0b0100_0000;
        const DOT = 0b1000_0000;
        const ZERO = !(Self::G.bits | Self::DOT.bits);
        const ONE = Self::B.bits | Self::C.bits;
        const TWO = !(Self::C.bits | Self::F.bits | Self::DOT.bits);
        const THREE = !(Self::E.bits | Self::F.bits | Self::DOT.bits);
        const FOUR = Self::B.bits | Self::C.bits | Self::F.bits | Self::G.bits;
        const FIVE = !(Self::B.bits | Self::E.bits | Self::DOT.bits);
        const SIX = !(Self::B.bits | Self::DOT.bits);
        const SEVEN = Self::A.bits | Self::B.bits | Self::C.bits;
        const EIGHT = !Self::DOT.bits;
        const NINE = !(Self::E.bits | Self::DOT.bits);
    }
}

/// Handle a PCINT interrupt.
///
/// This is conveniently named such that it will end up at the correct address for a PCINT0
/// interrupt. Here we're incrementing the number. Showing that number on the display.
/// Additionally PIN13 on the Arduino is toggled, which toggles the onboard led.
///
/// We're doing all kinds of evil stuff here, which is causing this function to be unsafe.
///  - Mutating a (read "two") static variable. This should probably be done using internal
///    mutation. I might redo this part.
///  - Reading/Writing in a volatile way. These function calls will not be reordered by the
///    compiler, which is rather important when changing I/O.
#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_3() {
    // Skip every second interrupt routine. This is a slightly nasty hack to prevent
    // counting twice for each button press, since changing the voltage at the pin from
    // low to high to low is twice the amount of change we want to have.
    TOGGLE_SWITCH = !TOGGLE_SWITCH;
    if TOGGLE_SWITCH {
        return;
    }
    // This code toggles the led on the Arduino Uno. If you have no such LED, you might
    // aswell remove this code or connect an external LED.
    let prev_value = read_volatile(PORTB::ADDRESS);
    write_volatile(PORTB::ADDRESS, prev_value ^ port::B5::MASK);
    // Now increment the current displayed number. This will result in a counter, that
    // counts from 0 to 9 and then resets.
    NUMBER = (NUMBER + 1) % 10;
    // Most importantly actually display the number!
    Segments::from_u8(NUMBER).display()
}

impl Segments {
    /// Display the selected segments.
    ///
    /// On the seven segment LED display, that is connected to I/O port B.
    pub fn display(&self) {
        // Actually invert all segments, since my seven segment LED is using a common anode.
        let inv = !(*self);
        PORTD::write(inv.bits);
    }
    /// Get the active signals for a `nr` between 0 and 9.
    ///
    /// # Note:
    ///
    /// Everything above 9 is interpreted as zero.
    pub fn from_u8(nr: u8) -> Self {
        match nr {
            0 => Self::ZERO,
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            4 => Self::FOUR,
            5 => Self::FIVE,
            6 => Self::SIX,
            7 => Self::SEVEN,
            8 => Self::EIGHT,
            9 => Self::NINE,
            _ => Self::ZERO,
        }
    }
}
