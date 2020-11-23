// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    add_trb, device::pci::xhci::structures::ring::CycleBit, mem::allocator::page_box::PageBox,
};
use bit_field::BitField;
use core::convert::TryInto;
use x86_64::PhysAddr;

pub enum Control {
    Setup(SetupStage),
    Data(DataStage),
    Status(StatusStage),
}

add_trb!(SetupStage);
impl SetupStage {
    const ID: u8 = 2;

    fn new_get_descriptor<T>(b: &PageBox<T>, dti: DescTyIdx, c: CycleBit) -> Self {
        let mut t = Self::null();
        t.set_request_type(0b1000_0000);
        t.set_request(Request::GetDescriptor);
        t.set_value(dti.bits());
        t.set_length(b.bytes().as_usize().try_into().unwrap());
        t.set_trb_transfer_length(8);
        t.set_cycle_bit(c);
        t.set_trb_type(Self::ID);
        t.set_trt(3);
        t
    }

    fn null() -> Self {
        Self([0; 4])
    }

    fn set_request_type(&mut self, t: u8) {
        self.0[0].set_bits(0..=7, t.into());
    }

    fn set_request(&mut self, r: Request) {
        self.0[0].set_bits(8..=15, r as _);
    }

    fn set_value(&mut self, v: u16) {
        self.0[0].set_bits(16..=31, v.into());
    }

    fn set_index(&mut self, i: u16) {
        self.0[1].set_bits(0..=15, i.into());
    }

    fn set_length(&mut self, l: u16) {
        self.0[1].set_bits(16..=31, l.into());
    }

    fn set_trb_transfer_length(&mut self, l: u32) {
        self.0[2].set_bits(0..=16, l);
    }

    fn set_trt(&mut self, t: u8) {
        self.0[3].set_bits(16..=17, t.into());
    }
}

struct DescTyIdx {
    ty: DescTy,
    i: u8,
}
impl DescTyIdx {
    fn new(ty: DescTy, i: u8) -> Self {
        Self { ty, i }
    }
    fn bits(self) -> u16 {
        (self.ty as u16) << 8 | u16::from(self.i)
    }
}

enum Request {
    GetDescriptor = 6,
}

enum DescTy {
    Device = 1,
}

add_trb!(DataStage);
impl DataStage {
    const ID: u8 = 3;

    fn new<T>(b: &PageBox<T>, c: CycleBit, d: Direction) -> Self {
        let mut t = Self::null();
        t.set_data_buf(b.phys_addr());
        t.set_transfer_length(b.bytes().as_usize().try_into().unwrap());
        t.set_cycle_bit(c);
        t.set_trb_type(Self::ID);
        t.set_dir(d);
        t
    }

    fn null() -> Self {
        Self([0; 4])
    }

    fn set_data_buf(&mut self, b: PhysAddr) {
        let l = b.as_u64() & 0xffff_ffff;
        let u = b.as_u64() >> 32;

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
    }

    fn set_transfer_length(&mut self, l: u32) {
        self.0[2].set_bits(0..=16, l);
    }

    fn set_td_size(&mut self, s: u8) {
        self.0[2].set_bits(17..=21, s.into());
    }

    fn set_dir(&mut self, d: Direction) {
        self.0[3].set_bit(16, d.into());
    }
}

enum Direction {
    Out = 0,
    In = 1,
}
impl From<Direction> for bool {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Out => false,
            Direction::In => true,
        }
    }
}

add_trb!(StatusStage);
impl StatusStage {
    const ID: u8 = 4;

    fn new(c: CycleBit) -> Self {
        let mut t = Self::null();
        t.set_cycle_bit(c);
        t.set_ioc(true);
        t.set_trb_type(Self::ID);
        t
    }

    fn null() -> Self {
        Self([0; 4])
    }

    fn set_ioc(&mut self, ioc: bool) {
        self.0[3].set_bit(5, ioc);
    }
}