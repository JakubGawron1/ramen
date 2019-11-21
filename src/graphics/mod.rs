use crate::asm;

pub mod font;
pub mod screen;

pub struct Vram {
    pub x_len: i16,
    pub y_len: i16,
    pub ptr: *mut u8,
}

impl Vram {
    pub fn new() -> Vram {
        Vram {
            x_len: unsafe { *(0x0ff4 as *const i16) },
            y_len: unsafe { *(0x0ff6 as *const i16) },
            ptr: unsafe { &mut *(*(0x0ff8 as *const i32) as *mut u8) },
        }
    }

    pub fn init_palette(&self) -> () {
        const RGB_TABLE: [[u8; 3]; 16] = [
            [0x00, 0x00, 0x00],
            [0xff, 0x00, 0x00],
            [0x00, 0xff, 0x00],
            [0xff, 0xff, 0x00],
            [0x00, 0x00, 0xff],
            [0xff, 0x00, 0xff],
            [0x00, 0xff, 0xff],
            [0xff, 0xff, 0xff],
            [0xc6, 0xc6, 0xc6],
            [0x84, 0x00, 0x00],
            [0x00, 0x84, 0x00],
            [0x84, 0x84, 0x00],
            [0x00, 0x00, 0x84],
            [0x84, 0x00, 0x84],
            [0x00, 0x84, 0x84],
            [0x84, 0x84, 0x84],
        ];

        Self::set_palette(0, 15, RGB_TABLE);
    }

    fn set_palette(start: i32, end: i32, rgb: [[u8; 3]; 16]) -> () {
        let eflags: i32 = asm::load_eflags();
        asm::cli();
        asm::out8(0x03c8, start);
        for i in start..(end + 1) {
            for j in 0..3 {
                asm::out8(0x03c9, (rgb[i as usize][j] >> 2) as i32);
            }
        }
        asm::store_eflags(eflags);
    }
}

#[derive(Clone, Copy)]
pub enum ColorIndex {
    Rgb000000 = 0,
    RgbFF0000 = 1,
    Rgb00FF00 = 2,
    RgbFFFF00 = 3,
    Rgb0000FF = 4,
    RgbFF00FF = 5,
    Rgb00FFFF = 6,
    RgbFFFFFF = 7,
    RgbC6C6C6 = 8,
    Rgb840000 = 9,
    Rgb008400 = 10,
    Rgb848400 = 11,
    Rgb000084 = 12,
    Rgb840084 = 13,
    Rgb008484 = 14,
    Rgb848484 = 15,
}