// SPDX-License-Identifier: GPL-3.0-or-later

pub mod log;
pub mod writer;

use super::{font, Vram};
use core::{cmp, convert::TryFrom};
use rgb::RGB8;
use vek::Vec2;

pub const MOUSE_CURSOR_WIDTH: usize = 16;
pub const MOUSE_CURSOR_HEIGHT: usize = 16;

pub const MOUSE_GRAPHIC: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
    [
        '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '0', '0', '0', '*', '*', '*', '*', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '0', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '0', '*', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '*', '.', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '*', '.', '.', '.', '.', '*', '0', '0', '*', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '.', '.', '.', '.', '.', '*', '0', '*', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
    [
        '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
    ],
];

#[macro_export]
macro_rules! print_with_pos {
    ($coord:expr,$color:expr,$text:expr,$($args:expr),*) => {
        let mut screen_write =
            crate::graphics::screen::writer::Writer::new($coord, $color);

        // To narrow the scope of `use core::fmt::Write;`, enclose sentences by curly braces.
        {
            use core::fmt::Write;
            write!(screen_write, $text, $($args,)*).unwrap();
        }
    };
}

pub struct Screen;

impl Screen {
    // TODO: Specify top left coordinate and length, rather than two coordinates.
    pub fn draw_rectangle(color: RGB8, top_left: &Vec2<isize>, bottom_right: &Vec2<isize>) {
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                unsafe {
                    Vram::set_color(&Vec2::new(x, y), color);
                }
            }
        }
    }
}

pub struct MouseCursor {
    coord: Vec2<isize>,
    image: [[RGB8; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
}

impl MouseCursor {
    pub fn new(
        background_color: RGB8,
        image: [[char; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
    ) -> Self {
        let mut colored_dots: [[RGB8; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] =
            [[background_color; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_WIDTH];

        for y in 0..MOUSE_CURSOR_HEIGHT {
            for x in 0..MOUSE_CURSOR_WIDTH {
                colored_dots[y][x] = match image[y][x] {
                    '*' => RGB8::new(0, 0, 0),
                    '0' => RGB8::new(0xff, 0xff, 0xff),
                    _ => background_color,
                }
            }
        }

        Self {
            coord: Vec2::new(0, 0),
            image: colored_dots,
        }
    }

    pub fn print_coord(&mut self, coord: Vec2<isize>) {
        Screen::draw_rectangle(
            RGB8::new(0x00, 0x84, 0x84),
            &Vec2::new(16, 32),
            &Vec2::new(16 + 8 * 12 - 1, 32 + 15),
        );

        print_with_pos!(
            coord,
            RGB8::new(0xff, 0xff, 0xff),
            "({}, {})",
            self.coord.x,
            self.coord.y
        );
    }

    pub fn draw_offset(&mut self, offset: Vec2<isize>) {
        let new_coord = self.coord + offset;
        self.draw(new_coord)
    }

    fn put_coord_on_screen(mut coord: Vec2<isize>) -> Vec2<isize> {
        coord.x = cmp::max(coord.x, 0);
        coord.y = cmp::max(coord.y, 0);

        coord.x = cmp::min(
            coord.x,
            isize::try_from(Vram::resolution().x - MOUSE_CURSOR_WIDTH - 1).unwrap(),
        );
        coord.y = cmp::min(
            coord.y,
            isize::try_from(Vram::resolution().y - MOUSE_CURSOR_HEIGHT - 1).unwrap(),
        );

        coord
    }

    pub fn draw(&mut self, coord: Vec2<isize>) {
        self.remove_previous_cursor();

        let adjusted_coord = Self::put_coord_on_screen(coord);
        for y in 0..MOUSE_CURSOR_HEIGHT {
            for x in 0..MOUSE_CURSOR_WIDTH {
                unsafe {
                    Vram::set_color(
                        &(adjusted_coord
                            + Vec2::new(isize::try_from(x).unwrap(), isize::try_from(y).unwrap())),
                        self.image[y][x],
                    );
                }
            }
        }

        self.coord = adjusted_coord;
    }

    fn remove_previous_cursor(&self) {
        Screen::draw_rectangle(
            RGB8::new(0, 0x84, 0x84),
            &Vec2::new(self.coord.x, self.coord.y),
            &Vec2::new(
                self.coord.x + isize::try_from(MOUSE_CURSOR_WIDTH).unwrap(),
                self.coord.y + isize::try_from(MOUSE_CURSOR_HEIGHT).unwrap(),
            ),
        );
    }
}

#[rustfmt::skip]
pub fn draw_desktop() {
    let x_len: isize = isize::try_from(Vram::resolution().x).unwrap();
    let y_len: isize = isize::try_from(Vram::resolution().y).unwrap();

    // It seems that changing the arguments as `color, coord_1, coord_2` actually makes the code
    // dirty because by doing it lots of `Coord::new(x1, x2)` appear on below.
    let draw_desktop_part = |color, x0, y0, x1, y1| {
        let rgb = RGB8::new(
            u8::try_from((color >> 16) & 0xff).unwrap(),
            u8::try_from((color >> 8) & 0xff).unwrap(),
            u8::try_from(color & 0xff).unwrap(),
        );
        Screen::draw_rectangle(rgb, &Vec2::new(x0, y0), &Vec2::new(x1, y1))
    };

    draw_desktop_part(0x0000_8484,          0,          0, x_len -  1, y_len - 29);
    draw_desktop_part(0x00C6_C6C6,          0, y_len - 28, x_len -  1, y_len - 28);
    draw_desktop_part(0x00FF_FFFF,          0, y_len - 27, x_len -  1, y_len - 27);
    draw_desktop_part(0x00C6_C6C6,          0, y_len - 26, x_len -  1, y_len -  1);

    draw_desktop_part(0x00FF_FFFF,          3, y_len - 24,         59, y_len - 24);
    draw_desktop_part(0x00FF_FFFF,          2, y_len - 24,          2, y_len -  4);
    draw_desktop_part(0x0084_8484,          3, y_len -  4,         59, y_len -  4);
    draw_desktop_part(0x0084_8484,         59, y_len - 23,         59, y_len -  5);
    draw_desktop_part(0x0000_0000,          2, y_len -  3,         59, y_len -  3);
    draw_desktop_part(0x0000_0000,         60, y_len - 24,         60, y_len -  3);

    draw_desktop_part(0x0084_8484, x_len - 47, y_len - 24, x_len -  4, y_len - 24);
    draw_desktop_part(0x0084_8484, x_len - 47, y_len - 23, x_len - 47, y_len -  4);
    draw_desktop_part(0x00FF_FFFF, x_len - 47, y_len -  3, x_len -  4, y_len -  3);
    draw_desktop_part(0x00FF_FFFF, x_len -  3, y_len - 24, x_len -  3, y_len -  3);
}
