// SPDX-License-Identifier: GPL-3.0-or-later

use {
    boot_info::mem::MemoryDescriptor,
    core::ops::DerefMut,
    frame_manager::FrameManager,
    os_units::NumOfPages,
    spinning_top::Spinlock,
    x86_64::{
        structures::paging::{FrameAllocator, FrameDeallocator, Size4KiB},
        PhysAddr,
    },
};

static FRAME_MANAGER: Spinlock<FrameManager> = Spinlock::new(FrameManager::new());

pub(crate) fn init(mem_map: &[MemoryDescriptor]) {
    FRAME_MANAGER.lock().init(mem_map);
}

pub(in super::super) fn allocator(
) -> impl DerefMut<Target = impl FrameAllocator<Size4KiB> + FrameDeallocator<Size4KiB>> {
    lock_manager()
}

pub(super) fn alloc(num_of_pages: NumOfPages<Size4KiB>) -> Option<PhysAddr> {
    lock_manager().deref_mut().alloc(num_of_pages)
}

pub(super) fn free(addr: PhysAddr) {
    lock_manager().deref_mut().free(addr);
}

fn lock_manager() -> impl DerefMut<Target = FrameManager> {
    FRAME_MANAGER
        .try_lock()
        .expect("Failed to lock the frame manager.")
}
