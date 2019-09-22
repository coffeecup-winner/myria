use core::fmt::{Result, Write};

pub struct VGA {
    width: u8,
    height: u8,
    cur_x: u8,
    cur_y: u8,
}

impl VGA {
    pub fn new(width: u8, height: u8) -> VGA {
        VGA {
            width,
            height,
            cur_x: 0,
            cur_y: 0,
        }
    }

    pub fn width(&self) -> u8 { self.width }

    pub fn height(&self) -> u8 { self.height }

    pub fn set_cursor(&mut self, x: u8, y: u8) {
        self.cur_x = x;
        self.cur_y = y;
    }

    pub fn putc(&self, x: u8, y: u8, ch: char) {
        if x >= self.width || y >= self.height {
            panic!("Invalid VGA::putc()");
        }
        let vga_base = 0xb8000 as *mut u8;
        let offset = 2 * ((y as isize) * (self.width as isize) + (x as isize));
        unsafe {
            *vga_base.offset(offset) = ch as u8;
        }
    }

    pub fn puts(&self, x: u8, y: u8, s: &str) {
        if s.len() > (self.width - x) as usize || y >= self.height {
            panic!("Invalid VGA::puts()")
        }
        let vga_base = 0xb8000 as *mut u8;
        let mut offset = 2 * ((y as isize) * (self.width as isize) + (x as isize));
        for ch in s.bytes() {
            unsafe {
                *vga_base.offset(offset) = ch;
                offset += 2;
            }
        }
    }
}

impl Write for VGA {
    fn write_str(&mut self, s: &str) -> Result {
        self.puts(self.cur_x, self.cur_y, s);
        self.cur_x += s.bytes().len() as u8;
        Ok(())
    }
}
