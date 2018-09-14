#![no_main]
#![no_std]
#[macro_use]

extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l0;
extern crate cortex_m;

use stm32l0::stm32l0x3;
use rt::ExceptionFrame;
use cortex_m::asm;

entry!(main);

fn main() -> ! {
    let per = stm32l0x3::Peripherals::take().unwrap();
    let gpioa = &per.GPIOA;
    let rcc = &per.RCC;

    rcc.iopenr.write(|w| w.iopaen().set_bit());
    unsafe {
        // SVD made this unsafe without reason...
        gpioa.moder.write(|w| w.mode5().bits(1));
    }
    loop {
        gpioa.odr.write(|w| {
            w.od5().bit(gpioa.odr.read().od5().bit_is_clear())
        });
        for _i in 0..1000 {
            asm::nop()
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
