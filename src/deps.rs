use crate::deps::timer::{atomic_t, spinlock_t, wait_queue_head_t, timer_list, hlist_node};
use crate::lib::c_void;

pub mod pci;
pub mod timer;

pub type mode_t = u16;
pub type ssize_t = i32;
pub type size_t = u32;
pub type dev_t = u32;
pub type phys_addr_t = u32;
pub type resource_size_t = phys_addr_t;

pub const NR_IRQS: u32 = 224;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct klist {
    pub k_lock: spinlock_t,
    pub k_list: list_head,
    pub get: ::std::option::Option<unsafe extern "C" fn(arg1: *mut klist_node)>,
    pub put: ::std::option::Option<unsafe extern "C" fn(arg1: *mut klist_node)>,
}
extern "C" {
    pub fn klist_init(
        k: *mut klist,
        get: ::std::option::Option<unsafe extern "C" fn(arg1: *mut klist_node)>,
        put: ::std::option::Option<unsafe extern "C" fn(arg1: *mut klist_node)>,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dentry {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kref {
    pub refcount: atomic_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct completion {
    pub done: u32,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct klist_node {
    pub n_klist: *mut klist,
    pub n_node: list_head,
    pub n_ref: kref,
    pub n_removed: completion,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobj_type {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobject {
    pub k_name: *const ::std::os::raw::c_char,
    pub name: [::std::os::raw::c_char; 20usize],
    pub kref: kref,
    pub entry: list_head,
    pub parent: *mut kobject,
    pub kset: *mut kset,
    pub ktype: *mut kobj_type,
    pub dentry: *mut dentry,
    pub poll: wait_queue_head_t,
}

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
#[derive(Debug, Copy, Clone)]
pub struct semaphore {
    pub count: atomic_t,
    pub sleepers: i32,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct module {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kset_uevent_ops {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class_attribute {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class_device_attribute {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class_device {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct attribute_group {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct attribute {
    pub name: *const ::std::os::raw::c_char,
    pub owner: *mut module,
    pub mode: mode_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device_attribute {
    pub attr: attribute,
    pub show: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut device,
            attr: *mut device_attribute,
            buf: *mut ::std::os::raw::c_char,
        ) -> ssize_t,
    >,
    pub store: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut device,
            attr: *mut device_attribute,
            buf: *const ::std::os::raw::c_char,
            count: size_t,
        ) -> ssize_t,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bus_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device_driver {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pm_message {
    pub event: i32,
}
pub type pm_message_t = pm_message;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dev_pm_info {
    pub power_state: pm_message_t,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}

impl dev_pm_info {
    #[inline]
    pub fn can_wakeup(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_can_wakeup(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(can_wakeup: u32) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let can_wakeup: u32 = unsafe { ::std::mem::transmute(can_wakeup) };
            can_wakeup as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dma_coherent_mem {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dev_archdata {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device<'a> {
    pub klist_children: klist,
    pub knode_parent: klist_node,
    pub knode_driver: klist_node,
    pub knode_bus: klist_node,
    pub parent: *mut device,
    pub kobj: kobject,
    pub bus_id: [::std::os::raw::c_char; 20usize],
    pub type_: *mut device_type,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub uevent_attr: device_attribute,
    pub devt_attr: *mut device_attribute,
    pub sem: semaphore,
    pub bus: *mut bus_type,
    pub driver: *mut device_driver,
    pub driver_data: &'a NetDevice,
    pub platform_data: *mut ::std::os::raw::c_void,
    pub power: dev_pm_info,
    pub dma_mask: *mut u64,
    pub coherent_dma_mask: u64,
    pub dma_pools: list_head,
    pub dma_mem: *mut dma_coherent_mem,
    pub archdata: dev_archdata,
    pub devres_lock: spinlock_t,
    pub devres_head: list_head,
    pub node: list_head,
    pub class: *mut class,
    pub devt: dev_t,
    pub groups: *mut *mut attribute_group,
    pub release: ::std::option::Option<unsafe extern "C" fn(dev: *mut device)>,
}
impl device {
    #[inline]
    pub fn is_registered(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_is_registered(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn uevent_suppress(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_uevent_suppress(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        is_registered: u32,
        uevent_suppress: u32,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let is_registered: u32 = unsafe { ::std::mem::transmute(is_registered) };
            is_registered as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let uevent_suppress: u32 = unsafe { ::std::mem::transmute(uevent_suppress) };
            uevent_suppress as u64
        });
        __bindgen_bitfield_unit
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_ulong,
    pub parent: *mut resource,
    pub sibling: *mut resource,
    pub child: *mut resource,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class {
    pub name: *const ::std::os::raw::c_char,
    pub owner: *mut module,
    pub subsys: kset,
    pub children: list_head,
    pub devices: list_head,
    pub interfaces: list_head,
    pub class_dirs: kset,
    pub sem: semaphore,
    pub class_attrs: *mut class_attribute,
    pub class_dev_attrs: *mut class_device_attribute,
    pub dev_attrs: *mut device_attribute,
    pub uevent: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut class_device,
            envp: *mut *mut ::std::os::raw::c_char,
            num_envp: i32,
            buffer: *mut ::std::os::raw::c_char,
            buffer_size: i32,
        ) -> i32,
    >,
    pub dev_uevent: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut device,
            envp: *mut *mut ::std::os::raw::c_char,
            num_envp: i32,
            buffer: *mut ::std::os::raw::c_char,
            buffer_size: i32,
        ) -> i32,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(dev: *mut class_device)>,
    pub class_release: ::std::option::Option<unsafe extern "C" fn(class: *mut class)>,
    pub dev_release: ::std::option::Option<unsafe extern "C" fn(dev: *mut device)>,
    pub suspend:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut device, state: pm_message_t) -> i32>,
    pub resume: ::std::option::Option<unsafe extern "C" fn(arg1: *mut device) -> i32>,
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
#[derive(Debug, Copy, Clone)]
pub struct ethtool_ops {
    pub _address: u8,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dev_mc_list {
    pub _address: u8,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wireless_dev {
    pub _address: u8,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Qdisc {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SkBuff {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ifreq {
    _unused: [u8; 0],
}

pub type net_device__bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ifmap {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct neighbour {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hh_cache {
    _unused: [u8; 0],
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vlan_group {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct neigh_parms {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_bridge_port {
    pub _address: u8,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetDevice {
    pub name: [i8; 16usize],
    pub name_hlist: hlist_node,
    pub mem_end: u64,
    pub mem_start: u64,
    pub base_addr: u64,
    pub irq: u32,
    pub if_port: u8,
    pub dma: u8,
    pub state: u64,
    pub dev_list: list_head,
    pub init:
    ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice) -> i32>,
    pub features: u64,
    pub next_sched: *mut NetDevice,
    pub ifindex: i32,
    pub iflink: i32,
    pub get_stats:
    ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice) -> *mut NetDeviceStats>,
    pub stats: NetDeviceStats,
    pub ethtool_ops: *mut ethtool_ops,
    pub flags: u32,
    pub gflags: u16,
    pub priv_flags: u16,
    pub padded: u16,
    pub operstate: u8,
    pub link_mode: u8,
    pub mtu: u32,
    pub type_: u16,
    pub hard_header_len: u16,
    pub master: *mut NetDevice,
    pub perm_addr: [u8; 32usize],
    pub addr_len: u8,
    pub dev_id: u16,
    pub mc_list: *mut dev_mc_list,
    pub mc_count: i32,
    pub promiscuity: i32,
    pub allmulti: i32,
    pub atalk_ptr: *mut ::std::os::raw::c_void,
    pub ip_ptr: *mut ::std::os::raw::c_void,
    pub dn_ptr: *mut ::std::os::raw::c_void,
    pub ip6_ptr: *mut ::std::os::raw::c_void,
    pub ec_ptr: *mut ::std::os::raw::c_void,
    pub ax25_ptr: *mut ::std::os::raw::c_void,
    pub ieee80211_ptr: *mut wireless_dev,
    pub poll_list: list_head,
    pub poll: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut NetDevice,
            quota: *mut i32,
        ) -> i32,
    >,
    pub quota: i32,
    pub weight: i32,
    pub last_rx: u64,
    pub dev_addr: [u8; 32usize],
    pub broadcast: [u8; 32usize],
    pub queue_lock: spinlock_t,
    pub qdisc: *mut Qdisc,
    pub qdisc_sleeping: *mut Qdisc,
    pub qdisc_list: list_head,
    pub tx_queue_len: u64,
    pub gso_skb: *mut SkBuff,
    pub ingress_lock: spinlock_t,
    pub qdisc_ingress: *mut Qdisc,
    pub _xmit_lock: spinlock_t,
    pub xmit_lock_owner: i32,
    pub priv_: *mut ::std::os::raw::c_void,
    pub hard_start_xmit: ::std::option::Option<
        unsafe extern "C" fn(skb: *mut SkBuff, dev: *mut NetDevice) -> i32,
    >,
    pub trans_start: u64,
    pub watchdog_timeo: i32,
    pub watchdog_timer: timer_list,
    pub refcnt: atomic_t,
    pub todo_list: list_head,
    pub index_hlist: hlist_node,
    pub link_watch_next: *mut NetDevice,
    pub reg_state: net_device__bindgen_ty_1,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice)>,
    pub destructor: ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice)>,
    pub open:
    ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice) -> i32>,
    pub stop:
    ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice) -> i32>,
    pub hard_header: ::std::option::Option<
        unsafe extern "C" fn(
            skb: *mut SkBuff,
            dev: *mut NetDevice,
            type_: u16,
            daddr: *mut ::std::os::raw::c_void,
            saddr: *mut ::std::os::raw::c_void,
            len: u32,
        ) -> i32,
    >,
    pub rebuild_header:
    ::std::option::Option<unsafe extern "C" fn(skb: *mut SkBuff) -> i32>,
    pub set_multicast_list: ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice)>,
    pub set_mac_address: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut NetDevice,
            addr: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub do_ioctl: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut NetDevice,
            ifr: *mut ifreq,
            cmd: i32,
        ) -> i32,
    >,
    pub set_config: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut NetDevice, map: *mut ifmap) -> i32,
    >,
    pub hard_header_cache: ::std::option::Option<
        unsafe extern "C" fn(neigh: *mut neighbour, hh: *mut hh_cache) -> i32,
    >,
    pub header_cache_update: ::std::option::Option<
        unsafe extern "C" fn(
            hh: *mut hh_cache,
            dev: *mut NetDevice,
            haddr: *mut u8,
        ),
    >,
    pub change_mtu: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut NetDevice,
            new_mtu: i32,
        ) -> i32,
    >,
    pub tx_timeout: ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice)>,
    pub vlan_rx_register:
    ::std::option::Option<unsafe extern "C" fn(dev: *mut NetDevice, grp: *mut vlan_group)>,
    pub vlan_rx_add_vid: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut NetDevice, vid: u16),
    >,
    pub vlan_rx_kill_vid: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut NetDevice, vid: u16),
    >,
    pub hard_header_parse: ::std::option::Option<
        unsafe extern "C" fn(
            skb: *mut SkBuff,
            haddr: *mut u8,
        ) -> i32,
    >,
    pub neigh_setup: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut NetDevice, arg1: *mut neigh_parms) -> i32,
    >,
    pub br_port: *mut net_bridge_port,
    pub dev: device,
    pub sysfs_groups: [*mut attribute_group; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kset {
    pub ktype: *mut kobj_type,
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub kobj: kobject,
    pub uevent_ops: *mut kset_uevent_ops,
}
