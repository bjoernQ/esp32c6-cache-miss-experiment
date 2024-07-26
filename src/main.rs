#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    enable_cache_check();

    let mut counter = 0;
    loop {
        println!("Hello world!");
        delay.delay(500.millis());

        if counter == 10 || counter == 15 {
            foo1();
        }

        if counter == 20 || counter == 25 {
            foo2();
        }

        if counter == 30 {
            foo3();
        }

        counter = (counter + 1) % 35;
    }
}

#[ram]
fn foo1() {
    unsafe {
        (0x4200_0000 as *mut u32).read_volatile();
    }
}

#[ram]
fn foo2() {
    unsafe {
        (0x4200_2000 as *mut u32).read_volatile();
    }
}

#[ram]
fn foo3() {
    for addr in (0x4200_0000u32..0x4201_0000u32).step_by(0x10) {
        unsafe {
            (addr as *mut u32).read_volatile();
        }
    }
}

#[ram]
fn enable_cache_check() {
    esp_hal::interrupt::enable_direct(
        esp_hal::peripherals::Interrupt::MSPI,
        esp_hal::interrupt::Priority::Priority3,
        esp_hal::interrupt::CpuInterrupt::Interrupt20,
    )
    .unwrap();
    let spi0 = unsafe { esp_hal::peripherals::SPI0::steal() };
    spi0.int_clr().write(|w| w.mst_st_end().clear_bit_by_one());
    spi0.int_ena().modify(|_, w| w.mst_st_end().set_bit());
}

#[ram]
#[no_mangle]
fn interrupt20(ctx: &esp_hal::trapframe::TrapFrame) {
    println!("0x{:x} - flash access", ctx.pc); // the PC isn't helpful at all

    let spi0 = unsafe { esp_hal::peripherals::SPI0::steal() };
    spi0.int_clr().write(|w| w.mst_st_end().clear_bit_by_one());
}
