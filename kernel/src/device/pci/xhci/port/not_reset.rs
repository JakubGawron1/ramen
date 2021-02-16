// SPDX-License-Identifier: GPL-3.0-or-later

use super::slot_not_assigned::SlotNotAssigned;
use crate::device::pci::xhci::structures::registers;
use xhci::registers::PortRegisterSet;

pub(super) struct NotReset {
    port_number: u8,
}
impl NotReset {
    pub(super) fn new(port_number: u8) -> Self {
        Self { port_number }
    }

    pub(super) fn port_number(&self) -> u8 {
        self.port_number
    }

    pub(super) fn connected(&self) -> bool {
        self.read_port_register(|r| r.portsc.current_connect_status())
    }

    pub(super) fn reset(self) -> SlotNotAssigned {
        self.start_resetting();
        self.wait_until_reset_is_completed();
        SlotNotAssigned::new(self.port_number)
    }

    fn start_resetting(&self) {
        self.update_port_register(|r| r.portsc.set_port_reset(true));
    }

    fn wait_until_reset_is_completed(&self) {
        while !self.reset_completed() {}
    }

    fn reset_completed(&self) -> bool {
        self.read_port_register(|r| r.portsc.port_reset_changed())
    }

    fn read_port_register<T, U>(&self, f: T) -> U
    where
        T: FnOnce(&PortRegisterSet) -> U,
    {
        registers::handle(|r| f(&r.port_register_set.read_at((self.port_number - 1).into())))
    }

    fn update_port_register<T>(&self, f: T)
    where
        T: FnOnce(&mut PortRegisterSet),
    {
        registers::handle(|r| {
            r.port_register_set
                .update_at((self.port_number - 1).into(), f)
        })
    }
}
