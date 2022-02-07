#![no_main]
#![no_std]

#[macro_use]
mod print;
use r_efi::efi;
use core::fmt::Write;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    log!("hello!");
    
    // Wait for key input, by waiting on the `wait_for_key` event hook.
    let r = unsafe {
        let mut x: usize = 0;
        ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut x)
    };
    if r.is_error() {
        return r;
    }

    efi::Status::SUCCESS
}