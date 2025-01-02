
use core::arch::asm;

#[repr(C)]
pub struct TrapFrame {
    pub regs: [usize; 32],  // x0-x31
    pub freq: usize,        // Frequency counter
    pub sepc: usize,        // Program counter
}

use crate::TIMER;

use crate::{SCHEDULER, UART};
use crate::sched::Task;
use crate::print_uart;
use core::fmt::Write;

#[no_mangle]
#[link_section = ".trap_vector"]


pub extern "C" fn trap_vector() {
    unsafe {
        asm!(
            // Save registers
            "csrrw t6, mscratch, t6",
            "sd x1, 1*8(t6)",
            "sd x2, 2*8(t6)",
            
            // Handle trap
            "csrr t0, mcause",
            "bgez t0, 1f",  // Forward reference to label 1
            
            // Exception handling
            "j 2f",         // Forward reference to label 2
            
            "1:",           // Numeric label for interrupt handling
            "li t0, 7",
            "bne t0, t1, 3f",
            "call timer_handler",
            "j 4f",
            
            "3:",          // Other interrupt
            "call generic_handler",
            
            "4:",          // Exit
            // Restore registers
            "ld x1, 1*8(t6)",
            "ld x2, 2*8(t6)",
            "csrrw t6, mscratch, t6",
            "mret",
            
            "2:",          // Exception label
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_handler() {
    unsafe {
        // Handle timer interrupt
        // Reset timer compare value
        // Schedule next interrupt

        if let Some(next_task) = SCHEDULER.schedule() {
            print_uart!("Switching to task {}", next_task.id);
            // Save current context and switch to next task
            switch_to_task(next_task);
        }

        TIMER.handle_interrupt();
    }
}

fn switch_to_task(task: &Task) {
    unsafe {
        core::arch::asm!(
            "mv sp, {0}",
            in(reg) task.stack_ptr,
        );
    }
}

#[no_mangle]
pub extern "C" fn generic_handler() {
    unsafe {
        // Handle other interrupts
    }
}