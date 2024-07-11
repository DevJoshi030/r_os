#![no_std]
#![no_main]

use core::panic::PanicInfo;

use r_os::{
  hlt_loop, println, serial_print, serial_println,
  testing::{exit_qemu, QemuExitCode},
};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  serial_print!("basic_boot::basic_boot...\t");
  test_println();
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);

  hlt_loop();
}

fn test_println() {
  println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  r_os::testing::test_panic_handler(info)
}
