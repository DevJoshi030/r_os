#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use r_os::{hlt_loop, memory::active_level_4_table, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use x86_64::VirtAddr;

  println!("Hello World{}", "!");

  r_os::init();

  // x86_64::instructions::interrupts::int3();

  // use x86_64::registers::control::Cr3;

  // let (level_4_page_table, _) = Cr3::read();

  // println!(
  //   "Level 4 page table at: {:#?}",
  //   level_4_page_table.start_address()
  // );

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

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

  use x86_64::structures::paging::PageTable;

  for (i, entry) in l4_table.iter().enumerate() {
    if !entry.is_unused() {
      println!("L4 Entry {}: {:?}", i, entry);

      let phys = entry.frame().unwrap().start_address();
      let virt = phys.as_u64() + boot_info.physical_memory_offset;
      let ptr = VirtAddr::new(virt).as_mut_ptr();
      let l3_table: &PageTable = unsafe { &*ptr };

      for (i, entry) in l3_table.iter().enumerate() {
        if !entry.is_unused() {
          println!("  L3 Entry {}: {:?}", i, entry);
        }
      }
    }
  }

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
