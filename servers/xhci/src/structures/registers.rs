// SPDX-License-Identifier: GPL-3.0-or-later

use {
    conquer_once::spin::OnceCell, core::convert::TryInto, ralib::mem::accessor::Mapper,
    spinning_top::Spinlock, x86_64::PhysAddr, xhci::Registers,
};

static REGISTERS: OnceCell<Spinlock<Registers<Mapper>>> = OnceCell::uninit();

pub(crate) unsafe fn init(mmio_base: PhysAddr) {
    let mmio_base: usize = mmio_base.as_u64().try_into().unwrap();

    REGISTERS
        .try_init_once(|| Spinlock::new(unsafe { Registers::new(mmio_base, Mapper) }))
        .expect("Failed to initialize `REGISTERS`.");
}

/// Handle xHCI registers.
///
/// To avoid deadlocking, this method takes a closure. Caller is supposed not to call this method
/// inside the closure, otherwise a deadlock will happen.
///
/// Alternative implementation is to define a method which returns `impl Deref<Target =
/// Registers>`, but this will expand the scope of the mutex guard, increasing the possibility of
/// deadlocks.
pub(crate) fn handle<T, U>(f: T) -> U
where
    T: FnOnce(&mut Registers<Mapper>) -> U,
{
    let mut r = REGISTERS.try_get().unwrap().lock();
    f(&mut r)
}
