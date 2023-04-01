#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(not(test))]
use core::panic::PanicInfo;

use crate::interrupts::hlt_loop;

#[cfg(test)]
mod test;
mod vga_buffer;
mod interrupts;
mod gdt;
mod math;
mod log;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // create the interrupt descriptor table
    interrupts::init_idt();
    // double fault stuff
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    #[cfg(test)]
    test_main();

    info!("Started w/o error");
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    hlt_loop();
}