/* automatically generated by rust-bindgen 0.59.1 */

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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 33;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const KOBJ_NAME_LEN: u32 = 20;
pub const BUS_ID_SIZE: u32 = 20;
pub const DEVICE_COUNT_COMPATIBLE: u32 = 4;
pub const DEVICE_COUNT_RESOURCE: u32 = 12;
pub const MAX_ADDR_LEN: u32 = 32;
pub const NETIF_F_SG: u32 = 1;
pub const NETIF_F_IP_CSUM: u32 = 2;
pub const NETIF_F_NO_CSUM: u32 = 4;
pub const NETIF_F_HW_CSUM: u32 = 8;
pub const NETIF_F_HIGHDMA: u32 = 32;
pub const NETIF_F_FRAGLIST: u32 = 64;
pub const NETIF_F_HW_VLAN_TX: u32 = 128;
pub const NETIF_F_HW_VLAN_RX: u32 = 256;
pub const NETIF_F_HW_VLAN_FILTER: u32 = 512;
pub const NETIF_F_VLAN_CHALLENGED: u32 = 1024;
pub const NETIF_F_GSO: u32 = 2048;
pub const NETIF_F_LLTX: u32 = 4096;
pub const NETIF_F_GSO_SHIFT: u32 = 16;
pub const NETIF_F_GSO_MASK: u32 = 4294901760;
pub const NETIF_F_GEN_CSUM: u32 = 12;
pub const NETIF_F_ALL_CSUM: u32 = 14;
pub type __u_char = u8;
pub type __u_short = u16;
pub type __u_int = u32;
pub type __u_long = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = u8;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = u64;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = u64;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __ino64_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [i32; 2usize],
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = u64;
pub type __rlim64_t = u64;
pub type __id_t = u32;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = u32;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = i32;
pub type __key_t = i32;
pub type __clockid_t = i32;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = u64;
pub type __fsblkcnt64_t = u64;
pub type __fsfilcnt_t = u64;
pub type __fsfilcnt64_t = u64;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = u64;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut i8;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = u32;
pub type __sig_atomic_t = i32;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct raw_spinlock_t {
    pub slock: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct spinlock_t {
    pub raw_lock: raw_spinlock_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wait_queue_head {
    pub lock: spinlock_t,
    pub task_list: list_head,
}
pub type wait_queue_head_t = __wait_queue_head;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct completion {
    pub done: u32,
    pub wait: wait_queue_head_t,
}
pub type pci_channel_state_t = u32;
pub type __kernel_ino_t = u64;
pub type __kernel_mode_t = u16;
pub type __kernel_nlink_t = u16;
pub type __kernel_off_t = ::std::os::raw::c_long;
pub type __kernel_pid_t = i32;
pub type __kernel_ipc_pid_t = u16;
pub type __kernel_uid_t = u16;
pub type __kernel_gid_t = u16;
pub type __kernel_size_t = u32;
pub type __kernel_ssize_t = i32;
pub type __kernel_ptrdiff_t = i32;
pub type __kernel_time_t = ::std::os::raw::c_long;
pub type __kernel_suseconds_t = ::std::os::raw::c_long;
pub type __kernel_clock_t = ::std::os::raw::c_long;
pub type __kernel_timer_t = i32;
pub type __kernel_clockid_t = i32;
pub type __kernel_daddr_t = i32;
pub type __kernel_caddr_t = *mut i8;
pub type __kernel_uid16_t = u16;
pub type __kernel_gid16_t = u16;
pub type __kernel_uid32_t = u32;
pub type __kernel_gid32_t = u32;
pub type pci_power_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct atomic_t {
    pub counter: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kref {
    pub refcount: atomic_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct task_struct {
    _unused: [u8; 0],
}
pub type u64_ = u64;
pub type s64 = i64;
pub type u32_ = u32;
pub type s32 = i32;
pub type u16_ = u16;
pub type s16 = ::std::os::raw::c_short;
pub type u8_ = u8;
pub type s8 = ::std::os::raw::c_schar;
pub type size_t = __kernel_size_t;
pub type ssize_t = __kernel_ssize_t;
pub type mode_t = __kernel_mode_t;
pub type phys_addr_t = u32_;
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
pub struct klist_node {
    pub n_klist: *mut klist,
    pub n_node: list_head,
    pub n_ref: kref,
    pub n_removed: completion,
}
extern "C" {
    pub fn klist_add_tail(n: *mut klist_node, k: *mut klist);
}
extern "C" {
    pub fn klist_add_head(n: *mut klist_node, k: *mut klist);
}
extern "C" {
    pub fn klist_del(n: *mut klist_node);
}
extern "C" {
    pub fn klist_remove(n: *mut klist_node);
}
extern "C" {
    pub fn klist_node_attached(n: *mut klist_node) -> i32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct klist_iter {
    pub i_klist: *mut klist,
    pub i_head: *mut list_head,
    pub i_cur: *mut klist_node,
}
pub type resource_size_t = phys_addr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct attribute {
    pub name: *const i8,
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
            buf: *mut i8,
        ) -> ssize_t,
    >,
    pub store: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut device,
            attr: *mut device_attribute,
            buf: *const i8,
            count: size_t,
        ) -> ssize_t,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ustat {
    pub f_tfree: __kernel_daddr_t,
    pub f_tinode: __kernel_ino_t,
    pub f_fname: [i8; 6usize],
    pub f_fpack: [i8; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobject {
    pub k_name: *const i8,
    pub name: [i8; 20usize],
    pub kref: kref,
    pub entry: list_head,
    pub parent: *mut kobject,
    pub kset: *mut kset,
    pub ktype: *mut kobj_type,
    pub dentry: *mut dentry,
    pub poll: wait_queue_head_t,
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
    pub fn new_bitfield_1(
        can_wakeup: u32,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
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
pub struct dev_archdata {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kset {
    pub ktype: *mut kobj_type,
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub kobj: kobject,
    pub uevent_ops: *mut kset_uevent_ops,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [u64; 16usize],
}
pub type __kernel_dev_t = u32;
pub type fd_set = __kernel_fd_set;
pub type dev_t = __kernel_dev_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class {
    pub name: *const i8,
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
            envp: *mut *mut i8,
            num_envp: i32,
            buffer: *mut i8,
            buffer_size: i32,
        ) -> i32,
    >,
    pub dev_uevent: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut device,
            envp: *mut *mut i8,
            num_envp: i32,
            buffer: *mut i8,
            buffer_size: i32,
        ) -> i32,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(dev: *mut class_device)>,
    pub class_release: ::std::option::Option<unsafe extern "C" fn(class: *mut class)>,
    pub dev_release: ::std::option::Option<unsafe extern "C" fn(dev: *mut device)>,
    pub suspend: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut device, state: pm_message_t) -> i32,
    >,
    pub resume:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut device) -> i32>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device {
    pub klist_children: klist,
    pub knode_parent: klist_node,
    pub knode_driver: klist_node,
    pub knode_bus: klist_node,
    pub parent: *mut device,
    pub kobj: kobject,
    pub bus_id: [i8; 20usize],
    pub type_: *mut device_type,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub uevent_attr: device_attribute,
    pub devt_attr: *mut device_attribute,
    pub sem: semaphore,
    pub bus: *mut bus_type,
    pub driver: *mut device_driver,
    pub driver_data: *mut ::std::os::raw::c_void,
    pub platform_data: *mut ::std::os::raw::c_void,
    pub power: dev_pm_info,
    pub dma_mask: *mut u64_,
    pub coherent_dma_mask: u64_,
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
    pub name: *const i8,
    pub flags: u64,
    pub parent: *mut resource,
    pub sibling: *mut resource,
    pub child: *mut resource,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct resource_list {
    pub next: *mut resource_list,
    pub res: *mut resource,
    pub dev: *mut pci_dev,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pci_dev {
    pub global_list: list_head,
    pub bus_list: list_head,
    pub bus: *mut pci_bus,
    pub subordinate: *mut pci_bus,
    pub sysdata: *mut ::std::os::raw::c_void,
    pub procent: *mut proc_dir_entry,
    pub devfn: u32,
    pub vendor: u16,
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub class: u32,
    pub hdr_type: u8_,
    pub rom_base_reg: u8_,
    pub pin: u8_,
    pub driver: *mut pci_driver,
    pub dma_mask: u64_,
    pub current_state: pci_power_t,
    pub error_state: pci_channel_state_t,
    pub dev: device,
    pub vendor_compatible: [u16; 4usize],
    pub device_compatible: [u16; 4usize],
    pub cfg_size: i32,
    pub irq: u32,
    pub resource: [resource; 12usize],
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub enable_cnt: atomic_t,
    pub saved_config_space: [u32_; 16usize],
    pub saved_cap_space: hlist_head,
    pub rom_attr: *mut bin_attribute,
    pub rom_attr_enabled: i32,
    pub res_attr: [*mut bin_attribute; 12usize],
}
impl pci_dev {
    #[inline]
    pub fn transparent(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_transparent(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn multifunction(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_multifunction(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn is_busmaster(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_is_busmaster(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn no_msi(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_no_msi(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn no_d1d2(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_no_d1d2(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn block_ucfg_access(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_block_ucfg_access(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn broken_parity_status(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_broken_parity_status(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn msi_enabled(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_msi_enabled(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn msix_enabled(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_msix_enabled(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn is_managed(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_is_managed(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        transparent: u32,
        multifunction: u32,
        is_busmaster: u32,
        no_msi: u32,
        no_d1d2: u32,
        block_ucfg_access: u32,
        broken_parity_status: u32,
        msi_enabled: u32,
        msix_enabled: u32,
        is_managed: u32,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let transparent: u32 = unsafe { ::std::mem::transmute(transparent) };
            transparent as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let multifunction: u32 = unsafe { ::std::mem::transmute(multifunction) };
            multifunction as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let is_busmaster: u32 = unsafe { ::std::mem::transmute(is_busmaster) };
            is_busmaster as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let no_msi: u32 = unsafe { ::std::mem::transmute(no_msi) };
            no_msi as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let no_d1d2: u32 = unsafe { ::std::mem::transmute(no_d1d2) };
            no_d1d2 as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let block_ucfg_access: u32 = unsafe { ::std::mem::transmute(block_ucfg_access) };
            block_ucfg_access as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let broken_parity_status: u32 = unsafe { ::std::mem::transmute(broken_parity_status) };
            broken_parity_status as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let msi_enabled: u32 = unsafe { ::std::mem::transmute(msi_enabled) };
            msi_enabled as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let msix_enabled: u32 = unsafe { ::std::mem::transmute(msix_enabled) };
            msix_enabled as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let is_managed: u32 = unsafe { ::std::mem::transmute(is_managed) };
            is_managed as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const IFNAMSIZ: i32 = 16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_device_stats {
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
pub struct pollfd {
    pub fd: i32,
    pub events: ::std::os::raw::c_short,
    pub revents: ::std::os::raw::c_short,
}
#[repr(C)]
#[derive(Debug)]
pub struct poll_list {
    pub next: *mut poll_list,
    pub len: i32,
    pub entries: __IncompleteArrayField<pollfd>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tvec_t_base_s {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timer_list {
    pub entry: list_head,
    pub expires: u64,
    pub function: ::std::option::Option<unsafe extern "C" fn(arg1: u64)>,
    pub data: u64,
    pub base: *mut tvec_t_base_s,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_device {
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
        ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device) -> i32>,
    pub features: u64,
    pub next_sched: *mut net_device,
    pub ifindex: i32,
    pub iflink: i32,
    pub get_stats:
        ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device) -> *mut net_device_stats>,
    pub stats: net_device_stats,
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
    pub master: *mut net_device,
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
            dev: *mut net_device,
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
    pub gso_skb: *mut sk_buff,
    pub ingress_lock: spinlock_t,
    pub qdisc_ingress: *mut Qdisc,
    pub _xmit_lock: spinlock_t,
    pub xmit_lock_owner: i32,
    pub priv_: *mut ::std::os::raw::c_void,
    pub hard_start_xmit: ::std::option::Option<
        unsafe extern "C" fn(skb: *mut sk_buff, dev: *mut net_device) -> i32,
    >,
    pub trans_start: u64,
    pub watchdog_timeo: i32,
    pub watchdog_timer: timer_list,
    pub refcnt: atomic_t,
    pub todo_list: list_head,
    pub index_hlist: hlist_node,
    pub link_watch_next: *mut net_device,
    pub reg_state: net_device__bindgen_ty_1,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub destructor: ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub open:
        ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device) -> i32>,
    pub stop:
        ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device) -> i32>,
    pub hard_header: ::std::option::Option<
        unsafe extern "C" fn(
            skb: *mut sk_buff,
            dev: *mut net_device,
            type_: u16,
            daddr: *mut ::std::os::raw::c_void,
            saddr: *mut ::std::os::raw::c_void,
            len: u32,
        ) -> i32,
    >,
    pub rebuild_header:
        ::std::option::Option<unsafe extern "C" fn(skb: *mut sk_buff) -> i32>,
    pub set_multicast_list: ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub set_mac_address: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut net_device,
            addr: *mut ::std::os::raw::c_void,
        ) -> i32,
    >,
    pub do_ioctl: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut net_device,
            ifr: *mut ifreq,
            cmd: i32,
        ) -> i32,
    >,
    pub set_config: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut net_device, map: *mut ifmap) -> i32,
    >,
    pub hard_header_cache: ::std::option::Option<
        unsafe extern "C" fn(neigh: *mut neighbour, hh: *mut hh_cache) -> i32,
    >,
    pub header_cache_update: ::std::option::Option<
        unsafe extern "C" fn(
            hh: *mut hh_cache,
            dev: *mut net_device,
            haddr: *mut u8,
        ),
    >,
    pub change_mtu: ::std::option::Option<
        unsafe extern "C" fn(
            dev: *mut net_device,
            new_mtu: i32,
        ) -> i32,
    >,
    pub tx_timeout: ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub vlan_rx_register:
        ::std::option::Option<unsafe extern "C" fn(dev: *mut net_device, grp: *mut vlan_group)>,
    pub vlan_rx_add_vid: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut net_device, vid: u16),
    >,
    pub vlan_rx_kill_vid: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut net_device, vid: u16),
    >,
    pub hard_header_parse: ::std::option::Option<
        unsafe extern "C" fn(
            skb: *mut sk_buff,
            haddr: *mut u8,
        ) -> i32,
    >,
    pub neigh_setup: ::std::option::Option<
        unsafe extern "C" fn(dev: *mut net_device, arg1: *mut neigh_parms) -> i32,
    >,
    pub br_port: *mut net_bridge_port,
    pub dev: device,
    pub sysfs_groups: [*mut attribute_group; 3usize],
}
pub const net_device_NETREG_UNINITIALIZED: u32 = 0;
pub const net_device_NETREG_REGISTERED: u32 = 1;
pub const net_device_NETREG_UNREGISTERING: u32 = 2;
pub const net_device_NETREG_UNREGISTERED: u32 = 3;
pub const net_device_NETREG_RELEASED: u32 = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct module {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kobj_type {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dentry {
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
pub struct dma_coherent_mem {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct attribute_group {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pci_bus {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_dir_entry {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pci_driver {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bin_attribute {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sk_buff {
    pub _address: u8,
}




