// SPDX-License-Identifier: GPL-3.0-or-later

use {crate::mem::accessor::Accessor, bitfield::bitfield, os_units::Bytes, x86_64::PhysAddr};

pub struct RuntimeBaseRegisters {
    erst_sz: Accessor<EventRingSegmentTableSizeRegister>,
    erst_ba: Accessor<EventRingSegmentTableBaseAddressRegister>,
    erd_p: Accessor<EventRingDequeuePointerRegister>,
}
impl<'a> RuntimeBaseRegisters {
    pub fn new(mmio_base: PhysAddr, runtime_register_space_offset: usize) -> Self {
        let runtime_base = mmio_base + runtime_register_space_offset;
        let erst_sz = Accessor::new(runtime_base, Bytes::new(0x28));
        let erst_ba = Accessor::new(runtime_base, Bytes::new(0x30));
        let erd_p = Accessor::new(runtime_base, Bytes::new(0x38));

        Self {
            erst_sz,
            erst_ba,
            erd_p,
        }
    }

    pub fn set_event_ring_segment_table_size(&mut self, size: u16) {
        self.erst_sz.set(size)
    }

    pub fn set_event_ring_segment_table_addr(&mut self, addr: PhysAddr) {
        self.erst_ba.set(addr)
    }

    pub fn set_event_ring_dequeue_ptr(&mut self, ptr: PhysAddr) {
        self.erd_p.set_address(ptr)
    }
}

bitfield! {
    #[repr(transparent)]
     struct InterruptManagementRegister(u32);

     interrupt_pending,set_interrupt_pending: 0;
     interrupt_enable, set_interrupt_status: 1;
}

bitfield! {
    #[repr(transparent)]
    struct InterruptModerationRegister(u32);

    interrupt_moderation_interval, set_interrupt_interval: 15, 0;
}

#[repr(transparent)]
#[derive(Debug)]
struct EventRingSegmentTableSizeRegister(u32);
impl EventRingSegmentTableSizeRegister {
    fn set(&mut self, val: u16) {
        self.0 = u32::from(val)
    }
}

#[repr(transparent)]
#[derive(Debug)]
struct EventRingSegmentTableBaseAddressRegister(u64);
impl EventRingSegmentTableBaseAddressRegister {
    fn set(&mut self, addr: PhysAddr) {
        let addr = addr.as_u64();
        assert!(addr.trailing_zeros() >= 6);
        self.0 = addr
    }
}

#[repr(transparent)]
struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    fn set_address(&mut self, addr: PhysAddr) {
        assert!(addr.as_u64().trailing_zeros() >= 4);
        self.0 = addr.as_u64();
    }
}