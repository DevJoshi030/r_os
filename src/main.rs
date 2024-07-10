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

  use x86_64::registers::control::Cr3;

  let (level_4_page_table, _) = Cr3::read();

  println!(
    "Level 4 page table at: {:#?}",
    level_4_page_table.start_address()
  );

  // unsafe {
  //   // *(0xdeadbeef as *mut u8) = 42;
  //   let mut _x = *(0x204f24 as *mut u8);
  //   println!("read worked");
  //   *(0x204f24 as *mut u8) = 42;
  //   println!("write worked");
  // }

  // #[allow(unconditional_recursion)]
  // fn stack_overflow() {
  //   stack_overflow();
  // }

  // 0x204f24

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
