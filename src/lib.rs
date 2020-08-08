#![no_std]
#![feature(asm)]
#![feature(start)]
#![feature(naked_functions)]

#[macro_use]
#[allow(unused_imports)]
extern crate debug;

extern crate common_items;

mod asm;
mod descriptor_table;
mod gdt;
mod interrupt;
mod memory;
mod queue;

#[macro_use]
mod graphics;

use interrupt::handler;

#[no_mangle]
#[start]
pub fn os_main() {
    let boot_info = common_items::BootInfo::get();
    let vram = graphics::Vram::new_from_boot_info(&boot_info);

    let mut mouse_device: interrupt::mouse::Device = interrupt::mouse::Device::new(&vram);
    let mut mouse_cursor: graphics::screen::MouseCursor = graphics::screen::MouseCursor::new(
        graphics::RGB::new(0x008484),
        graphics::screen::MOUSE_GRAPHIC,
        &vram,
    );

    initialization(&mut mouse_device, &mut mouse_cursor, &vram);

    main_loop(&mut mouse_device, &mut mouse_cursor, &vram)
}

fn initialization(
    mouse_device: &mut interrupt::mouse::Device,
    mouse_cursor: &mut graphics::screen::MouseCursor,
    vram: &graphics::Vram,
) -> () {
    gdt::init();
    descriptor_table::init();
    interrupt::init_pic();
    // Temporarily disable interruption to see whether desktop is drawn successfully or not.
    // asm::sti();

    graphics::screen::draw_desktop(&vram);

    print_with_pos!(
        vram,
        graphics::screen::Coord::new(16, 64),
        graphics::RGB::new(0xFFFFFF),
        "x_len = {}",
        vram.x_len
    );

    interrupt::set_init_pic_bits();
    interrupt::init_keyboard();
    mouse_device.enable();

    mouse_cursor.draw_offset(graphics::screen::Coord::new(300, 300))
}

fn main_loop(
    mouse_device: &mut interrupt::mouse::Device,
    mouse_cursor: &mut graphics::screen::MouseCursor,
    vram: &graphics::Vram,
) -> () {
    loop {
        asm::cli();
        if interrupt::KEY_QUEUE.lock().size() != 0 {
            handler::keyboard_data(vram);
        } else if interrupt::mouse::QUEUE.lock().size() != 0 {
            handler::mouse_data(mouse_device, mouse_cursor, vram);
        } else {
            asm::stihlt();
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        asm::hlt()
    }
}
