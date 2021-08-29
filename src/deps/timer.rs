pub type hrtimer_restart = i32;
pub type ktime_t = i64;

pub type __kernel_clockid_t = i32;
pub type clockid_t = __kernel_clockid_t;

//noinspection RsStructNaming
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct atomic64_t {
    pub counter: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seqcount {
    pub sequence: u32,
}

pub type seqcount_t = seqcount;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wait_queue_head {
    pub lock: spinlock_t,
    pub task_list: list_head,
}
pub type wait_queue_head_t = __wait_queue_head;


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
pub struct atomic_t {
    pub counter: i32,
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
#[derive(Debug, Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: u64,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timerqueue_node {
    pub node: rb_node,
    pub expires: ktime_t,
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
    pub index: u32,
    pub clockid: clockid_t,
    pub seq: seqcount_t,
    pub running: *mut hrtimer,
    pub active: timerqueue_head,
    // pub get_time: Option<unsafe extern "C" fn() -> ktime_t>,
    pub offset: ktime_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hrtimer {
    pub node: timerqueue_node,
    pub _softexpires: ktime_t,
    // pub function:
    // Option<unsafe extern "C" fn(arg1: *mut hrtimer) -> hrtimer_restart>,
    pub base: *mut hrtimer_clock_base,
    pub state: u8,
    pub is_rel: u8,
    pub is_soft: u8,
    pub is_hard: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct spinlock_t {
    pub magic: u64,
    pub lock: u64,
    pub babble: u32,
    pub module: *const i8,
    pub owner: *mut i8,
    pub oline: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer_cpu_base {
    pub lock: raw_spinlock_t,
    pub cpu: u32,
    pub active_bases: u32,
    pub clock_was_set_seq: u32,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub expires_next: ktime_t,
    pub next_timer: *mut hrtimer,
    pub softirq_expires_next: ktime_t,
    pub softirq_next_timer: *mut hrtimer,
    pub clock_base: [hrtimer_clock_base; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct gro_list {
    pub list: list_head,
    pub count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct work_struct {
    _unused: [u8; 0],
}
pub type work_func_t = Option<unsafe extern "C" fn(work: *mut work_struct)>;

pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_MONOTONIC: hrtimer_base_type = 0;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_REALTIME: hrtimer_base_type = 1;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_BOOTTIME: hrtimer_base_type = 2;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_TAI: hrtimer_base_type = 3;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_MONOTONIC_SOFT: hrtimer_base_type = 4;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_REALTIME_SOFT: hrtimer_base_type = 5;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_BOOTTIME_SOFT: hrtimer_base_type = 6;
pub const HRTIMER_BASE_TYPE_HRTIMER_BASE_TAI_SOFT: hrtimer_base_type = 7;
pub const HRTIMER_BASE_TYPE_HRTIMER_MAX_CLOCK_BASES: hrtimer_base_type = 8;
pub type hrtimer_base_type = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timer_list {
    pub entry: hlist_node,
    pub expires: u64,
    pub function: Option<unsafe extern "C" fn(arg1: *mut timer_list)>,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct workqueue_struct {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct delayed_work {
    pub work: work_struct,
    pub __bindgen_padding_0: [u64; 4usize],
    pub timer: timer_list,
    pub wq: *mut workqueue_struct,
    pub cpu: i32,
}

pub type atomic_long_t = atomic64_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mutex {
    pub owner: atomic_long_t,
    pub wait_lock: spinlock_t,
    pub wait_list: list_head,
}
