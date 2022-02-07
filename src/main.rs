#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

use r_efi_alloc;

extern crate alloc;

use alloc::string::*;

#[macro_use]
mod print;
use core::char::from_u32;

use r_efi::efi::protocols::simple_text_input::InputKey;
use r_efi::*;

#[global_allocator]
static GLOBAL_ALLOCATOR: r_efi_alloc::global::Bridge = r_efi_alloc::global::Bridge::new();

#[alloc_error_handler]
fn rust_oom_handler(_layout: core::alloc::Layout) -> ! {
    panic!();
}

#[panic_handler]
fn rust_panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn efi_run(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    for i in 0..10 {
        let mut x = InputKey::default();
        let mut s: usize = 0;
        let _ = unsafe {
            ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut s)
        };

        let r = unsafe { ((*(*st).con_in).read_key_stroke)((*st).con_in, &mut x) };

        if r.is_error() {
            log!("err");
            return r;
        }

        let s = String::from("aiueo");
        log!("{} {}", s, from_u32(x.unicode_char as u32).unwrap());
    }

    efi::Status::SUCCESS
}

#[no_mangle]
pub extern "C" fn efi_main(h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    unsafe {
        let mut allocator = r_efi_alloc::alloc::Allocator::from_system_table(st, efi::LOADER_DATA);
        let _attachment = GLOBAL_ALLOCATOR.attach(&mut allocator);

        log!("hello from uefi runtime driver!");
        efi_run(h, st)
    }
}
