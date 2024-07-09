#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use r_os::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  r_os::init();

  // x86_64::instructions::interrupts::int3();

  // unsafe {
  //   *(0xdeadbeef as *mut u8) = 42;
  // }

  // #[allow(unconditional_recursion)]
  // fn stack_overflow() {
  //   stack_overflow();
  // }

  // stack_overflow();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");

  // #[allow(clippy::empty_loop)]
  // loop {
  //   use r_os::print;
  //   print!("-");
  // }

  hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  r_os::test_panic_handler(info)
}
