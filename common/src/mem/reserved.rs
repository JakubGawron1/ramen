// SPDX-License-Identifier: GPL-3.0-or-later

use crate::constant::{KERNEL_ADDR, NUM_OF_PAGES_STACK, STACK_LOWER, VRAM_ADDR};
use crate::size::{Byte, Size};
use crate::vram;
use x86_64::{PhysAddr, VirtAddr};

pub struct KernelPhysRange {
    start: PhysAddr,
    bytes: Size<Byte>,
}

impl KernelPhysRange {
    pub fn new(start: PhysAddr, bytes: Size<Byte>) -> Self {
        Self { start, bytes }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Map([Range; 3]);
impl Map {
    pub fn new(kernel: &KernelPhysRange, phys_addr_stack: PhysAddr, vram: &vram::Info) -> Self {
        Self {
            0: [
                Range::kernel(&kernel),
                Range::stack(phys_addr_stack),
                Range::vram(vram),
            ],
        }
    }

    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Range> {
        self.0.iter()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Range {
    virt: VirtAddr,
    phys: PhysAddr,
    bytes: Size<Byte>,
}

impl Range {
    #[must_use]
    pub fn kernel(kernel: &KernelPhysRange) -> Self {
        Self {
            virt: KERNEL_ADDR,
            phys: kernel.start,
            bytes: kernel.bytes,
        }
    }

    #[must_use]
    pub fn vram(vram: &vram::Info) -> Self {
        Self {
            virt: VRAM_ADDR,
            phys: vram.phys_ptr(),
            bytes: vram.bytes(),
        }
    }

    #[must_use]
    pub fn stack(phys: PhysAddr) -> Self {
        Self {
            virt: STACK_LOWER,
            phys,
            bytes: NUM_OF_PAGES_STACK.as_bytes(),
        }
    }

    #[must_use]
    pub fn virt(&self) -> VirtAddr {
        self.virt
    }

    #[must_use]
    pub fn phys(&self) -> PhysAddr {
        self.phys
    }

    #[must_use]
    pub fn bytes(&self) -> Size<Byte> {
        self.bytes
    }
}
