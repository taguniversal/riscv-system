
use riscv::register::{mie};

// CLINT memory map for HiFive Pro
const CLINT_BASE: usize = 0x0200_0000;
const MTIME: *const u64 = (CLINT_BASE + 0xBFF8) as *const u64;
const MTIMECMP: *mut u64 = (CLINT_BASE + 0x4000) as *mut u64;

pub struct Timer {
    interval: u64,
}

impl Timer {
    pub const fn new() -> Self {
        Timer { interval: 10000000 } // Default interval (adjust based on clock)
    }

    pub fn init(&self) {
        unsafe {
            // Enable timer interrupts
            mie::set_mtimer();
            
            // Set first timeout
            self.set_next_timeout();
        }
    }

    pub fn set_next_timeout(&self) {
        unsafe {
            let current = MTIME.read_volatile();
            MTIMECMP.write_volatile(current + self.interval);
        }
    }

    pub fn handle_interrupt(&self) {
        // Set next timeout
        self.set_next_timeout();
        
        // Clear pending bit
        unsafe {
            
            riscv::register::mip::clear_stimer();
        }
    }
}
