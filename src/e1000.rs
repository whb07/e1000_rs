use crate::lib::*;

use crate::deps::timer::{
    atomic_t, gro_list, hlist_node, hrtimer, list_head, spinlock_t, timer_list, work_struct,
};

use crate::deps::pci::PciDev;
use crate::e1000_hw::{E1000Hw, E1000HwStats, E1000PhyInfo, E1000PhyStats, E1000TxDesc};

use crate::lib::fmt::Formatter;

pub const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
pub const PCI_ANY_ID: u32 = !0;
pub const IFNAMSIZ: usize = 16;

pub type BooleanT = usize;
pub const FALSE: BooleanT = 0;
pub const TRUE: BooleanT = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct PciDeviceId {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: u64,
}

pub const fn intel_e1000_ethernet_device(device_id: u32) -> PciDeviceId {
    PciDeviceId {
        vendor: PCI_VENDOR_ID_INTEL,
        device: device_id,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0x0,
        class_mask: 0x0,
        driver_data: 0x0,
    }
}

pub const BAR_0: u32 = 0;
pub const BAR_1: u32 = 1;
pub const BAR_5: u32 = 5;

pub const VLAN_N_VID: u32 = 4096;

pub const E1000_MAX_INTR: u32 = 10;

/// Count for polling __E1000_RESET condition every 10-20msec.
pub const E1000_CHECK_RESET_COUNT: u32 = 50;

// TX/RX descriptor defines
pub const E1000_DEFAULT_TXD: u32 = 256;
pub const E1000_MAX_TXD: u32 = 256;
pub const E1000_MIN_TXD: u32 = 48;
pub const E1000_MAX_82544_TXD: u32 = 4096;

pub const E1000_DEFAULT_RXD: u32 = 256;
pub const E1000_MAX_RXD: u32 = 256;
pub const E1000_MIN_RXD: u32 = 48;
pub const E1000_MAX_82544_RXD: u32 = 4096;

/// 100000 irq/sec
pub const E1000_MIN_ITR_USECS: u32 = 10;
/// 100 irq/sec
pub const E1000_MAX_ITR_USECS: u32 = 10000;
/// this is the size past which hardware will drop packets when setting LPE=0
pub const MAXIMUM_ETHERNET_VLAN_SIZE: u32 = 1522;

// Supported Rx Buffer Sizes
/// Used for packet split
pub const E1000_RXBUFFER_128: u32 = 128;
/// Used for packet split
pub const E1000_RXBUFFER_256: u32 = 256;
pub const E1000_RXBUFFER_512: u32 = 512;
pub const E1000_RXBUFFER_1024: u32 = 1024;
pub const E1000_RXBUFFER_2048: u32 = 2048;
pub const E1000_RXBUFFER_4096: u32 = 4096;
pub const E1000_RXBUFFER_8192: u32 = 8192;
pub const E1000_RXBUFFER_16384: u32 = 16384;

// SmartSpeed delimiters
pub const E1000_SMARTSPEED_DOWNSHIFT: u32 = 3;
pub const E1000_SMARTSPEED_MAX: u32 = 15;

// Packet Buffer allocations
pub const E1000_PBA_BYTES_SHIFT: u32 = 0xA;
pub const E1000_TX_HEAD_ADDR_SHIFT: u32 = 7;
pub const E1000_PBA_TX_MASK: u32 = 0xFFFF0000;

// Flow Control Watermarks
/// High: 5688 bytes below Rx FIFO size
pub const E1000_FC_HIGH_DIFF: u32 = 0x1638;
/// Low:  5696 bytes below Rx FIFO size
pub const E1000_FC_LOW_DIFF: u32 = 0x1640;

/// pause for the max or until send xon
pub const E1000_FC_PAUSE_TIME: u32 = 0xFFFF;

pub const E1000_TX_QUEUE_WAKE: u32 = 16;
pub const E1000_RX_BUFFER_WRITE: u32 = 16;
pub const AUTO_ALL_MODES: u32 = 0;
pub const E1000_EEPROM_82544_APM: u32 = 0x0004;
pub const E1000_EEPROM_APME: u32 = 0x0400;
pub const E1000_MNG_VLAN_NONE: i32 = -1;

pub type DmaAddrT = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SkBuff {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Page {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetDevice {
    pub _address: u8,
}

pub const VLAN_GROUP_ARRAY_LEN: u32 = 4096;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VlanGroup {
    pub real_dev_ifindex: i32,
    pub vlan_devices: [*mut NetDevice; 4096usize],
    pub next: *mut VlanGroup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NapiStruct {
    pub poll_list: list_head,
    pub state: u64,
    pub weight: i32,
    pub defer_hard_irqs_count: i32,
    pub gro_bitmask: u64,
    pub poll: Option<unsafe extern "C" fn(arg1: *mut NapiStruct, arg2: i32) -> i32>,
    pub dev: *mut NetDevice,
    pub gro_hash: [gro_list; 8usize],
    pub skb: *mut SkBuff,
    pub rx_list: list_head,
    pub rx_count: i32,
    pub timer: hrtimer,
    pub dev_list: list_head,
    pub napi_hash_node: hlist_node,
    pub napi_id: u32,
}

/// wrapper around a pointer to a socket buffer,
/// so a DMA handle can be stored along with the buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000Buffer {
    pub skb: *mut SkBuff,
    pub dma: DmaAddrT,
    pub page: *mut Page,
    pub time_stamp: u64,
    pub length: u16,
    pub next_to_watch: u16,
    pub segs: u32,
    pub bytecount: u32,
    pub mapped_as_page: u16,
}

#[derive(Copy, Clone, Debug)]
struct ArrayDesc<T, const N: usize> {
    value: [T; N],
}

#[repr(C)]
#[derive(Debug)]
pub struct E1000Ring<'a> {
    pub adapter: &'a mut E1000Adapter<'a>,
    pub desc: *mut c_void,
    pub dma: DmaAddrT,
    pub size: u32,
    pub count: u32,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub head: *mut c_void,
    pub tail: *mut c_void,
    pub buffer_info: *mut E1000Buffer,
    pub name: [i8; 261usize],
    pub ims_val: u16,
    pub itr_val: u16,
    pub itr_register: *mut c_void,
    pub set_itr: i32,
    pub rx_skb_top: *mut SkBuff,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct E1000TxRing {
    pub desc: Box<Vec<E1000TxDesc>>,
    pub dma: DmaAddrT,
    pub size: u32,
    pub count: u32,
    pub next_to_use: u32,
    pub next_to_clean: u32,
    pub buffer_info: *mut E1000Buffer,
    pub tdh: u16,
    pub tdt: u16,
    pub last_tx_tso: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct E1000RxRing {
    pub desc: Box<Vec<E1000TxDesc>>,
    pub dma: DmaAddrT,
    pub size: u32,
    pub count: u32,
    pub next_to_use: u32,
    pub next_to_clean: u32,
    pub buffer_info: *mut E1000Buffer,
    pub rx_skb_top: *mut SkBuff,
    pub cpu: i32,
    pub rdh: u16,
    pub rdt: u16,
}

trait TxRing {
    fn nextup(&self) -> u32;
    fn get_desc<T>(&self, index: u32) -> T;
}

fn nextup(next_to_clean: u32, next_to_use: u32, count: u32) -> u32 {
    let val = if next_to_clean > next_to_use {
        0
    } else {
        count
    };
    val + (next_to_clean - next_to_use - 1)
}

// impl<N> TxRing for E1000TxRing<N> {
//     fn nextup(&self) -> u32 {
//         nextup(self.next_to_clean, self.next_to_use, self.count)
//     }
//     fn get_desc<E1000TxDesc>(&self, index: u32) -> &mut E1000TxDesc {
//         self.desc.value[index]
//     }
// }

// impl<N> TxRing for E1000RxRing<N> {
//     fn nextup(&self) -> u32 {
//         nextup(self.next_to_clean, self.next_to_use, self.count)
//     }
//
//     fn get_desc<E1000RxDesc>(&self, index: u32) -> &mut E1000RxDesc {
//         self.desc.value[index]
//     }
// }

// static bool e1000_clean_jumbo_rx_irq(struct e1000_adapter *adapter,
// struct e1000_rx_ring *rx_ring,
// int *work_done, int work_to_do)

pub trait PolFnTr: Fn(&E1000Adapter, &E1000RxRing, i32, i32) -> bool {}
impl<F> PolFnTr for F where F: Fn(&E1000Adapter, &E1000RxRing, i32, i32) -> bool {}
pub type CleanRxFn = dyn PolFnTr<Output = bool>;

impl fmt::Debug for CleanRxFn {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("foo").finish()
    }
}

pub trait AllocFnTr: Fn(&E1000Adapter, &E1000RxRing, i32) {}
impl<F> AllocFnTr for F where F: Fn(&E1000Adapter, &E1000RxRing, i32) {}
pub type AllocFn = dyn AllocFnTr<Output = ()>;

impl fmt::Debug for AllocFn {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("foo").finish()
    }
}

#[repr(C)]
#[derive(Debug)]
struct e1000_desc_ring {
    /* pointer to the descriptor ring memory */
    desc: c_void,
    /* physical address of the descriptor ring */
    dma: DmaAddrT,
    /* length of descriptor ring in bytes */
    size: u32,
    /* number of descriptors in the ring */
    count: u32,
    /* (atomic) number of desc with no buffer */
    unused: atomic_t,
    /* number of desc with no buffer */
    unused_count: u32,
    /* next descriptor to associate a buffer with */
    next_to_use: u32,
    /* next descriptor to check for DD status bit */
    next_to_clean: u32,
    /* array of buffer information structs */
    buffer_info: E1000Buffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetDeviceStats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub multicast: u64,
    pub collisions: u64,
    pub rx_length_errors: u64,
    pub rx_over_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_fifo_errors: u64,
    pub rx_missed_errors: u64,
    pub tx_aborted_errors: u64,
    pub tx_carrier_errors: u64,
    pub tx_fifo_errors: u64,
    pub tx_heartbeat_errors: u64,
    pub tx_window_errors: u64,
    pub rx_compressed: u64,
    pub tx_compressed: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct E1000Adapter<'a> {
    watchdog_timer: timer_list,
    phy_info_timer: timer_list,
    vlgrp: &'a VlanGroup,
    id_string: &'a mut c_void,
    bd_number: u32,
    rx_buffer_len: u32,
    part_num: u32,
    wol: u32,
    link_speed: u16,
    link_duplex: u16,
    stats_lock: spinlock_t,
    irq_sem: atomic_t,
    tx_timeout_task: work_struct,

    blink_timer: timer_list,
    led_status: u64,

    /* TX */
    tx_ring: e1000_desc_ring,
    txd_cmd: u32,
    tx_int_delay: u32,
    tx_abs_int_delay: u32,
    max_data_per_txd: i32,

    /* RX */
    rx_ring: e1000_desc_ring,
    hw_csum_err: u64,
    hw_csum_good: u64,
    rx_int_delay: u32,
    rx_abs_int_delay: u32,
    rx_csum: BooleanT,

    /* OS defined structs */
    netdev: NetDevice,
    pdev: PciDev,
    net_stats: NetDeviceStats,

    /* structs defined in e1000_hw.h */
    hw: E1000Hw<'a>,
    stats: E1000HwStats,
    phy_info: E1000PhyInfo,
    phy_stats: E1000PhyStats,
    pci_state: [u32; 16],
    ifname: [u8; IFNAMSIZ],
}

pub type E1000StateT = u32;
pub const E1000_STATE_T_E1000_TESTING: E1000StateT = 0;
pub const E1000_STATE_T_E1000_RESETTING: E1000StateT = 1;
pub const E1000_STATE_T_E1000_DOWN: E1000StateT = 2;
