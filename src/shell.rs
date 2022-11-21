use core::ptr::null_mut;
use crate::{print, println};
use crate::vga_buf::SCREEN;
use pc_keyboard::DecodedKey;
use lazy_static::lazy_static;

lazy_static! {
    static ref SH: spin::Mutex<Shell> = spin::Mutex::new({
        let mut sh = Shell::new();
        sh
    });
}

pub fn handle_keyboard_interrupt(key: DecodedKey) {
    match key { 
        DecodedKey::Unicode(c) => SH.lock().on_key_pressed(c as u8),
        DecodedKey::RawKey(rk) => {}
    }
} 

struct Shell {
    buf: [u8; 80],
    buf_len: usize,
}

impl Shell {

    pub fn new() -> Shell {
        Shell {
            buf: [0; 80],
            buf_len: 0
        }
    }

    pub fn on_key_pressed(&mut self, key: u8) {
        match key {
            b'\n' => {
                print!("\nImplement command execution: ");
                for i in 0..self.buf_len {
                    print!("{}", self.buf[i] as char)
                }
                println!()
            }
            _ => {
                self.buf[self.buf_len] = key;
                self.buf_len += 1;
                print!("{}", key as char);
            }
        }
    }
}
