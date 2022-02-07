#![no_main]
#![no_std]

#[macro_use]
mod print;
use core::char::from_u32;

use r_efi::efi;
use r_efi::efi::protocols::simple_text_input::InputKey;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    log!("hello from uefi runtime driver!");
    
    loop {
        let mut x= InputKey::default();
        let mut s: usize = 0;
        let _ = unsafe {
            ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut s)
        };

        let r = unsafe {
            ((*(*st).con_in).read_key_stroke)((*st).con_in, &mut x)
        };

        if r.is_error() {
            log!("err");
            return r;
        }
        log!("{}", from_u32(x.unicode_char as u32).unwrap());
    }

    efi::Status::SUCCESS
}