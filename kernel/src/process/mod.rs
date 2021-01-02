// SPDX-License-Identifier: GPL-3.0-or-later

mod creator;
mod manager;
mod stack_frame;

use crate::{mem::allocator::page_box::PageBox, tests, tss::TSS};
use common::constant::INTERRUPT_STACK;
use conquer_once::spin::OnceCell;
use creator::Creator;
use stack_frame::StackFrame;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{PageTable, PhysFrame},
    VirtAddr,
};

static KERNEL_PML4: OnceCell<PhysFrame> = OnceCell::uninit();

pub fn init() {
    register_initial_interrupt_stack_table_addr();
    save_kernel_pml4();
}

pub fn add(p: Process) {
    manager::add_process(p);
}

pub fn switch() -> VirtAddr {
    if cfg!(feature = "qemu_test") {
        tests::process::count_switch();
    }
    manager::switch_process()
}

fn register_initial_interrupt_stack_table_addr() {
    TSS.lock().interrupt_stack_table[0] = INTERRUPT_STACK;
}

fn save_kernel_pml4() {
    KERNEL_PML4
        .try_init_once(|| Cr3::read().0)
        .expect("`KERNEL_PML4` is already initialized.");
}

pub struct Process {
    _stack: Option<PageBox<[u8]>>,
    pml4: PageBox<PageTable>,
    stack_frame: Option<PageBox<StackFrame>>,
}
impl Process {
    pub fn new(f: fn() -> !) -> Self {
        Creator::new(f).create()
    }

    fn stack_frame_top_addr(&self) -> VirtAddr {
        self.stack_frame().virt_addr()
    }

    fn stack_frame_bottom_addr(&self) -> VirtAddr {
        let b = self.stack_frame().bytes();
        self.stack_frame_top_addr() + b.as_usize()
    }

    fn stack_frame(&self) -> &PageBox<StackFrame> {
        self.stack_frame
            .as_ref()
            .expect("Stack frame is not created")
    }
}
