use core::fmt::{self, Arguments, Result, Write};

// Copied from Rust sources
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

// Copied from Rust sources
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::vga::_print(format_args_nl!($($arg)*));
    })
}

pub fn _print(args: Arguments) {
    unsafe {
        fmt::write(&mut __VGA, args).unwrap();
    }
}

pub fn clear_screen() {
    unsafe {
        for y in 0..__VGA.height() {
            for x in 0..__VGA.width() {
                __VGA.putc(x, y, ' ');
            }
        }
    }
}

struct VGA {
    width: u8,
    height: u8,
    cur_x: u8,
    cur_y: u8,
}

static mut __VGA: VGA = VGA::new(80, 25);

impl VGA {
    pub const fn new(width: u8, height: u8) -> VGA {
        VGA {
            width,
            height,
            cur_x: 0,
            cur_y: 0,
        }
    }

    fn width(&self) -> u8 { self.width }

    fn height(&self) -> u8 { self.height }

    fn set_cursor(&mut self, x: u8, y: u8) {
        self.cur_x = x;
        self.cur_y = y;
    }

    fn putc(&self, x: u8, y: u8, ch: char) {
        if x >= self.width || y >= self.height {
            panic!("Invalid VGA::putc()");
        }
        let vga_base = 0xb8000 as *mut u8;
        let offset = 2 * ((y as isize) * (self.width as isize) + (x as isize));
        unsafe {
            *vga_base.offset(offset) = ch as u8;
        }
    }

    fn puts(&self, x: u8, y: u8, s: &str) {
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
        let mut line = s;
        while line.len() > 0 {
            match line.find(|c| c == '\n') {
                Some(idx) => {
                    let (part, rest) = line.split_at(idx);
                    self.puts(self.cur_x, self.cur_y, part);
                    self.cur_x = 0;
                    self.cur_y += 1;
                    line = &rest[1..];
                }
                None => {
                    self.puts(self.cur_x, self.cur_y, line);
                    self.cur_x += s.bytes().len() as u8;
                    break;
                }
            }
        }
        Ok(())
    }
}
