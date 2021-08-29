use crate::deps::{list_head, klist, klist_node, kobject, __BindgenBitfieldUnit,
                  semaphore, class, module, device_attribute, attribute_group, resource, device};
use crate::deps::timer::{spinlock_t, atomic_t, hlist_head};

pub type pci_power_t = i32;

pub type pci_channel_state_t = u32;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bin_attribute {
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
pub struct PciDev {
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
    pub hdr_type: u8,
    pub rom_base_reg: u8,
    pub pin: u8,
    pub driver: *mut pci_driver,
    pub dma_mask: u64,
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
    pub saved_config_space: [u32; 16usize],
    pub saved_cap_space: hlist_head,
    pub rom_attr: *mut bin_attribute,
    pub rom_attr_enabled: i32,
    pub res_attr: [*mut bin_attribute; 12usize],
}