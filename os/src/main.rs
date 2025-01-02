#![no_std]
#![no_main]
#![feature(naked_functions)]

mod uart;
mod mem;
mod trap;
mod timer;
mod sched;

use timer::Timer;

static TIMER: Timer = Timer::new();

use core::panic::PanicInfo;
use core::fmt::Write;
use uart::Uart;
use core::mem::MaybeUninit;
pub use sched::SCHEDULER;

const UART0: usize = 0x1001_0000;
static mut UART: MaybeUninit<Uart> = MaybeUninit::uninit();


pub extern "C" fn _start() -> ! {
    unsafe {
        // Clear BSS section
        mem::clear_bss();
        
        // Set up trap vector
        let trap_vector = trap::trap_vector as usize;
        riscv::register::mtvec::write(trap_vector, riscv::register::mtvec::TrapMode::Direct);
        
        // Initialize UART
        UART.write(Uart::new(UART0));
        let uart = UART.assume_init_mut();
        print_uart!("HiFive Pro booting...");

        // Initialize timer
        TIMER.init();
        print_uart!("Timer initialized");
        
        // Enable interrupts
        riscv::register::mstatus::set_mie();
        
        // Start main system
        main();
    }
}

fn task1() -> ! {
    loop {
        unsafe {
            print_uart!("Task 1 running");
            for _ in 0..1000000 { core::hint::spin_loop(); }
        }
    }
}

fn task2() -> ! {
    loop {
        unsafe {
            print_uart!("Task 2 running");
            for _ in 0..1000000 { core::hint::spin_loop(); }
        }
    }
}

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        SCHEDULER.create_task(task1);
        SCHEDULER.create_task(task2);
        
        print_uart!("Tasks created, entering main loop");
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        print_uart!("Panic: {}", info);
    }
    loop {}
}

