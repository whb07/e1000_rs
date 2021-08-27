use crate::deps::timer::{
    atomic_t, delayed_work, gro_list, hlist_node, hrtimer, list_head, mutex, spinlock_t,
    work_struct,
};
use crate::e1000_hw::{
    e1000_1000t_rx_status, e1000_10bt_ext_dist_enable, e1000_auto_x_mode, e1000_cable_length,
    e1000_downshift, e1000_hw, e1000_hw_stats, e1000_phy_stats, e1000_polarity_reversal,
    e1000_rev_polarity,
};

pub const PCI_VENDOR_ID_INTEL: u32 = 0x8086;
pub const PCI_ANY_ID: u32 = !0;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: u64,
}

fn INTEL_E1000_ETHERNET_DEVICE(device_id: u32) -> pci_device_id {
    pci_device_id {
        vendor: PCI_VENDOR_ID_INTEL,
        device: device_id,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        ..Default::default()
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

pub type dma_addr_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sk_buff {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct page {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pci_dev {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_device {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_struct {
    pub poll_list: list_head,
    pub state: ::std::os::raw::c_ulong,
    pub weight: ::std::os::raw::c_int,
    pub defer_hard_irqs_count: ::std::os::raw::c_int,
    pub gro_bitmask: ::std::os::raw::c_ulong,
    pub poll: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut napi_struct,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub dev: *mut net_device,
    pub gro_hash: [gro_list; 8usize],
    pub skb: *mut sk_buff,
    pub rx_list: list_head,
    pub rx_count: ::std::os::raw::c_int,
    pub timer: hrtimer,
    pub dev_list: list_head,
    pub napi_hash_node: hlist_node,
    pub napi_id: ::std::os::raw::c_uint,
}

/// wrapper around a pointer to a socket buffer,
/// so a DMA handle can be stored along with the buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub time_stamp: u64,
    pub length: u16,
    pub next_to_watch: u16,
    pub segs: u32,
    pub bytecount: u32,
    pub mapped_as_page: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_tx_ring {
    pub desc: *mut ::std::os::raw::c_void,
    pub dma: dma_addr_t,
    pub size: u32,
    pub count: u32,
    pub next_to_use: u32,
    pub next_to_clean: u32,
    pub buffer_info: *mut e1000_buffer,
    pub tdh: u16,
    pub tdt: u16,
    pub last_tx_tso: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_rx_ring {
    pub desc: *mut ::std::os::raw::c_void,
    pub dma: dma_addr_t,
    pub size: u32,
    pub count: u32,
    pub next_to_use: u32,
    pub next_to_clean: u32,
    pub buffer_info: *mut e1000_buffer,
    pub rx_skb_top: *mut sk_buff,
    pub cpu: i32,
    pub rdh: u16,
    pub rdt: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_adapter {
    pub active_vlans: [::std::os::raw::c_ulong; 64usize],
    pub mng_vlan_id: u16,
    pub bd_number: u32,
    pub rx_buffer_len: u32,
    pub wol: u32,
    pub smartspeed: u32,
    pub en_mng_pt: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub stats_lock: spinlock_t,
    pub total_tx_bytes: u32,
    pub total_tx_packets: u32,
    pub total_rx_bytes: u32,
    pub total_rx_packets: u32,
    pub itr: u32,
    pub itr_setting: u32,
    pub tx_itr: u16,
    pub rx_itr: u16,
    pub fc_autoneg: u8,
    pub tx_ring: *mut e1000_tx_ring,
    pub restart_queue: u32,
    pub txd_cmd: u32,
    pub tx_int_delay: u32,
    pub tx_abs_int_delay: u32,
    pub gotcl: u32,
    pub gotcl_old: u64,
    pub tpt_old: u64,
    pub colc_old: u64,
    pub tx_timeout_count: u32,
    pub tx_fifo_head: u32,
    pub tx_head_addr: u32,
    pub tx_fifo_size: u32,
    pub tx_timeout_factor: u8,
    pub tx_fifo_stall: atomic_t,
    pub pcix_82544: bool,
    pub detect_tx_hung: bool,
    pub dump_buffers: bool,
    pub clean_rx: ::std::option::Option<
        unsafe extern "C" fn(
            adapter: *mut e1000_adapter,
            rx_ring: *mut e1000_rx_ring,
            work_done: *mut i32,
            work_to_do: i32,
        ) -> bool,
    >,
    pub alloc_rx_buf: ::std::option::Option<
        unsafe extern "C" fn(
            adapter: *mut e1000_adapter,
            rx_ring: *mut e1000_rx_ring,
            cleaned_count: i32,
        ),
    >,
    pub rx_ring: *mut e1000_rx_ring,
    pub napi: napi_struct,
    pub num_tx_queues: i32,
    pub num_rx_queues: i32,
    pub hw_csum_err: u64,
    pub hw_csum_good: u64,
    pub alloc_rx_buff_failed: u32,
    pub rx_int_delay: u32,
    pub rx_abs_int_delay: u32,
    pub rx_csum: bool,
    pub gorcl: u32,
    pub gorcl_old: u64,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub hw: e1000_hw,
    pub stats: e1000_hw_stats,
    pub phy_info: e1000_phy_info,
    pub phy_stats: e1000_phy_stats,
    pub test_icr: u32,
    pub test_tx_ring: e1000_tx_ring,
    pub test_rx_ring: e1000_rx_ring,
    pub msg_enable: i32,
    pub tso_force: bool,
    pub smart_power_down: bool,
    pub quad_port_a: bool,
    pub flags: ::std::os::raw::c_ulong,
    pub eeprom_wol: u32,
    pub bars: i32,
    pub need_ioport: i32,
    pub discarding: bool,
    pub reset_task: work_struct,
    pub __bindgen_padding_0: [u64; 4usize],
    pub watchdog_task: delayed_work,
    pub fifo_stall_task: delayed_work,
    pub phy_info_task: delayed_work,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_phy_info {
    pub cable_length: e1000_cable_length,
    pub extended_10bt_distance: e1000_10bt_ext_dist_enable,
    pub cable_polarity: e1000_rev_polarity,
    pub downshift: e1000_downshift,
    pub polarity_correction: e1000_polarity_reversal,
    pub mdix_mode: e1000_auto_x_mode,
    pub local_rx: e1000_1000t_rx_status,
    pub remote_rx: e1000_1000t_rx_status,
}
