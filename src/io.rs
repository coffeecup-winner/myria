extern "C" {
    pub fn _outb(port: u16, value: u8);
    pub fn _inb(port: u16) -> u8;
    pub fn _iowait();
}
