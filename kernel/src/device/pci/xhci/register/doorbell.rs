// SPDX-License-Identifier: GPL-3.0-or-later

use {
    super::hc_capability_registers::DoorbellOffset, crate::mem::accessor::Accessor,
    core::convert::TryInto, os_units::Bytes, x86_64::PhysAddr,
};

const NUM_OF_REGISTERS: usize = 256;

pub struct Array(Accessor<[Register]>);
impl Array {
    pub fn new(mmio_base: PhysAddr, db_off: &DoorbellOffset) -> Self {
        Self(Accessor::new_slice(
            mmio_base,
            Bytes::new(db_off.get().try_into().unwrap()),
            NUM_OF_REGISTERS,
        ))
    }

    pub fn notify_to_hc(&mut self) {
        self.0[0].write_for_hc();
    }
}

#[repr(transparent)]
struct Register(u32);
impl Register {
    fn write_for_hc(&mut self) {
        self.0 = 0;
    }
}