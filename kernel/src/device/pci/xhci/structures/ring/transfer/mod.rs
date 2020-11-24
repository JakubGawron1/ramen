// SPDX-License-Identifier: GPL-3.0-or-later

use super::{super::registers::Registers, raw, CycleBit};
use crate::mem::allocator::page_box::PageBox;
use alloc::rc::Rc;
use bit_field::BitField;
use core::cell::RefCell;
use trb::Trb;
use x86_64::PhysAddr;

mod trb;

const SIZE_OF_RING: usize = 256;

pub struct Ring {
    raw: Raw,
}
impl Ring {
    pub fn new() -> Self {
        Self { raw: Raw::new() }
    }

    pub fn phys_addr(&self) -> PhysAddr {
        self.raw.phys_addr()
    }
}

struct Raw {
    ring: PageBox<[[u32; 4]]>,
    enq_p: usize,
    c: CycleBit,
}
impl Raw {
    fn new() -> Self {
        Self {
            ring: PageBox::new_slice([0; 4], SIZE_OF_RING),
            enq_p: 0,
            c: CycleBit::new(true),
        }
    }

    fn try_enqueue(&mut self, trb: Trb) -> Result<PhysAddr, Error> {
        if self.full() {
            Err(Error::QueueIsFull)
        } else {
            Ok(self.enqueue(trb))
        }
    }

    fn full(&self) -> bool {
        self.c_bit_of_next_trb() == self.c
    }

    fn c_bit_of_next_trb(&self) -> CycleBit {
        let raw = self.ring[self.enq_p];
        CycleBit::new(raw[3].get_bit(0))
    }

    fn enqueue(&mut self, trb: Trb) -> PhysAddr {
        self.write_trb_on_memory(trb);
        let addr_to_trb = self.addr_to_enqueue_ptr();
        self.increment_enqueue_ptr();

        addr_to_trb
    }

    fn write_trb_on_memory(&mut self, trb: Trb) {
        self.ring[self.enq_p] = trb.into();
    }

    fn addr_to_enqueue_ptr(&self) -> PhysAddr {
        self.phys_addr() + Trb::SIZE.as_usize() * self.enq_p
    }

    fn phys_addr(&self) -> PhysAddr {
        self.ring.phys_addr()
    }

    fn increment_enqueue_ptr(&mut self) {
        self.enq_p += 1;
        if self.enq_p < self.len() - 1 {
            return;
        }

        self.append_link_trb();
        self.move_enqueue_ptr_to_the_beginning();
    }

    fn len(&self) -> usize {
        self.ring.len()
    }

    fn append_link_trb(&mut self) {
        self.ring[self.enq_p] = Trb::new_link(self.phys_addr(), self.c).into();
    }

    fn move_enqueue_ptr_to_the_beginning(&mut self) {
        self.enq_p = 0;
        self.c.toggle();
    }
}

#[derive(Debug)]
enum Error {
    QueueIsFull,
}
