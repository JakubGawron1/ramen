// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    tss::TSS,
    x86_64::{
        instructions::{segmentation, tables},
        structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    },
};
use conquer_once::spin::Lazy;
use x86_64::registers::model_specific::Star;

pub static GDT: Lazy<Gdt> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
    let kernel_data = gdt.add_entry(Descriptor::kernel_data_segment());
    let user_data = gdt.add_entry(Descriptor::user_data_segment());
    let user_code = gdt.add_entry(Descriptor::user_code_segment());

    // SAFETY: This operation is safe because there is no instances of `MutexGuard` which wraps
    // `TSS`.
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(unsafe { &*TSS.data_ptr() }));

    Gdt {
        table: gdt,
        kernel_code,
        kernel_data,
        user_data,
        user_code,
        tss_selector,
    }
});

pub struct Gdt {
    table: GlobalDescriptorTable,
    pub kernel_data: SegmentSelector,
    pub kernel_code: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.table.load();
    unsafe {
        segmentation::set_cs(GDT.kernel_code);

        segmentation::load_ds(GDT.kernel_data);
        segmentation::load_es(GDT.kernel_data);
        segmentation::load_fs(GDT.kernel_data);
        segmentation::load_gs(GDT.kernel_data);
        segmentation::load_ss(GDT.kernel_data);
        tables::load_tss(GDT.tss_selector);
    }

    init_star();
}

fn init_star() {
    Star::write(
        GDT.user_code,
        GDT.user_data,
        GDT.kernel_code,
        GDT.kernel_data,
    )
    .unwrap();
}
