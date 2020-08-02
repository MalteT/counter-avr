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
//!  - an ATmega328P (or ATmega328, I guess)
//!
//!    I'm using an Arduino, but this should work without one. Just lookup the PIN mapping from the
//!    mcu to the Arduino pins and change the circuit accordingly.
//!  - a seven segment LED (or seven leds, if that's what you have on hand)
//!  - a pushbutton
//!  - one capacitor, I think mine is 1nF
//!  - some (9) resistors, 220Ω should do
//!
//!    I also used a 1kΩ resistor combined with the capacitor to create a simple debouncer. I think
//!    it was mainly luck that mine worked out of the box. There are a number of different
//!    techniques for creating the same effect.
//!  - some wires/jumpers and
//!  - a breadboard
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
//!
//! [1]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/foto_of_the_result.JPG
//! [2]: https://raw.githubusercontent.com/MalteT/counter-avr/main/static/Counter_bb.svg
//! [3]: https://github.com/MalteT/counter-avr/blob/main/static/Counter_schema.svg
//! [4]: https://github.com/MalteT/counter-avr/blob/0ad680ff392639b7e11c5dfc12527a8bcf817132/src/main.rs#L63

use core::ptr::{read_volatile, write_volatile};
use bitflags::bitflags;
use ruduino::cores::current as avr_core;
use ruduino::{Pin, RegisterBits, Register};

use avr_core::{DDRD, PORTD, port, PCMSK0, PCICR, DDRB, SREG, PORTB};

static mut NUMBER: u8 = 0;
static mut TOGGLE_SWITCH: bool = false;

#[no_mangle]
pub extern fn main() {
    DDRD::set(RegisterBits::new(0b1111_1111));
    DDRB::set(RegisterBits::new(0b0010_0000));
    PCMSK0::set(PCMSK0::PCINT0);
    PCICR::set(PCICR::PCIE0);
    SREG::set(SREG::I);

    loop {
    }
}

bitflags! {
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

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_3() {
    TOGGLE_SWITCH = !TOGGLE_SWITCH;
    if TOGGLE_SWITCH {
        return
    }
    let prev_value = read_volatile(PORTB::ADDRESS);
    write_volatile(PORTB::ADDRESS, prev_value ^ port::B5::MASK);
    NUMBER = (NUMBER + 1) % 10;
    Segments::from_u8(NUMBER).display()
}

impl Segments {
    pub fn display(&self) {
            let inv = !(*self);
            PORTD::write(inv.bits);
    }

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
