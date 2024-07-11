#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
  gdt::init();
  interrupts::init_idt();
  unsafe { interrupts::PICS.lock().initialize() };
  x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
  loop {
    x86_64::instructions::hlt();
  }
}

pub mod testing {

  use core::panic::PanicInfo;

  use crate::{hlt_loop, serial_print, serial_println};

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  #[repr(u32)]
  pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
  }

  pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
      let mut port = Port::new(0xf4);
      port.write(exit_code as u32);
    }
  }

  pub trait Testable {
    fn run(&self);
  }

  impl<T> Testable for T
  where
    T: Fn(),
  {
    fn run(&self) {
      serial_print!("{}...\t", core::any::type_name::<T>());
      self();
      serial_println!("[ok]");
    }
  }

  pub fn runner(tests: &[&dyn Testable], exit: bool) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
      test.run();
    }

    if exit {
      exit_qemu(QemuExitCode::Success);
    }
  }

  pub fn test_runner(tests: &[&dyn Testable]) {
    runner(tests, true);
  }

  pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
  }
}

#[cfg(test)]
mod tests {
  use core::panic::PanicInfo;

  use bootloader::entry_point;
  use bootloader::BootInfo;

  use crate::hlt_loop;
  use crate::interrupts::tests::tests as interrupts_tests;
  use crate::vga_buffer::tests::tests as vga_buffer_tests;

  entry_point!(test_kernel_main);

  /// Entry point for `cargo test`
  fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    use crate::init;

    init();

    run_tests();

    hlt_loop();
  }

  fn run_tests() {
    vga_buffer_tests(false);
    interrupts_tests(true);
  }

  #[panic_handler]
  fn panic(info: &PanicInfo) -> ! {
    crate::testing::test_panic_handler(info)
  }
}
