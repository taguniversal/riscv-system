pub struct FrameBuffer {
    data: [u32; 1024],  // 32x32 pixels = 1024
    width: usize,
    height: usize,
 }
 
 // Color constants
pub const RED: u32    = 0xFF0000;
pub const GREEN: u32  = 0x00FF00;
pub const BLUE: u32   = 0x0000FF;
pub const WHITE: u32  = 0xFFFFFF;
pub const BLACK: u32  = 0x000000;

 pub struct Hub75 {
    r1_pin: u32,
    g1_pin: u32,
    b1_pin: u32,
    r2_pin: u32,
    g2_pin: u32,
    b2_pin: u32,
    a_pin: u32,
    b_pin: u32,
    c_pin: u32,
    lat_pin: u32,
    oe_pin: u32,
    clk_pin: u32,
    front_buffer: FrameBuffer,
    back_buffer: FrameBuffer,
    active_buffer: bool,  // false = front, true = back
}


 impl FrameBuffer {
    pub const fn new() -> Self {
        FrameBuffer {
            data: [0; 1024],
            width: 32,
            height: 32,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(BLACK);
    }
 
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        // Bresenham's line algorithm
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
 
        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                self.set_pixel(x as usize, y as usize, color);
            }
            if x == x1 && y == y1 { break; }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
 
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for i in 0..width {
            let x_pos = x + i;
            if x_pos >= 0 && x_pos < self.width as i32 {
                if y >= 0 && y < self.height as i32 {
                    self.set_pixel(x_pos as usize, y as usize, color);
                }
                if y + height - 1 >= 0 && y + height - 1 < self.height as i32 {
                    self.set_pixel(x_pos as usize, (y + height - 1) as usize, color);
                }
            }
        }
 
        for i in 0..height {
            let y_pos = y + i;
            if y_pos >= 0 && y_pos < self.height as i32 {
                if x >= 0 && x < self.width as i32 {
                    self.set_pixel(x as usize, y_pos as usize, color);
                }
                if x + width - 1 >= 0 && x + width - 1 < self.width as i32 {
                    self.set_pixel((x + width - 1) as usize, y_pos as usize, color);
                }
            }
        }
    }
 
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for i in 0..width {
            for j in 0..height {
                let x_pos = x + i;
                let y_pos = y + j;
                if x_pos >= 0 && x_pos < self.width as i32 && 
                   y_pos >= 0 && y_pos < self.height as i32 {
                    self.set_pixel(x_pos as usize, y_pos as usize, color);
                }
            }
        }
    }
 
    pub fn draw_circle(&mut self, x_center: i32, y_center: i32, radius: i32, color: u32) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;
 
        while x >= y {
            self.set_pixel_safe(x_center + x, y_center + y, color);
            self.set_pixel_safe(x_center + y, y_center + x, color);
            self.set_pixel_safe(x_center - y, y_center + x, color);
            self.set_pixel_safe(x_center - x, y_center + y, color);
            self.set_pixel_safe(x_center - x, y_center - y, color);
            self.set_pixel_safe(x_center - y, y_center - x, color);
            self.set_pixel_safe(x_center + y, y_center - x, color);
            self.set_pixel_safe(x_center + x, y_center - y, color);
 
            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
 
    fn set_pixel_safe(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.set_pixel(x as usize, y as usize, color);
        }
    }
 }

impl Hub75 {
    pub fn init(&mut self) {
        // Configure GPIO pins as outputs
    }

    pub fn set_row(&mut self, row: u8) {
        // Set address lines A, B, C for row pair
        let row_pair = row >> 1;  // Divide by 2 since we drive two rows at once
        
        self.gpio_set(self.a_pin, (row_pair & 0b001) != 0);
        self.gpio_set(self.b_pin, (row_pair & 0b010) != 0);
        self.gpio_set(self.c_pin, (row_pair & 0b100) != 0);
    }

    pub fn set_row(&mut self, row: u8) {
        // Set address lines A, B, C for row pair
        let row_pair = row >> 1;  // Divide by 2 since we drive two rows at once
        
        self.gpio_set(self.a_pin, (row_pair & 0b001) != 0);
        self.gpio_set(self.b_pin, (row_pair & 0b010) != 0);
        self.gpio_set(self.c_pin, (row_pair & 0b100) != 0);
    }
 

    pub fn write_pixels(&mut self, data: &[u32]) {
        // Shift out RGB data
        for pixel in data {
            self.clock_out(*pixel);
        }
        self.latch();
    }

    pub fn display_row(&mut self, row: u8, data: &[u32]) {
        // Disable output while shifting data
        self.enable_output(false);
        
        // Set row address
        self.set_row(row);
        
        // Shift out RGB data
        self.write_pixels(data);
        
        // Latch the data
        self.latch();
        
        // Enable output
        self.enable_output(true);
    }

    fn clock_out(&mut self, rgb: u32) {
        // Clock out RGB bits
    }

    fn latch(&mut self) {
        self.gpio_set(self.lat_pin, true);
        // Small delay (could use a timer here)
        for _ in 0..10 { core::hint::spin_loop(); }
        self.gpio_set(self.lat_pin, false);
    }

    fn gpio_set(&mut self, pin: u32, value: bool) {
        unsafe {
            // HiFive Pro GPIO registers
            const GPIO_BASE: u32 = 0x10010000;
            const GPIO_OUT: u32 = 0x00;
            const GPIO_DIR: u32 = 0x04;
            
            // Set direction to output
            let dir_reg = (GPIO_BASE + GPIO_DIR) as *mut u32;
            *dir_reg |= 1 << pin;
            
            // Set output value
            let out_reg = (GPIO_BASE + GPIO_OUT) as *mut u32;
            if value {
                *out_reg |= 1 << pin;
            } else {
                *out_reg &= !(1 << pin);
            }
        }
    }

    fn clock_out(&mut self, rgb: u32) {
        // Split into R1G1B1R2G2B2
        let r1 = (rgb >> 20) & 0x3F;
        let g1 = (rgb >> 14) & 0x3F;
        let b1 = (rgb >> 8) & 0x3F;
        let r2 = (rgb >> 6) & 0x3F;
        let g2 = (rgb >> 2) & 0x3F;
        let b2 = rgb & 0x3F;
 
        for bit in 0..6 {
            self.gpio_set(self.r1_pin, (r1 >> bit) & 1 != 0);
            self.gpio_set(self.g1_pin, (g1 >> bit) & 1 != 0);
            self.gpio_set(self.b1_pin, (b1 >> bit) & 1 != 0);
            self.gpio_set(self.r2_pin, (r2 >> bit) & 1 != 0);
            self.gpio_set(self.g2_pin, (g2 >> bit) & 1 != 0);
            self.gpio_set(self.b2_pin, (b2 >> bit) & 1 != 0);
            
            // Clock pulse
            self.gpio_set(self.clk_pin, true);
            self.gpio_set(self.clk_pin, false);
        }
    }

    pub fn display_row_pwm(&mut self, row: u8, data: &[u32], brightness_step: u8) {
        // Disable output while shifting data
        self.enable_output(false);
        
        // Set row address
        self.set_row(row);
        
        // Binary Code Modulation for PWM
        let should_light = |color: u8| -> bool {
            color & (1 << (PWM_BITS - 1 - brightness_step)) != 0
        };
 
        // Process each pixel's RGB values
        for pixel in data {
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
 
            // Output bits based on current PWM step
            self.gpio_set(self.r1_pin, should_light(r));
            self.gpio_set(self.g1_pin, should_light(g));
            self.gpio_set(self.b1_pin, should_light(b));
            
            // Clock the data in
            self.gpio_set(self.clk_pin, true);
            self.gpio_set(self.clk_pin, false);
        }
        
        // Latch and display
        self.latch();
        self.enable_output(true);
    }

    pub fn display_frame(&mut self, frame_buffer: &[u32], rows: u8) {
        for brightness in 0..PWM_STEPS {
            for row in 0..rows {
                let row_start = row as usize * 32;  // Assuming 32 pixels per row
                let row_data = &frame_buffer[row_start..row_start + 32];
                self.display_row_pwm(row, row_data, brightness);
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        // Wait for current frame to finish
        self.enable_output(false);
        self.active_buffer = !self.active_buffer;
        self.enable_output(true);
    }
 
    pub fn get_draw_buffer(&mut self) -> &mut FrameBuffer {
        if self.active_buffer {
            &mut self.front_buffer
        } else {
            &mut self.back_buffer
        }
    }
 
    pub fn display_current_buffer(&mut self) {
        let buffer = if self.active_buffer {
            &self.back_buffer
        } else {
            &self.front_buffer
        };
 
        for brightness in 0..PWM_STEPS {
            for row in 0..buffer.height {
                let row_start = row * buffer.width;
                let row_data = &buffer.data[row_start..row_start + buffer.width];
                self.display_row_pwm(row as u8, row_data, brightness);
            }
        }
    }
}