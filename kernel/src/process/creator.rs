// SPDX-License-Identifier: GPL-3.0-or-later

use core::convert::TryInto;

use x86_64::{
    structures::paging::{PageSize, PageTable, PageTableFlags, Size4KiB},
    VirtAddr,
};

use crate::mem::{allocator::page_box::PageBox, paging::pml4::PML4};

use super::{stack_frame::StackFrame, Process};

pub struct Creator {
    f: fn() -> !,
}
impl Creator {
    pub fn new(f: fn() -> !) -> Self {
        Self { f }
    }

    pub fn create(self) -> Process {
        Process {
            _stack: None,
            pml4: Pml4Creator::new().create(),
            stack_frame: None,
        }
    }
}

struct Pml4Creator {
    pml4: PageBox<PageTable>,
}
impl Pml4Creator {
    fn new() -> Self {
        Self {
            pml4: PageBox::new(PageTable::new()),
        }
    }

    fn create(mut self) -> PageBox<PageTable> {
        self.enable_recursive_paging();
        self.map_kernel_area();
        self.pml4
    }

    fn enable_recursive_paging(&mut self) {
        let a = self.pml4.phys_addr();
        let f =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
        self.pml4[511].set_addr(a, f);
    }

    fn map_kernel_area(&mut self) {
        self.pml4[510] = PML4.lock().level_4_table()[510].clone();
    }
}
