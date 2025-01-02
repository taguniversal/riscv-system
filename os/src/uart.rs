
use core::cell::UnsafeCell;
use core::fmt::{self, Write};


#[macro_export]
macro_rules! print_uart {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        unsafe {
            write!($crate::UART.assume_init_mut(), $($arg)*).unwrap();
        }
    });
}

pub struct Uart {
    base_address: UnsafeCell<*mut u32>
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Uart {
            base_address: UnsafeCell::new(base as *mut u32)
        }
    }

    pub fn write_reg(&mut self, offset: u32, value: u32) {
        unsafe {
            let base = *self.base_address.get();
            base.add(offset as usize).write_volatile(value);
        }
    }

    pub fn send(&mut self, byte: u8) {
        // Wait until TX FIFO is ready
        unsafe {
            let base = *self.base_address.get();
            while (base.add(0x14).read_volatile() & 0x80000000) != 0 {}
            self.write_reg(0, byte as u32);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

unsafe impl Sync for Uart {}