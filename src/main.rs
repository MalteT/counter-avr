#![feature(llvm_asm, lang_items, abi_avr_interrupt)]
#![no_std]
#![no_main]

extern crate ruduino;
extern crate bitflags;

use core::ptr::{read_volatile, write_volatile};
use bitflags::bitflags;
use ruduino::cores::atmega328 as avr_core;
use ruduino::{Pin, RegisterBits, Register};

use avr_core::{DDRD, PORTD, port, PCMSK0, PCICR, DDRB, SREG, PORTB};

static mut NUMBER: u8 = 0;

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

/// A small busy loop.
fn small_delay() {
    for _ in 0..1000000 {
        unsafe { llvm_asm!("" :::: "volatile")}
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
