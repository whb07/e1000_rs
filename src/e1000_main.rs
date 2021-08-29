
use crate::e1000::{intel_e1000_ethernet_device, NetDevice, PciDeviceId};

use crate::lib::*;

pub const IRQRETURN_IRQ_NONE: Irqreturn = 0;
pub const IRQRETURN_IRQ_HANDLED: Irqreturn = 1;
pub const IRQRETURN_IRQ_WAKE_THREAD: Irqreturn = 2;
pub type Irqreturn = u32;

pub fn e1000_intr(_irq: i32, _data: &NetDevice) -> Irqreturn {
    IRQRETURN_IRQ_NONE
}

pub struct atomic_t {
    counter: Cell<i32>,
}

pub fn atomic_inc(n: &mut atomic_t) {
    n.counter.set(n.counter.get() + 1);
}

const E1000_PCI_TBL: &[PciDeviceId] = &[
    intel_e1000_ethernet_device(0x1000),
    intel_e1000_ethernet_device(0x1001),
    intel_e1000_ethernet_device(0x1004),
    intel_e1000_ethernet_device(0x1008),
    intel_e1000_ethernet_device(0x1009),
    intel_e1000_ethernet_device(0x100C),
    intel_e1000_ethernet_device(0x100D),
    intel_e1000_ethernet_device(0x100E),
    intel_e1000_ethernet_device(0x100F),
    intel_e1000_ethernet_device(0x1010),
    intel_e1000_ethernet_device(0x1011),
    intel_e1000_ethernet_device(0x1012),
    intel_e1000_ethernet_device(0x1013),
    intel_e1000_ethernet_device(0x1014),
    intel_e1000_ethernet_device(0x1015),
    intel_e1000_ethernet_device(0x1016),
    intel_e1000_ethernet_device(0x1017),
    intel_e1000_ethernet_device(0x1018),
    intel_e1000_ethernet_device(0x1019),
    intel_e1000_ethernet_device(0x101A),
    intel_e1000_ethernet_device(0x101D),
    intel_e1000_ethernet_device(0x101E),
    intel_e1000_ethernet_device(0x1026),
    intel_e1000_ethernet_device(0x1027),
    intel_e1000_ethernet_device(0x1028),
    intel_e1000_ethernet_device(0x1075),
    intel_e1000_ethernet_device(0x1076),
    intel_e1000_ethernet_device(0x1077),
    intel_e1000_ethernet_device(0x1078),
    intel_e1000_ethernet_device(0x1079),
    intel_e1000_ethernet_device(0x107A),
    intel_e1000_ethernet_device(0x107B),
    intel_e1000_ethernet_device(0x107C),
    intel_e1000_ethernet_device(0x108A),
    intel_e1000_ethernet_device(0x1099),
    intel_e1000_ethernet_device(0x10B5),
    intel_e1000_ethernet_device(0x2E6E),
];

// fn e1000_get_hw_dev(hw: &E1000Hw) -> NetDevice {
//     hw.back.netdev
// }
//
// // fn e1000_request_irq(adapter: &E1000Adapter) -> i32 {
// //     let netdev = adapter.netdev;
// //
// // }
//
// pub fn e1000_irq_disable(adapter: &E1000Adapter){
//     atomic_inc(adapter.irq_sem);
// }
//
// pub fn e1000_down(adapter: &E1000Adapter){
//     // let netdev: NetDevice = adapter.netdev;
//
// }
//
//
//
// pub fn e1000_tx_timeout(netdev: &NetDevice){
//     let x = Hello {counter: Cell::new(10)};
//     e1000_down(adapter);
//     e1000_up(adapter);
// }
