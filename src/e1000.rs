

pub const PCI_VENDOR_ID_INTEL:u32 = 0x8086;
pub const PCI_ANY_ID:u32 = !0;

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

fn INTEL_E1000_ETHERNET_DEVICE(device_id:u32) -> pci_device_id {
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: device_id, subvendor:PCI_ANY_ID, subdevice: PCI_ANY_ID, ..Default::default()}
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

pub type hrtimer_restart = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct atomic_t {
    pub counter: ::std::os::raw::c_int,
}

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
pub struct workqueue_struct {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gro_list {
    pub list: list_head,
    pub count: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_device {
    pub _address: u8,
}

pub type ktime_t = i64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: ::std::os::raw::c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qspinlock__bindgen_ty_1__bindgen_ty_1 {
    pub locked: u8,
    pub pending: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qspinlock__bindgen_ty_1__bindgen_ty_2 {
    pub locked_pending: u16,
    pub tail: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qspinlock__bindgen_ty_1 {
    pub val: atomic_t,
    pub __bindgen_anon_1: qspinlock__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: qspinlock__bindgen_ty_1__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timerqueue_node {
    pub node: rb_node,
    pub expires: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qspinlock {
    pub __bindgen_anon_1: qspinlock__bindgen_ty_1,
}

pub type arch_spinlock_t = qspinlock;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_spinlock {
    pub raw_lock: arch_spinlock_t,
}
pub type raw_spinlock_t = raw_spinlock;


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer_cpu_base {
    pub lock: raw_spinlock_t,
    pub cpu: ::std::os::raw::c_uint,
    pub active_bases: ::std::os::raw::c_uint,
    pub clock_was_set_seq: ::std::os::raw::c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub expires_next: ktime_t,
    pub next_timer: *mut hrtimer,
    pub softirq_expires_next: ktime_t,
    pub softirq_next_timer: *mut hrtimer,
    pub clock_base: [hrtimer_clock_base; 8usize],
}

pub type __kernel_clockid_t = i32;
pub type clockid_t = __kernel_clockid_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seqcount {
    pub sequence: ::std::os::raw::c_uint,
}

pub type seqcount_t = seqcount;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timerqueue_head {
    pub head: rb_root,
    pub next: *mut timerqueue_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hrtimer_clock_base {
    pub cpu_base: *mut hrtimer_cpu_base,
    pub index: ::std::os::raw::c_uint,
    pub clockid: clockid_t,
    pub seq: seqcount_t,
    pub running: *mut hrtimer,
    pub active: timerqueue_head,
    pub get_time: ::std::option::Option<unsafe extern "C" fn() -> ktime_t>,
    pub offset: ktime_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hrtimer {
    pub node: timerqueue_node,
    pub _softexpires: ktime_t,
    pub function:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut hrtimer) -> hrtimer_restart>,
    pub base: *mut hrtimer_clock_base,
    pub state: u8,
    pub is_rel: u8,
    pub is_soft: u8,
    pub is_hard: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct spinlock_t {
    pub magic: ::std::os::raw::c_ulong,
    pub lock: ::std::os::raw::c_ulong,
    pub babble: ::std::os::raw::c_uint,
    pub module: *const ::std::os::raw::c_char,
    pub owner: *mut ::std::os::raw::c_char,
    pub oline: ::std::os::raw::c_int,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
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
pub struct e1000_hw_stats {
    pub crcerrs: u64,
    pub algnerrc: u64,
    pub symerrs: u64,
    pub rxerrc: u64,
    pub txerrc: u64,
    pub mpc: u64,
    pub scc: u64,
    pub ecol: u64,
    pub mcc: u64,
    pub latecol: u64,
    pub colc: u64,
    pub dc: u64,
    pub tncrs: u64,
    pub sec: u64,
    pub cexterr: u64,
    pub rlec: u64,
    pub xonrxc: u64,
    pub xontxc: u64,
    pub xoffrxc: u64,
    pub xofftxc: u64,
    pub fcruc: u64,
    pub prc64: u64,
    pub prc127: u64,
    pub prc255: u64,
    pub prc511: u64,
    pub prc1023: u64,
    pub prc1522: u64,
    pub gprc: u64,
    pub bprc: u64,
    pub mprc: u64,
    pub gptc: u64,
    pub gorcl: u64,
    pub gorch: u64,
    pub gotcl: u64,
    pub gotch: u64,
    pub rnbc: u64,
    pub ruc: u64,
    pub rfc: u64,
    pub roc: u64,
    pub rlerrc: u64,
    pub rjc: u64,
    pub mgprc: u64,
    pub mgpdc: u64,
    pub mgptc: u64,
    pub torl: u64,
    pub torh: u64,
    pub totl: u64,
    pub toth: u64,
    pub tpr: u64,
    pub tpt: u64,
    pub ptc64: u64,
    pub ptc127: u64,
    pub ptc255: u64,
    pub ptc511: u64,
    pub ptc1023: u64,
    pub ptc1522: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub tsctc: u64,
    pub tsctfc: u64,
    pub iac: u64,
    pub icrxptc: u64,
    pub icrxatc: u64,
    pub ictxptc: u64,
    pub ictxatc: u64,
    pub ictxqec: u64,
    pub ictxqmtc: u64,
    pub icrxdmtc: u64,
    pub icrxoc: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_hw {
    pub hw_addr: *mut u8,
    pub flash_address: *mut u8,
    pub ce4100_gbe_mdio_base_virt: *mut ::std::os::raw::c_void,
    pub mac_type: e1000_mac_type,
    pub phy_type: e1000_phy_type,
    pub phy_init_script: u32,
    pub media_type: e1000_media_type,
    pub back: *mut ::std::os::raw::c_void,
    pub eeprom_shadow_ram: *mut e1000_shadow_ram,
    pub flash_bank_size: u32,
    pub flash_base_addr: u32,
    pub fc: e1000_fc_type,
    pub bus_speed: e1000_bus_speed,
    pub bus_width: e1000_bus_width,
    pub bus_type: e1000_bus_type,
    pub eeprom: e1000_eeprom_info,
    pub master_slave: e1000_ms_type,
    pub original_master_slave: e1000_ms_type,
    pub ffe_config_state: e1000_ffe_config,
    pub asf_firmware_present: u32,
    pub eeprom_semaphore_present: u32,
    pub io_base: ::std::os::raw::c_ulong,
    pub phy_id: u32,
    pub phy_revision: u32,
    pub phy_addr: u32,
    pub original_fc: u32,
    pub txcw: u32,
    pub autoneg_failed: u32,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
    pub mc_filter_type: u32,
    pub num_mc_addrs: u32,
    pub collision_delta: u32,
    pub tx_packet_delta: u32,
    pub ledctl_default: u32,
    pub ledctl_mode1: u32,
    pub ledctl_mode2: u32,
    pub tx_pkt_filtering: bool,
    pub mng_cookie: e1000_host_mng_dhcp_cookie,
    pub phy_spd_default: u16,
    pub autoneg_advertised: u16,
    pub pci_cmd_word: u16,
    pub fc_high_water: u16,
    pub fc_low_water: u16,
    pub fc_pause_time: u16,
    pub current_ifs_val: u16,
    pub ifs_min_val: u16,
    pub ifs_max_val: u16,
    pub ifs_step_size: u16,
    pub ifs_ratio: u16,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub autoneg: u8,
    pub mdix: u8,
    pub forced_speed_duplex: u8,
    pub wait_autoneg_complete: u8,
    pub dma_fairness: u8,
    pub mac_addr: [u8; 6usize],
    pub perm_mac_addr: [u8; 6usize],
    pub disable_polarity_correction: bool,
    pub speed_downgraded: bool,
    pub smart_speed: e1000_smart_speed,
    pub dsp_config_state: e1000_dsp_config,
    pub get_link_status: bool,
    pub serdes_has_link: bool,
    pub tbi_compatibility_en: bool,
    pub tbi_compatibility_on: bool,
    pub laa_is_present: bool,
    pub phy_reset_disable: bool,
    pub initialize_hw_bits_disable: bool,
    pub fc_send_xon: bool,
    pub fc_strict_ieee: bool,
    pub report_tx_early: bool,
    pub adaptive_ifs: bool,
    pub ifs_params_forced: bool,
    pub in_ifs_mode: bool,
    pub mng_reg_access_disabled: bool,
    pub leave_av_bit_off: bool,
    pub bad_tx_carr_stats_fd: bool,
    pub has_smbus: bool,
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