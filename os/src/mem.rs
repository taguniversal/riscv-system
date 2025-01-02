
#[naked]
#[no_mangle]
#[link_section = ".init.rust"]
unsafe extern "C" fn _start_rust() -> ! {
    // Initialize stack pointer
    extern "C" {
        static __stack_top: u8;
    }
    core::arch::naked_asm!(
        "la sp, {stack_top}",
        stack_top = sym __stack_top
    );
}

pub unsafe fn clear_bss() {
    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;
    }
    
    let start = &mut __bss_start as *mut u8;
    let end = &mut __bss_end as *mut u8;
    let size = end.offset_from(start) as usize;
    start.write_bytes(0, size);
}