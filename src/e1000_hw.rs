pub type __le32 = u32;
pub type __le16 = u16;

pub type __le64 = u64;

pub const e1000_mac_type_e1000_undefined: e1000_mac_type = 0;
pub const e1000_mac_type_e1000_82542_rev2_0: e1000_mac_type = 1;
pub const e1000_mac_type_e1000_82542_rev2_1: e1000_mac_type = 2;
pub const e1000_mac_type_e1000_82543: e1000_mac_type = 3;
pub const e1000_mac_type_e1000_82544: e1000_mac_type = 4;
pub const e1000_mac_type_e1000_82540: e1000_mac_type = 5;
pub const e1000_mac_type_e1000_82545: e1000_mac_type = 6;
pub const e1000_mac_type_e1000_82545_rev_3: e1000_mac_type = 7;
pub const e1000_mac_type_e1000_82546: e1000_mac_type = 8;
pub const e1000_mac_type_e1000_ce4100: e1000_mac_type = 9;
pub const e1000_mac_type_e1000_82546_rev_3: e1000_mac_type = 10;
pub const e1000_mac_type_e1000_82541: e1000_mac_type = 11;
pub const e1000_mac_type_e1000_82541_rev_2: e1000_mac_type = 12;
pub const e1000_mac_type_e1000_82547: e1000_mac_type = 13;
pub const e1000_mac_type_e1000_82547_rev_2: e1000_mac_type = 14;
pub const e1000_mac_type_e1000_num_macs: e1000_mac_type = 15;
pub type e1000_mac_type = u32;

pub const e1000_phy_type_e1000_phy_m88: e1000_phy_type = 0;
pub const e1000_phy_type_e1000_phy_igp: e1000_phy_type = 1;
pub const e1000_phy_type_e1000_phy_8211: e1000_phy_type = 2;
pub const e1000_phy_type_e1000_phy_8201: e1000_phy_type = 3;
pub const e1000_phy_type_e1000_phy_undefined: e1000_phy_type = 255;
pub type e1000_phy_type = u32;

pub const e1000_media_type_e1000_media_type_copper: e1000_media_type = 0;
pub const e1000_media_type_e1000_media_type_fiber: e1000_media_type = 1;
pub const e1000_media_type_e1000_media_type_internal_serdes: e1000_media_type = 2;
pub const e1000_media_type_e1000_num_media_types: e1000_media_type = 3;
pub type e1000_media_type = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_shadow_ram {
    pub eeprom_word: u16,
    pub modified: bool,
}

pub const e1000_fc_type_E1000_FC_NONE: e1000_fc_type = 0;
pub const e1000_fc_type_E1000_FC_RX_PAUSE: e1000_fc_type = 1;
pub const e1000_fc_type_E1000_FC_TX_PAUSE: e1000_fc_type = 2;
pub const e1000_fc_type_E1000_FC_FULL: e1000_fc_type = 3;
pub const e1000_fc_type_E1000_FC_DEFAULT: e1000_fc_type = 0xFF;
pub type e1000_fc_type = u32;

pub const e1000_speed_duplex_type_e1000_10_half: e1000_speed_duplex_type = 0;
pub const e1000_speed_duplex_type_e1000_10_full: e1000_speed_duplex_type = 1;
pub const e1000_speed_duplex_type_e1000_100_half: e1000_speed_duplex_type = 2;
pub const e1000_speed_duplex_type_e1000_100_full: e1000_speed_duplex_type = 3;
pub type e1000_speed_duplex_type = u32;

/// PCI bus speeds
pub type e1000_bus_speed = u32;
pub const e1000_bus_speed_e1000_bus_speed_unknown: e1000_bus_speed = 0;
pub const e1000_bus_speed_e1000_bus_speed_33: e1000_bus_speed = 1;
pub const e1000_bus_speed_e1000_bus_speed_66: e1000_bus_speed = 2;
pub const e1000_bus_speed_e1000_bus_speed_100: e1000_bus_speed = 3;
pub const e1000_bus_speed_e1000_bus_speed_120: e1000_bus_speed = 4;
pub const e1000_bus_speed_e1000_bus_speed_133: e1000_bus_speed = 5;
pub const e1000_bus_speed_e1000_bus_speed_reserved: e1000_bus_speed = 6;

/// PCI bus types
pub type e1000_bus_type = u32;
pub const e1000_bus_type_e1000_bus_type_unknown: e1000_bus_type = 0;
pub const e1000_bus_type_e1000_bus_type_pci: e1000_bus_type = 1;
pub const e1000_bus_type_e1000_bus_type_pcix: e1000_bus_type = 2;
pub const e1000_bus_type_e1000_bus_type_reserved: e1000_bus_type = 3;

/// PCI bus widths
pub type e1000_bus_width = u32;
pub const e1000_bus_width_e1000_bus_width_unknown: e1000_bus_width = 0;
pub const e1000_bus_width_e1000_bus_width_32: e1000_bus_width = 1;
pub const e1000_bus_width_e1000_bus_width_64: e1000_bus_width = 2;
pub const e1000_bus_width_e1000_bus_width_reserved: e1000_bus_width = 3;

/// PHY status info structure and supporting enums
pub type e1000_cable_length = u32;
pub const e1000_cable_length_e1000_cable_length_50: e1000_cable_length = 0;
pub const e1000_cable_length_e1000_cable_length_50_80: e1000_cable_length = 1;
pub const e1000_cable_length_e1000_cable_length_80_110: e1000_cable_length = 2;
pub const e1000_cable_length_e1000_cable_length_110_140: e1000_cable_length = 3;
pub const e1000_cable_length_e1000_cable_length_140: e1000_cable_length = 4;
pub const e1000_cable_length_e1000_cable_length_undefined: e1000_cable_length = 255;

pub type e1000_gg_cable_length = u32;
pub const e1000_gg_cable_length_e1000_gg_cable_length_60: e1000_gg_cable_length = 0;
pub const e1000_gg_cable_length_e1000_gg_cable_length_60_115: e1000_gg_cable_length = 1;
pub const e1000_gg_cable_length_e1000_gg_cable_length_115_150: e1000_gg_cable_length = 2;
pub const e1000_gg_cable_length_e1000_gg_cable_length_150: e1000_gg_cable_length = 4;

pub type e1000_eeprom_type = u32;
pub const e1000_eeprom_type_e1000_eeprom_uninitialized: e1000_eeprom_type = 0;
pub const e1000_eeprom_type_e1000_eeprom_spi: e1000_eeprom_type = 1;
pub const e1000_eeprom_type_e1000_eeprom_microwire: e1000_eeprom_type = 2;
pub const e1000_eeprom_type_e1000_eeprom_flash: e1000_eeprom_type = 3;
pub const e1000_eeprom_type_e1000_eeprom_none: e1000_eeprom_type = 4;
pub const e1000_eeprom_type_e1000_num_eeprom_types: e1000_eeprom_type = 5;

pub type e1000_ms_type = u32;
pub const e1000_ms_type_e1000_ms_hw_default: e1000_ms_type = 0;
pub const e1000_ms_type_e1000_ms_force_master: e1000_ms_type = 1;
pub const e1000_ms_type_e1000_ms_force_slave: e1000_ms_type = 2;
pub const e1000_ms_type_e1000_ms_auto: e1000_ms_type = 3;

pub type e1000_ffe_config = u32;
pub const e1000_ffe_config_e1000_ffe_config_enabled: e1000_ffe_config = 0;
pub const e1000_ffe_config_e1000_ffe_config_active: e1000_ffe_config = 1;
pub const e1000_ffe_config_e1000_ffe_config_blocked: e1000_ffe_config = 2;

pub type e1000_smart_speed = u32;
pub const e1000_smart_speed_e1000_smart_speed_default: e1000_smart_speed = 0;
pub const e1000_smart_speed_e1000_smart_speed_on: e1000_smart_speed = 1;
pub const e1000_smart_speed_e1000_smart_speed_off: e1000_smart_speed = 2;

pub type e1000_dsp_config = u32;
pub const e1000_dsp_config_e1000_dsp_config_disabled: e1000_dsp_config = 0;
pub const e1000_dsp_config_e1000_dsp_config_enabled: e1000_dsp_config = 1;
pub const e1000_dsp_config_e1000_dsp_config_activated: e1000_dsp_config = 2;
pub const e1000_dsp_config_e1000_dsp_config_undefined: e1000_dsp_config = 255;

pub type e1000_10bt_ext_dist_enable = u32;
pub const e1000_10bt_ext_dist_enable_e1000_10bt_ext_dist_enable_normal: e1000_10bt_ext_dist_enable =
    0;
pub const e1000_10bt_ext_dist_enable_e1000_10bt_ext_dist_enable_lower: e1000_10bt_ext_dist_enable =
    1;
pub const e1000_10bt_ext_dist_enable_e1000_10bt_ext_dist_enable_undefined:
    e1000_10bt_ext_dist_enable = 255;

pub type e1000_rev_polarity = u32;
pub const e1000_rev_polarity_e1000_rev_polarity_normal: e1000_rev_polarity = 0;
pub const e1000_rev_polarity_e1000_rev_polarity_reversed: e1000_rev_polarity = 1;
pub const e1000_rev_polarity_e1000_rev_polarity_undefined: e1000_rev_polarity = 255;

pub type e1000_downshift = u32;
pub const e1000_downshift_e1000_downshift_normal: e1000_downshift = 0;
pub const e1000_downshift_e1000_downshift_activated: e1000_downshift = 1;
pub const e1000_downshift_e1000_downshift_undefined: e1000_downshift = 255;

pub type e1000_polarity_reversal = u32;
pub const e1000_polarity_reversal_e1000_polarity_reversal_enabled: e1000_polarity_reversal = 0;
pub const e1000_polarity_reversal_e1000_polarity_reversal_disabled: e1000_polarity_reversal = 1;
pub const e1000_polarity_reversal_e1000_polarity_reversal_undefined: e1000_polarity_reversal = 255;

pub type e1000_auto_x_mode = u32;
pub const e1000_auto_x_mode_e1000_auto_x_mode_manual_mdi: e1000_auto_x_mode = 0;
pub const e1000_auto_x_mode_e1000_auto_x_mode_manual_mdix: e1000_auto_x_mode = 1;
pub const e1000_auto_x_mode_e1000_auto_x_mode_auto1: e1000_auto_x_mode = 2;
pub const e1000_auto_x_mode_e1000_auto_x_mode_auto2: e1000_auto_x_mode = 3;
pub const e1000_auto_x_mode_e1000_auto_x_mode_undefined: e1000_auto_x_mode = 255;

pub type e1000_1000t_rx_status = u32;
pub const e1000_1000t_rx_status_e1000_1000t_rx_status_not_ok: e1000_1000t_rx_status = 0;
pub const e1000_1000t_rx_status_e1000_1000t_rx_status_ok: e1000_1000t_rx_status = 1;
pub const e1000_1000t_rx_status_e1000_1000t_rx_status_undefined: e1000_1000t_rx_status = 255;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_eeprom_info {
    pub type_: e1000_eeprom_type,
    pub word_size: u16,
    pub opcode_bits: u16,
    pub address_bits: u16,
    pub delay_usec: u16,
    pub page_size: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_host_mng_dhcp_cookie {
    pub signature: u32,
    pub status: u8,
    pub reserved0: u8,
    pub vlan_id: u16,
    pub reserved1: u32,
    pub reserved2: u16,
    pub reserved3: u8,
    pub checksum: u8,
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
    pub io_base: u64,
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_phy_stats {
    pub idle_errors: u32,
    pub receive_errors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_context_desc {
    pub lower_setup: e1000_context_desc__bindgen_ty_1,
    pub upper_setup: e1000_context_desc__bindgen_ty_2,
    pub cmd_and_length: __le32,
    pub tcp_seg_setup: e1000_context_desc__bindgen_ty_3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_context_desc__bindgen_ty_1 {
    pub ip_config: __le32,
    pub ip_fields: e1000_context_desc__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_context_desc__bindgen_ty_1__bindgen_ty_1 {
    pub ipcss: u8,
    pub ipcso: u8,
    pub ipcse: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_context_desc__bindgen_ty_2 {
    pub tcp_config: __le32,
    pub tcp_fields: e1000_context_desc__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_context_desc__bindgen_ty_2__bindgen_ty_1 {
    pub tucss: u8,
    pub tucso: u8,
    pub tucse: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_context_desc__bindgen_ty_3 {
    pub data: __le32,
    pub fields: e1000_context_desc__bindgen_ty_3__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_context_desc__bindgen_ty_3__bindgen_ty_1 {
    pub status: u8,
    pub hdr_len: u8,
    pub mss: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_rx_desc {
    pub buffer_addr: __le64,
    pub length: __le16,
    pub csum: __le16,
    pub status: u8,
    pub errors: u8,
    pub special: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_tx_desc {
    pub buffer_addr: __le64,
    pub lower: e1000_tx_desc__bindgen_ty_1,
    pub upper: e1000_tx_desc__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_tx_desc__bindgen_ty_1 {
    pub data: __le32,
    pub flags: e1000_tx_desc__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_tx_desc__bindgen_ty_1__bindgen_ty_1 {
    pub length: __le16,
    pub cso: u8,
    pub cmd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_tx_desc__bindgen_ty_2 {
    pub data: __le32,
    pub fields: e1000_tx_desc__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_tx_desc__bindgen_ty_2__bindgen_ty_1 {
    pub status: u8,
    pub css: u8,
    pub special: __le16,
}
