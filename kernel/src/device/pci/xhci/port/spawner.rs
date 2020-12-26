// SPDX-License-Identifier: GPL-3.0-or-later

use super::Port;
use crate::{
    device::pci::xhci::exchanger::{command, receiver::Receiver},
    multitask, Futurelock,
};
use alloc::{sync::Arc, vec, vec::Vec};
use conquer_once::spin::{Lazy, OnceCell};
use multitask::task::Task;
use spinning_top::Spinlock;

static SPAWN_STATUS: Lazy<Spinlock<Vec<bool>>> =
    Lazy::new(|| Spinlock::new(vec![false; super::max_num().into()]));
static SPAWNER: OnceCell<Spawner> = OnceCell::uninit();

pub fn init(sender: Arc<Futurelock<command::Sender>>, receiver: Arc<Spinlock<Receiver>>) {
    SPAWNER
        .try_init_once(|| Spawner::new(sender, receiver))
        .expect("SPAWNER is already initialized.")
}

pub fn spawn_all_connected_ports() {
    spawner().spawn_all_connected_ports();
}

pub fn try_spawn(port_idx: u8) -> Result<(), PortNotConnected> {
    spawner().try_spawn(port_idx)
}

fn spawner() -> &'static Spawner {
    SPAWNER.try_get().expect("SPAWNER is not initialized.")
}

struct Spawner {
    sender: Arc<Futurelock<command::Sender>>,
    receiver: Arc<Spinlock<Receiver>>,
}
impl Spawner {
    fn new(sender: Arc<Futurelock<command::Sender>>, receiver: Arc<Spinlock<Receiver>>) -> Self {
        Self { sender, receiver }
    }

    fn spawn_all_connected_ports(&self) {
        let n = super::max_num();
        for i in 0..n {
            let _ = self.try_spawn(i + 1);
        }
    }

    fn try_spawn(&self, port_idx: u8) -> Result<(), PortNotConnected> {
        let p = Port::new(port_idx);
        if spawnable(&p) {
            self.spawn(p);
            Ok(())
        } else {
            Err(PortNotConnected)
        }
    }

    fn spawn(&self, p: Port) {
        mark_as_spawned(&p);
        self.add_task_for_port(p);
    }

    fn add_task_for_port(&self, p: Port) {
        multitask::add(Task::new(super::task(
            p,
            self.sender.clone(),
            self.receiver.clone(),
        )));
    }
}

fn spawnable(p: &Port) -> bool {
    p.connected() && !spawned(p)
}

fn spawned(p: &Port) -> bool {
    SPAWN_STATUS.lock()[usize::from(p.index)]
}

fn mark_as_spawned(p: &Port) {
    SPAWN_STATUS.lock()[usize::from(p.index)] = true;
}

#[derive(Debug)]
pub struct PortNotConnected;