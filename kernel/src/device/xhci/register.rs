// SPDX-License-Identifier: GPL-3.0-or-later

use bit::BitIndex;
use proc_macros::add_register_type;

add_register_type! {
    pub struct UsbLegacySupportCapability: u32{
        capability_id: 0..8,
        hc_bios_owned_semaphore: 16..17,
        hc_os_owned_semaphore: 24..25,
    }
}

add_register_type! {
    pub struct HCCapabilityParameters1:u32{
        xhci_extended_capabilities_pointer:16..32,
    }
}
