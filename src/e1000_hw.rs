use crate::e1000::E1000Adapter;

pub type c_void = [u8; 0];
pub type __le32 = u32;
pub type __le16 = u16;

pub type __le64 = u64;


pub const E1000_MAC_TYPE_E1000_UNDEFINED: E1000MacType = 0;
pub const E1000_MAC_TYPE_E1000_82542_REV2_0: E1000MacType = 1;
pub const E1000_MAC_TYPE_E1000_82542_REV2_1: E1000MacType = 2;
pub const E1000_MAC_TYPE_E1000_82543: E1000MacType = 3;
pub const E1000_MAC_TYPE_E1000_82544: E1000MacType = 4;
pub const E1000_MAC_TYPE_E1000_82540: E1000MacType = 5;
pub const E1000_MAC_TYPE_E1000_82545: E1000MacType = 6;
pub const E1000_MAC_TYPE_E1000_82545_REV_3: E1000MacType = 7;
pub const E1000_MAC_TYPE_E1000_82546: E1000MacType = 8;
pub const E1000_MAC_TYPE_E1000_CE4100: E1000MacType = 9;
pub const E1000_MAC_TYPE_E1000_82546_REV_3: E1000MacType = 10;
pub const E1000_MAC_TYPE_E1000_82541: E1000MacType = 11;
pub const E1000_MAC_TYPE_E1000_82541_REV_2: E1000MacType = 12;
pub const E1000_MAC_TYPE_E1000_82547: E1000MacType = 13;
pub const E1000_MAC_TYPE_E1000_82547_REV_2: E1000MacType = 14;
pub const E1000_MAC_TYPE_E1000_NUM_MACS: E1000MacType = 15;
pub type E1000MacType = u32;

pub const E1000_PHY_TYPE_E1000_PHY_M88: E1000PhyType = 0;
pub const E1000_PHY_TYPE_E1000_PHY_IGP: E1000PhyType = 1;
pub const E1000_PHY_TYPE_E1000_PHY_8211: E1000PhyType = 2;
pub const E1000_PHY_TYPE_E1000_PHY_8201: E1000PhyType = 3;
pub const E1000_PHY_TYPE_E1000_PHY_UNDEFINED: E1000PhyType = 255;
pub type E1000PhyType = u32;

pub const E1000_MEDIA_TYPE_E1000_MEDIA_TYPE_COPPER: E1000MediaType = 0;
pub const E1000_MEDIA_TYPE_E1000_MEDIA_TYPE_FIBER: E1000MediaType = 1;
pub const E1000_MEDIA_TYPE_E1000_MEDIA_TYPE_INTERNAL_SERDES: E1000MediaType = 2;
pub const E1000_MEDIA_TYPE_E1000_NUM_MEDIA_TYPES: E1000MediaType = 3;
pub type E1000MediaType = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000ShadowRam {
    pub eeprom_word: u16,
    pub modified: bool,
}

pub const E1000_FC_TYPE_E1000_FC_NONE: E1000FcType = 0;
pub const E1000_FC_TYPE_E1000_FC_RX_PAUSE: E1000FcType = 1;
pub const E1000_FC_TYPE_E1000_FC_TX_PAUSE: E1000FcType = 2;
pub const E1000_FC_TYPE_E1000_FC_FULL: E1000FcType = 3;
pub const E1000_FC_TYPE_E1000_FC_DEFAULT: E1000FcType = 0xFF;
pub type E1000FcType = u32;

pub const E1000_SPEED_DUPLEX_TYPE_E1000_10_HALF: E1000SpeedDuplexType = 0;
pub const E1000_SPEED_DUPLEX_TYPE_E1000_10_FULL: E1000SpeedDuplexType = 1;
pub const E1000_SPEED_DUPLEX_TYPE_E1000_100_HALF: E1000SpeedDuplexType = 2;
pub const E1000_SPEED_DUPLEX_TYPE_E1000_100_FULL: E1000SpeedDuplexType = 3;
pub type E1000SpeedDuplexType = u32;

/// PCI bus speeds
pub type E1000BusSpeed = u32;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_UNKNOWN: E1000BusSpeed = 0;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_33: E1000BusSpeed = 1;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_66: E1000BusSpeed = 2;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_100: E1000BusSpeed = 3;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_120: E1000BusSpeed = 4;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_133: E1000BusSpeed = 5;
pub const E1000_BUS_SPEED_E1000_BUS_SPEED_RESERVED: E1000BusSpeed = 6;

/// PCI bus types
pub type E1000BusType = u32;
pub const E1000_BUS_TYPE_E1000_BUS_TYPE_UNKNOWN: E1000BusType = 0;
pub const E1000_BUS_TYPE_E1000_BUS_TYPE_PCI: E1000BusType = 1;
pub const E1000_BUS_TYPE_E1000_BUS_TYPE_PCIX: E1000BusType = 2;
pub const E1000_BUS_TYPE_E1000_BUS_TYPE_RESERVED: E1000BusType = 3;

/// PCI bus widths
pub type E1000BusWidth = u32;
pub const E1000_BUS_WIDTH_E1000_BUS_WIDTH_UNKNOWN: E1000BusWidth = 0;
pub const E1000_BUS_WIDTH_E1000_BUS_WIDTH_32: E1000BusWidth = 1;
pub const E1000_BUS_WIDTH_E1000_BUS_WIDTH_64: E1000BusWidth = 2;
pub const E1000_BUS_WIDTH_E1000_BUS_WIDTH_RESERVED: E1000BusWidth = 3;

/// PHY status info structure and supporting enums
pub type E1000CableLength = u32;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_50: E1000CableLength = 0;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_50_80: E1000CableLength = 1;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_80_110: E1000CableLength = 2;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_110_140: E1000CableLength = 3;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_140: E1000CableLength = 4;
pub const E1000_CABLE_LENGTH_E1000_CABLE_LENGTH_UNDEFINED: E1000CableLength = 255;

pub type E1000GgCableLength = u32;
pub const E1000_GG_CABLE_LENGTH_E1000_GG_CABLE_LENGTH_60: E1000GgCableLength = 0;
pub const E1000_GG_CABLE_LENGTH_E1000_GG_CABLE_LENGTH_60_115: E1000GgCableLength = 1;
pub const E1000_GG_CABLE_LENGTH_E1000_GG_CABLE_LENGTH_115_150: E1000GgCableLength = 2;
pub const E1000_GG_CABLE_LENGTH_E1000_GG_CABLE_LENGTH_150: E1000GgCableLength = 4;

pub type E1000EepromType = u32;
pub const E1000_EEPROM_TYPE_E1000_EEPROM_UNINITIALIZED: E1000EepromType = 0;
pub const E1000_EEPROM_TYPE_E1000_EEPROM_SPI: E1000EepromType = 1;
pub const E1000_EEPROM_TYPE_E1000_EEPROM_MICROWIRE: E1000EepromType = 2;
pub const E1000_EEPROM_TYPE_E1000_EEPROM_FLASH: E1000EepromType = 3;
pub const E1000_EEPROM_TYPE_E1000_EEPROM_NONE: E1000EepromType = 4;
pub const E1000_EEPROM_TYPE_E1000_NUM_EEPROM_TYPES: E1000EepromType = 5;

pub type E1000MsType = u32;
pub const E1000_MS_TYPE_E1000_MS_HW_DEFAULT: E1000MsType = 0;
pub const E1000_MS_TYPE_E1000_MS_FORCE_MASTER: E1000MsType = 1;
pub const E1000_MS_TYPE_E1000_MS_FORCE_SLAVE: E1000MsType = 2;
pub const E1000_MS_TYPE_E1000_MS_AUTO: E1000MsType = 3;

pub type E1000FfeConfig = u32;
pub const E1000_FFE_CONFIG_E1000_FFE_CONFIG_ENABLED: E1000FfeConfig = 0;
pub const E1000_FFE_CONFIG_E1000_FFE_CONFIG_ACTIVE: E1000FfeConfig = 1;
pub const E1000_FFE_CONFIG_E1000_FFE_CONFIG_BLOCKED: E1000FfeConfig = 2;

pub type E1000SmartSpeed = u32;
pub const E1000_SMART_SPEED_E1000_SMART_SPEED_DEFAULT: E1000SmartSpeed = 0;
pub const E1000_SMART_SPEED_E1000_SMART_SPEED_ON: E1000SmartSpeed = 1;
pub const E1000_SMART_SPEED_E1000_SMART_SPEED_OFF: E1000SmartSpeed = 2;

pub type E1000DspConfig = u32;
pub const E1000_DSP_CONFIG_E1000_DSP_CONFIG_DISABLED: E1000DspConfig = 0;
pub const E1000_DSP_CONFIG_E1000_DSP_CONFIG_ENABLED: E1000DspConfig = 1;
pub const E1000_DSP_CONFIG_E1000_DSP_CONFIG_ACTIVATED: E1000DspConfig = 2;
pub const E1000_DSP_CONFIG_E1000_DSP_CONFIG_UNDEFINED: E1000DspConfig = 255;

pub type E100010btExtDistEnable = u32;
pub const E1000_10BT_EXT_DIST_ENABLE_E1000_10BT_EXT_DIST_ENABLE_NORMAL: E100010btExtDistEnable = 0;
pub const E1000_10BT_EXT_DIST_ENABLE_E1000_10BT_EXT_DIST_ENABLE_LOWER: E100010btExtDistEnable = 1;
pub const E1000_10BT_EXT_DIST_ENABLE_E1000_10BT_EXT_DIST_ENABLE_UNDEFINED: E100010btExtDistEnable =
    255;

pub type E1000RevPolarity = u32;
pub const E1000_REV_POLARITY_E1000_REV_POLARITY_NORMAL: E1000RevPolarity = 0;
pub const E1000_REV_POLARITY_E1000_REV_POLARITY_REVERSED: E1000RevPolarity = 1;
pub const E1000_REV_POLARITY_E1000_REV_POLARITY_UNDEFINED: E1000RevPolarity = 255;

pub type E1000Downshift = u32;
pub const E1000_DOWNSHIFT_E1000_DOWNSHIFT_NORMAL: E1000Downshift = 0;
pub const E1000_DOWNSHIFT_E1000_DOWNSHIFT_ACTIVATED: E1000Downshift = 1;
pub const E1000_DOWNSHIFT_E1000_DOWNSHIFT_UNDEFINED: E1000Downshift = 255;

pub type E1000PolarityReversal = u32;
pub const E1000_POLARITY_REVERSAL_E1000_POLARITY_REVERSAL_ENABLED: E1000PolarityReversal = 0;
pub const E1000_POLARITY_REVERSAL_E1000_POLARITY_REVERSAL_DISABLED: E1000PolarityReversal = 1;
pub const E1000_POLARITY_REVERSAL_E1000_POLARITY_REVERSAL_UNDEFINED: E1000PolarityReversal = 255;

pub type E1000AutoXMode = u32;
pub const E1000_AUTO_X_MODE_E1000_AUTO_X_MODE_MANUAL_MDI: E1000AutoXMode = 0;
pub const E1000_AUTO_X_MODE_E1000_AUTO_X_MODE_MANUAL_MDIX: E1000AutoXMode = 1;
pub const E1000_AUTO_X_MODE_E1000_AUTO_X_MODE_AUTO1: E1000AutoXMode = 2;
pub const E1000_AUTO_X_MODE_E1000_AUTO_X_MODE_AUTO2: E1000AutoXMode = 3;
pub const E1000_AUTO_X_MODE_E1000_AUTO_X_MODE_UNDEFINED: E1000AutoXMode = 255;

pub type E10001000tRxStatus = u32;
pub const E1000_1000T_RX_STATUS_E1000_1000T_RX_STATUS_NOT_OK: E10001000tRxStatus = 0;
pub const E1000_1000T_RX_STATUS_E1000_1000T_RX_STATUS_OK: E10001000tRxStatus = 1;
pub const E1000_1000T_RX_STATUS_E1000_1000T_RX_STATUS_UNDEFINED: E10001000tRxStatus = 255;

///  Flex ASF Information
pub const E1000_HOST_IF_MAX_SIZE: u32 = 2048;

pub const E1000_ALIGN_TYPE_E1000_BYTE_ALIGN: E1000AlignType = 0;
pub const E1000_ALIGN_TYPE_E1000_WORD_ALIGN: E1000AlignType = 1;
pub const E1000_ALIGN_TYPE_E1000_DWORD_ALIGN: E1000AlignType = 2;
pub type E1000AlignType = u32;

pub const E1000_SUCCESS: u32 = 0;
pub const E1000_ERR_EEPROM: u32 = 1;
pub const E1000_ERR_PHY: u32 = 2;
pub const E1000_ERR_CONFIG: u32 = 3;
pub const E1000_ERR_PARAM: u32 = 4;
pub const E1000_ERR_MAC_TYPE: u32 = 5;
pub const E1000_ERR_PHY_TYPE: u32 = 6;
pub const E1000_ERR_RESET: u32 = 9;
pub const E1000_ERR_MASTER_REQUESTS_PENDING: u32 = 10;
pub const E1000_ERR_HOST_INTERFACE_COMMAND: u32 = 11;
pub const E1000_BLK_PHY_RESET: u32 = 12;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000HostMngCommandHeader {
    pub command_id: u8,
    pub checksum: u8,
    pub reserved1: u16,
    pub reserved2: u16,
    pub command_length: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000HostMngCommandInfo {
    pub command_header: E1000HostMngCommandHeader,
    pub command_data: [u8; 1784usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct e1000_host_mng_dhcp_cookie {
    signature: u32,
    status: u8,
    reserved0: u8,
    vlan_id: u16,
    reserved1: u32,
    reserved2: u16,
    reserved3: u8,
    checksum: u8,
}

pub const E1000_DEV_ID_82542: u32 = 0x1000;
pub const E1000_DEV_ID_82543GC_FIBER: u32 = 0x1001;
pub const E1000_DEV_ID_82543GC_COPPER: u32 = 0x1004;
pub const E1000_DEV_ID_82544EI_COPPER: u32 = 0x1008;
pub const E1000_DEV_ID_82544EI_FIBER: u32 = 0x1009;
pub const E1000_DEV_ID_82544GC_COPPER: u32 = 0x100C;
pub const E1000_DEV_ID_82544GC_LOM: u32 = 0x100D;
pub const E1000_DEV_ID_82540EM: u32 = 0x100E;
pub const E1000_DEV_ID_82540EM_LOM: u32 = 0x1015;
pub const E1000_DEV_ID_82540EP_LOM: u32 = 0x1016;
pub const E1000_DEV_ID_82540EP: u32 = 0x1017;
pub const E1000_DEV_ID_82540EP_LP: u32 = 0x101E;
pub const E1000_DEV_ID_82545EM_COPPER: u32 = 0x100F;
pub const E1000_DEV_ID_82545EM_FIBER: u32 = 0x1011;
pub const E1000_DEV_ID_82545GM_COPPER: u32 = 0x1026;
pub const E1000_DEV_ID_82545GM_FIBER: u32 = 0x1027;
pub const E1000_DEV_ID_82545GM_SERDES: u32 = 0x1028;
pub const E1000_DEV_ID_82546EB_COPPER: u32 = 0x1010;
pub const E1000_DEV_ID_82546EB_FIBER: u32 = 0x1012;
pub const E1000_DEV_ID_82546EB_QUAD_COPPER: u32 = 0x101D;
pub const E1000_DEV_ID_82541EI: u32 = 0x1013;
pub const E1000_DEV_ID_82541EI_MOBILE: u32 = 0x1018;
pub const E1000_DEV_ID_82541ER_LOM: u32 = 0x1014;
pub const E1000_DEV_ID_82541ER: u32 = 0x1078;
pub const E1000_DEV_ID_82547GI: u32 = 0x1075;
pub const E1000_DEV_ID_82541GI: u32 = 0x1076;
pub const E1000_DEV_ID_82541GI_MOBILE: u32 = 0x1077;
pub const E1000_DEV_ID_82541GI_LF: u32 = 0x107C;
pub const E1000_DEV_ID_82546GB_COPPER: u32 = 0x1079;
pub const E1000_DEV_ID_82546GB_FIBER: u32 = 0x107A;
pub const E1000_DEV_ID_82546GB_SERDES: u32 = 0x107B;
pub const E1000_DEV_ID_82546GB_PCIE: u32 = 0x108A;
pub const E1000_DEV_ID_82546GB_QUAD_COPPER: u32 = 0x1099;
pub const E1000_DEV_ID_82547EI: u32 = 0x1019;
pub const E1000_DEV_ID_82547EI_MOBILE: u32 = 0x101A;
pub const E1000_DEV_ID_82546GB_QUAD_COPPER_KSP3: u32 = 0x10B5;
pub const E1000_DEV_ID_INTEL_CE4100_GBE: u32 = 0x2E6E;

pub const NODE_ADDRESS_SIZE: u32 = 6;

pub const MAC_DECODE_SIZE: u32 = 131072;

pub const E1000_82542_2_0_REV_ID: u32 = 2;
pub const E1000_82542_2_1_REV_ID: u32 = 3;
pub const E1000_REVISION_0: u32 = 0;
pub const E1000_REVISION_1: u32 = 1;
pub const E1000_REVISION_2: u32 = 2;
pub const E1000_REVISION_3: u32 = 3;
pub const SPEED_10: u32 = 10;
pub const SPEED_100: u32 = 100;
pub const SPEED_1000: u32 = 1000;
pub const HALF_DUPLEX: u32 = 1;
pub const FULL_DUPLEX: u32 = 2;

pub const ENET_HEADER_SIZE: u32 = 14;
pub const MINIMUM_ETHERNET_FRAME_SIZE: u32 = 64;
pub const ETHERNET_FCS_SIZE: u32 = 4;
pub const MINIMUM_ETHERNET_PACKET_SIZE: u32 = 60;
pub const CRC_LENGTH: u32 = 4;
pub const MAX_JUMBO_FRAME_SIZE: u32 = 0x3F00;

pub const VLAN_TAG_SIZE: u32 = 4;
pub const ETHERNET_IEEE_VLAN_TYPE: u32 = 0x8100;
pub const ETHERNET_IP_TYPE: u32 = 0x0800;
pub const ETHERNET_ARP_TYPE: u32 = 0x0806;

pub const IP_PROTOCOL_TCP: u32 = 6;
pub const IP_PROTOCOL_UDP: u32 = 0x11;

pub const E1000_RAR_ENTRIES: u32 = 15;
pub const MIN_NUMBER_OF_DESCRIPTORS: u32 = 8;
pub const MAX_NUMBER_OF_DESCRIPTORS: u32 = 0xFFF8;

pub const MAX_PS_BUFFERS: u32 = 4;

pub const E1000_RXD_STAT_DD: u32 = 1;
pub const E1000_RXD_STAT_EOP: u32 = 2;
pub const E1000_RXD_STAT_IXSM: u32 = 4;
pub const E1000_RXD_STAT_VP: u32 = 8;
pub const E1000_RXD_STAT_UDPCS: u32 = 16;
pub const E1000_RXD_STAT_TCPCS: u32 = 32;
pub const E1000_RXD_STAT_IPCS: u32 = 64;
pub const E1000_RXD_STAT_PIF: u32 = 128;
pub const E1000_RXD_STAT_IPIDV: u32 = 512;
pub const E1000_RXD_STAT_UDPV: u32 = 1024;
pub const E1000_RXD_STAT_ACK: u32 = 32768;
pub const E1000_RXD_ERR_CE: u32 = 1;
pub const E1000_RXD_ERR_SE: u32 = 2;
pub const E1000_RXD_ERR_SEQ: u32 = 4;
pub const E1000_RXD_ERR_CXE: u32 = 16;
pub const E1000_RXD_ERR_TCPE: u32 = 32;
pub const E1000_RXD_ERR_IPE: u32 = 64;
pub const E1000_RXD_ERR_RXE: u32 = 128;
pub const E1000_RXD_SPC_VLAN_MASK: u32 = 4095;
pub const E1000_RXD_SPC_PRI_MASK: u32 = 57344;
pub const E1000_RXD_SPC_PRI_SHIFT: u32 = 13;
pub const E1000_RXD_SPC_CFI_MASK: u32 = 4096;
pub const E1000_RXD_SPC_CFI_SHIFT: u32 = 12;

pub const E1000_RXDEXT_STATERR_CE: u32 = 16777216;
pub const E1000_RXDEXT_STATERR_SE: u32 = 33554432;
pub const E1000_RXDEXT_STATERR_SEQ: u32 = 67108864;
pub const E1000_RXDEXT_STATERR_CXE: u32 = 268435456;
pub const E1000_RXDEXT_STATERR_TCPE: u32 = 536870912;
pub const E1000_RXDEXT_STATERR_IPE: u32 = 1073741824;
pub const E1000_RXDEXT_STATERR_RXE: u32 = 2147483648;
pub const E1000_RXDPS_HDRSTAT_HDRSP: u32 = 32768;
pub const E1000_RXDPS_HDRSTAT_HDRLEN_MASK: u32 = 1023;

pub const E1000_RXD_ERR_FRAME_ERR_MASK: u32 = 151;
pub const E1000_RXDEXT_ERR_FRAME_ERR_MASK: u32 = 2533359616;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000EepromInfo {
    pub type_: E1000EepromType,
    pub word_size: u16,
    pub opcode_bits: u16,
    pub address_bits: u16,
    pub delay_usec: u16,
    pub page_size: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000HostMngDhcpCookie {
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
#[derive(Debug, Clone)]
pub struct E1000Hw<'a> {
    pub hw_addr: *mut u8,
    pub flash_address: *mut u8,
    pub ce4100_gbe_mdio_base_virt: *mut c_void,
    pub mac_type: E1000MacType,
    pub phy_type: E1000PhyType,
    pub phy_init_script: u32,
    pub media_type: E1000MediaType,
    pub back: &'a E1000Adapter<'a>,
    pub eeprom_shadow_ram: *mut E1000ShadowRam,
    pub flash_bank_size: u32,
    pub flash_base_addr: u32,
    pub fc: E1000FcType,
    pub bus_speed: E1000BusSpeed,
    pub bus_width: E1000BusWidth,
    pub bus_type: E1000BusType,
    pub eeprom: E1000EepromInfo,
    pub master_slave: E1000MsType,
    pub original_master_slave: E1000MsType,
    pub ffe_config_state: E1000FfeConfig,
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
    pub mng_cookie: E1000HostMngDhcpCookie,
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
    pub smart_speed: E1000SmartSpeed,
    pub dsp_config_state: E1000DspConfig,
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
pub struct E1000HwStats {
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
pub struct E1000PhyInfo {
    pub cable_length: E1000CableLength,
    pub extended_10bt_distance: E100010btExtDistEnable,
    pub cable_polarity: E1000RevPolarity,
    pub downshift: E1000Downshift,
    pub polarity_correction: E1000PolarityReversal,
    pub mdix_mode: E1000AutoXMode,
    pub local_rx: E10001000tRxStatus,
    pub remote_rx: E10001000tRxStatus,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000PhyStats {
    pub idle_errors: u32,
    pub receive_errors: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPConfig_(__le32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPFields_ {
    ipcss: u8,
    ipcso: u8,
    ipcse: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum LowerSetup {
    IPConfig(IPConfig_),
    IPFields(IPFields_),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TCPConfig_(__le32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TCPFields_ {
    tucss: u8,
    tucso: u8,
    tucse: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum UpperSetup {
    TCPConfig(TCPConfig_),
    TCPFields(TCPFields_),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fields__ {
    status: u8,
    hdr_len: u8,
    mss: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TcpSegSetup {
    Data(__le32),
    Fields(Fields__),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct E1000ContextDesc {
    pub lower_setup: LowerSetup,
    pub upper_setup: UpperSetup,
    pub cmd_and_length: __le32,
    pub tcp_seq_setup: TcpSegSetup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000RxDesc {
    pub buffer_addr: __le64,
    pub length: __le16,
    pub csum: __le16,
    pub errors: u8,
    pub special: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Flags_ {
    length: __le16,
    cso: u8,
    cmd: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data_(__le32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Lower_ {
    Data(Data_),
    Flags(Flags_),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fields_ {
    statu: u8,
    css: u8,
    special: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Upper_ {
    Data(Data_),
    Flags(Fields_),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Flags__ {
    length: __le16,
    typ_len_ext: u8,
    cmd: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum LowerE1000txDesc {
    Data(Data_),
    Flags(Flags__),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FieldsE1000txDesc {
    status: u8,
    popts: u8,
    special: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum UpperE1000txDesc {
    Data(Data_),
    Fields(FieldsE1000txDesc),
}

/// Transmit Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000TxDesc {
    pub buffer_addr: __le64,
    pub lower: LowerE1000txDesc,
    pub upper: UpperE1000txDesc,
}

/// Offload data descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct E1000DataDesc {
    pub buffer_addr: __le64,
    pub lower: Lower_,
    pub upper: Upper_,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CsumIp {
    /// IP id
    pub ip_id: u8,
    /// Packet Checksum
    pub csum: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum HiDword {
    Rss(__le32),
    CsumIp(CsumIp),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Lower {
    mrq: __le32,
    hi_dword: HiDword,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Upper {
    status_error: __le32,
    length: __le16,
    vlan: __le16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WriteBack {
    lower: Lower,
    upper: Upper,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Read_ {
    buffer_addr: __le64,
    reserved: __le64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum E1000RxDescExtended {
    WB(WriteBack),
    Read(Read_),
}

pub const E1000_FLEXIBLE_FILTER_COUNT_MAX: u32 = 4;

pub const E1000_FLEXIBLE_FILTER_SIZE_MAX: u32 = 128;

pub const E1000_FFLT_SIZE: u32 = E1000_FLEXIBLE_FILTER_COUNT_MAX;
pub const E1000_FFMT_SIZE: u32 = E1000_FLEXIBLE_FILTER_SIZE_MAX;
pub const E1000_FFVT_SIZE: u32 = E1000_FLEXIBLE_FILTER_SIZE_MAX;

pub const E1000_DISABLE_SERDES_LOOPBACK: u32 = 0x0400;

pub const E1000_TXD_DTYP_D: u32 = 0x00100000;
pub const E1000_TXD_DTYP_C: u32 = 0x00000000;
pub const E1000_TXD_POPTS_IXSM: u32 = 0x01;
pub const E1000_TXD_POPTS_TXSM: u32 = 0x02;
pub const E1000_TXD_CMD_EOP: u32 = 0x01000000;
pub const E1000_TXD_CMD_IFCS: u32 = 0x02000000;
pub const E1000_TXD_CMD_IC: u32 = 0x04000000;
pub const E1000_TXD_CMD_RS: u32 = 0x08000000;
pub const E1000_TXD_CMD_RPS: u32 = 0x10000000;
pub const E1000_TXD_CMD_DEXT: u32 = 0x20000000;
pub const E1000_TXD_CMD_VLE: u32 = 0x40000000;
pub const E1000_TXD_CMD_IDE: u32 = 0x80000000;
pub const E1000_TXD_STAT_DD: u32 = 0x00000001;
pub const E1000_TXD_STAT_EC: u32 = 0x00000002;
pub const E1000_TXD_STAT_LC: u32 = 0x00000004;
pub const E1000_TXD_STAT_TU: u32 = 0x00000008;
pub const E1000_TXD_CMD_TCP: u32 = 0x01000000;
pub const E1000_TXD_CMD_IP: u32 = 0x02000000;
pub const E1000_TXD_CMD_TSE: u32 = 0x04000000;
pub const E1000_TXD_STAT_TC: u32 = 0x00000004;

pub const E1000_NUM_UNICAST: u32 = 16;
pub const E1000_MC_TBL_SIZE: u32 = 128;
pub const E1000_VLAN_FILTER_TBL_SIZE: u32 = 128;

pub const E1000_CTRL: u32 = 0x00000;
pub const E1000_CTRL_DUP: u32 = 0x00004;
pub const E1000_STATUS: u32 = 0x00008;
pub const E1000_EECD: u32 = 0x00010;
pub const E1000_EERD: u32 = 0x00014;
pub const E1000_CTRL_EXT: u32 = 0x00018;
pub const E1000_FLA: u32 = 0x0001C;
pub const E1000_MDIC: u32 = 0x00020;

pub const E1000_SCTL: u32 = 0x00024;
pub const E1000_FEXTNVM: u32 = 0x00028;
pub const E1000_FCAL: u32 = 0x00028;
pub const E1000_FCAH: u32 = 0x0002C;
pub const E1000_FCT: u32 = 0x00030;
pub const E1000_VET: u32 = 0x00038;
pub const E1000_ICR: u32 = 0x000C0;
pub const E1000_ITR: u32 = 0x000C4;
pub const E1000_ICS: u32 = 0x000C8;
pub const E1000_IMS: u32 = 0x000D0;
pub const E1000_IMC: u32 = 0x000D8;
pub const E1000_IAM: u32 = 0x000E0;

pub const E1000_CTL_AUX: u32 = 0x000E0;
pub const E1000_CTL_AUX_END_SEL_SHIFT: u32 = 10;
pub const E1000_CTL_AUX_ENDIANESS_SHIFT: u32 = 8;
pub const E1000_CTL_AUX_RGMII_RMII_SHIFT: u32 = 0;

pub const E1000_RCTL: u32 = 0x00100;
pub const E1000_RDTR1: u32 = 0x02820;
pub const E1000_RDBAL1: u32 = 0x02900;
pub const E1000_RDBAH1: u32 = 0x02904;
pub const E1000_RDLEN1: u32 = 0x02908;
pub const E1000_RDH1: u32 = 0x02910;
pub const E1000_RDT1: u32 = 0x02918;
pub const E1000_FCTTV: u32 = 0x00170;
pub const E1000_TXCW: u32 = 0x00178;
pub const E1000_RXCW: u32 = 0x00180;
pub const E1000_TCTL: u32 = 0x00400;
pub const E1000_TCTL_EXT: u32 = 0x00404;
pub const E1000_TIPG: u32 = 0x00410;
pub const E1000_TBT: u32 = 0x00448;
pub const E1000_AIT: u32 = 0x00458;
pub const E1000_LEDCTL: u32 = 0x00E00;
pub const E1000_EXTCNF_CTRL: u32 = 0x00F00;
pub const E1000_EXTCNF_SIZE: u32 = 0x00F08;
pub const E1000_PHY_CTRL: u32 = 0x00F10;
pub const FEXTNVM_SW_CONFIG: u32 = 0x0001;
pub const E1000_PBA: u32 = 0x01000;
pub const E1000_PBS: u32 = 0x01008;
pub const E1000_EEMNGCTL: u32 = 0x01010;
pub const E1000_FLASH_UPDATES: u32 = 1000;
pub const E1000_EEARBC: u32 = 0x01024;
pub const E1000_FLASHT: u32 = 0x01028;
pub const E1000_EEWR: u32 = 0x0102C;
pub const E1000_FLSWCTL: u32 = 0x01030;
pub const E1000_FLSWDATA: u32 = 0x01034;
pub const E1000_FLSWCNT: u32 = 0x01038;
pub const E1000_FLOP: u32 = 0x0103C;
pub const E1000_ERT: u32 = 0x02008;
pub const E1000_FCRTL: u32 = 0x02160;
pub const E1000_FCRTH: u32 = 0x02168;
pub const E1000_PSRCTL: u32 = 0x02170;
pub const E1000_RDFH: u32 = 0x02410;
pub const E1000_RDFT: u32 = 0x02418;
pub const E1000_RDFHS: u32 = 0x02420;
pub const E1000_RDFTS: u32 = 0x02428;
pub const E1000_RDFPC: u32 = 0x02430;
pub const E1000_RDBAL: u32 = 0x02800;
pub const E1000_RDBAH: u32 = 0x02804;
pub const E1000_RDLEN: u32 = 0x02808;
pub const E1000_RDH: u32 = 0x02810;
pub const E1000_RDT: u32 = 0x02818;
pub const E1000_RDTR: u32 = 0x02820;
pub const E1000_RDBAL0: u32 = E1000_RDBAL;
pub const E1000_RDBAH0: u32 = E1000_RDBAH;
pub const E1000_RDLEN0: u32 = E1000_RDLEN;
pub const E1000_RDH0: u32 = E1000_RDH;
pub const E1000_RDT0: u32 = E1000_RDT;
pub const E1000_RDTR0: u32 = E1000_RDTR;
pub const E1000_RXDCTL: u32 = 0x02828;
pub const E1000_RXDCTL1: u32 = 0x02928;
pub const E1000_RADV: u32 = 0x0282C;
pub const E1000_RSRPD: u32 = 0x02C00;
pub const E1000_RAID: u32 = 0x02C08;
pub const E1000_TXDMAC: u32 = 0x03000;
pub const E1000_KABGTXD: u32 = 0x03004;
pub const E1000_TDFH: u32 = 0x03410;
pub const E1000_TDFT: u32 = 0x03418;
pub const E1000_TDFHS: u32 = 0x03420;
pub const E1000_TDFTS: u32 = 0x03428;
pub const E1000_TDFPC: u32 = 0x03430;
pub const E1000_TDBAL: u32 = 0x03800;
pub const E1000_TDBAH: u32 = 0x03804;
pub const E1000_TDLEN: u32 = 0x03808;
pub const E1000_TDH: u32 = 0x03810;
pub const E1000_TDT: u32 = 0x03818;
pub const E1000_TIDV: u32 = 0x03820;
pub const E1000_TXDCTL: u32 = 0x03828;
pub const E1000_TADV: u32 = 0x0382C;
pub const E1000_TSPMT: u32 = 0x03830;
pub const E1000_TARC0: u32 = 0x03840;
pub const E1000_TDBAL1: u32 = 0x03900;
pub const E1000_TDBAH1: u32 = 0x03904;
pub const E1000_TDLEN1: u32 = 0x03908;
pub const E1000_TDH1: u32 = 0x03910;
pub const E1000_TDT1: u32 = 0x03918;
pub const E1000_TXDCTL1: u32 = 0x03928;
pub const E1000_TARC1: u32 = 0x03940;
pub const E1000_CRCERRS: u32 = 0x04000;
pub const E1000_ALGNERRC: u32 = 0x04004;
pub const E1000_SYMERRS: u32 = 0x04008;
pub const E1000_RXERRC: u32 = 0x0400C;
pub const E1000_MPC: u32 = 0x04010;
pub const E1000_SCC: u32 = 0x04014;
pub const E1000_ECOL: u32 = 0x04018;
pub const E1000_MCC: u32 = 0x0401C;
pub const E1000_LATECOL: u32 = 0x04020;
pub const E1000_COLC: u32 = 0x04028;
pub const E1000_DC: u32 = 0x04030;
pub const E1000_TNCRS: u32 = 0x04034;
pub const E1000_SEC: u32 = 0x04038;
pub const E1000_CEXTERR: u32 = 0x0403C;
pub const E1000_RLEC: u32 = 0x04040;
pub const E1000_XONRXC: u32 = 0x04048;
pub const E1000_XONTXC: u32 = 0x0404C;
pub const E1000_XOFFRXC: u32 = 0x04050;
pub const E1000_XOFFTXC: u32 = 0x04054;
pub const E1000_FCRUC: u32 = 0x04058;
pub const E1000_PRC64: u32 = 0x0405C;
pub const E1000_PRC127: u32 = 0x04060;
pub const E1000_PRC255: u32 = 0x04064;
pub const E1000_PRC511: u32 = 0x04068;
pub const E1000_PRC1023: u32 = 0x0406C;
pub const E1000_PRC1522: u32 = 0x04070;
pub const E1000_GPRC: u32 = 0x04074;
pub const E1000_BPRC: u32 = 0x04078;
pub const E1000_MPRC: u32 = 0x0407C;
pub const E1000_GPTC: u32 = 0x04080;
pub const E1000_GORCL: u32 = 0x04088;
pub const E1000_GORCH: u32 = 0x0408C;
pub const E1000_GOTCL: u32 = 0x04090;
pub const E1000_GOTCH: u32 = 0x04094;
pub const E1000_RNBC: u32 = 0x040A0;
pub const E1000_RUC: u32 = 0x040A4;
pub const E1000_RFC: u32 = 0x040A8;
pub const E1000_ROC: u32 = 0x040AC;
pub const E1000_RJC: u32 = 0x040B0;
pub const E1000_MGTPRC: u32 = 0x040B4;
pub const E1000_MGTPDC: u32 = 0x040B8;
pub const E1000_MGTPTC: u32 = 0x040BC;
pub const E1000_TORL: u32 = 0x040C0;
pub const E1000_TORH: u32 = 0x040C4;
pub const E1000_TOTL: u32 = 0x040C8;
pub const E1000_TOTH: u32 = 0x040CC;
pub const E1000_TPR: u32 = 0x040D0;
pub const E1000_TPT: u32 = 0x040D4;
pub const E1000_PTC64: u32 = 0x040D8;
pub const E1000_PTC127: u32 = 0x040DC;
pub const E1000_PTC255: u32 = 0x040E0;
pub const E1000_PTC511: u32 = 0x040E4;
pub const E1000_PTC1023: u32 = 0x040E8;
pub const E1000_PTC1522: u32 = 0x040EC;
pub const E1000_MPTC: u32 = 0x040F0;
pub const E1000_BPTC: u32 = 0x040F4;
pub const E1000_TSCTC: u32 = 0x040F8;
pub const E1000_TSCTFC: u32 = 0x040FC;
pub const E1000_IAC: u32 = 0x04100;
pub const E1000_ICRXPTC: u32 = 0x04104;
pub const E1000_ICRXATC: u32 = 0x04108;
pub const E1000_ICTXPTC: u32 = 0x0410C;
pub const E1000_ICTXATC: u32 = 0x04110;
pub const E1000_ICTXQEC: u32 = 0x04118;
pub const E1000_ICTXQMTC: u32 = 0x0411C;
pub const E1000_ICRXDMTC: u32 = 0x04120;
pub const E1000_ICRXOC: u32 = 0x04124;
pub const E1000_RXCSUM: u32 = 0x05000;
pub const E1000_RFCTL: u32 = 0x05008;
pub const E1000_MTA: u32 = 0x05200;
pub const E1000_RA: u32 = 0x05400;
pub const E1000_VFTA: u32 = 0x05600;
pub const E1000_WUC: u32 = 0x05800;
pub const E1000_WUFC: u32 = 0x05808;
pub const E1000_WUS: u32 = 0x05810;
pub const E1000_MANC: u32 = 0x05820;
pub const E1000_IPAV: u32 = 0x05838;
pub const E1000_IP4AT: u32 = 0x05840;
pub const E1000_IP6AT: u32 = 0x05880;
pub const E1000_WUPL: u32 = 0x05900;
pub const E1000_WUPM: u32 = 0x05A00;
pub const E1000_FFLT: u32 = 0x05F00;
pub const E1000_HOST_IF: u32 = 0x08800;
pub const E1000_FFMT: u32 = 0x09000;
pub const E1000_FFVT: u32 = 0x09800;

pub const E1000_KUMCTRLSTA: u32 = 0x00034;
pub const E1000_MDPHYA: u32 = 0x0003C;
pub const E1000_MANC2H: u32 = 0x05860;
pub const E1000_SW_FW_SYNC: u32 = 0x05B5C;

pub const E1000_GCR: u32 = 0x05B00;
pub const E1000_GSCL_1: u32 = 0x05B10;
pub const E1000_GSCL_2: u32 = 0x05B14;
pub const E1000_GSCL_3: u32 = 0x05B18;
pub const E1000_GSCL_4: u32 = 0x05B1C;
pub const E1000_FACTPS: u32 = 0x05B30;
pub const E1000_SWSM: u32 = 0x05B50;
pub const E1000_FWSM: u32 = 0x05B54;
pub const E1000_FFLT_DBG: u32 = 0x05F04;
pub const E1000_HICR: u32 = 0x08F00;

pub const E1000_CPUVEC: u32 = 0x02C10;
pub const E1000_MRQC: u32 = 0x05818;
pub const E1000_RETA: u32 = 0x05C00;
pub const E1000_RSSRK: u32 = 0x05C80;
pub const E1000_RSSIM: u32 = 0x05864;
pub const E1000_RSSIR: u32 = 0x05868;

pub const E1000_82542_CTL_AUX: u32 = E1000_CTL_AUX;
pub const E1000_82542_CTRL: u32 = E1000_CTRL;
pub const E1000_82542_CTRL_DUP: u32 = E1000_CTRL_DUP;
pub const E1000_82542_STATUS: u32 = E1000_STATUS;
pub const E1000_82542_EECD: u32 = E1000_EECD;
pub const E1000_82542_EERD: u32 = E1000_EERD;
pub const E1000_82542_CTRL_EXT: u32 = E1000_CTRL_EXT;
pub const E1000_82542_FLA: u32 = E1000_FLA;
pub const E1000_82542_MDIC: u32 = E1000_MDIC;
pub const E1000_82542_SCTL: u32 = E1000_SCTL;
pub const E1000_82542_FEXTNVM: u32 = E1000_FEXTNVM;
pub const E1000_82542_FCAL: u32 = E1000_FCAL;
pub const E1000_82542_FCAH: u32 = E1000_FCAH;
pub const E1000_82542_FCT: u32 = E1000_FCT;
pub const E1000_82542_VET: u32 = E1000_VET;
pub const E1000_82542_RA: u32 = 0x00040;
pub const E1000_82542_ICR: u32 = E1000_ICR;
pub const E1000_82542_ITR: u32 = E1000_ITR;
pub const E1000_82542_ICS: u32 = E1000_ICS;
pub const E1000_82542_IMS: u32 = E1000_IMS;
pub const E1000_82542_IMC: u32 = E1000_IMC;
pub const E1000_82542_RCTL: u32 = E1000_RCTL;
pub const E1000_82542_RDTR: u32 = 0x00108;
pub const E1000_82542_RDFH: u32 = E1000_RDFH;
pub const E1000_82542_RDFT: u32 = E1000_RDFT;
pub const E1000_82542_RDFHS: u32 = E1000_RDFHS;
pub const E1000_82542_RDFTS: u32 = E1000_RDFTS;
pub const E1000_82542_RDFPC: u32 = E1000_RDFPC;
pub const E1000_82542_RDBAL: u32 = 0x00110;
pub const E1000_82542_RDBAH: u32 = 0x00114;
pub const E1000_82542_RDLEN: u32 = 0x00118;
pub const E1000_82542_RDH: u32 = 0x00120;
pub const E1000_82542_RDT: u32 = 0x00128;
pub const E1000_82542_RDTR0: u32 = E1000_82542_RDTR;
pub const E1000_82542_RDBAL0: u32 = E1000_82542_RDBAL;
pub const E1000_82542_RDBAH0: u32 = E1000_82542_RDBAH;
pub const E1000_82542_RDLEN0: u32 = E1000_82542_RDLEN;
pub const E1000_82542_RDH0: u32 = E1000_82542_RDH;
pub const E1000_82542_RDT0: u32 = E1000_82542_RDT;

pub const E1000_82542_RDBAH3: u32 = 0x02B04;
pub const E1000_82542_RDBAL3: u32 = 0x02B00;
pub const E1000_82542_RDLEN3: u32 = 0x02B08;
pub const E1000_82542_RDH3: u32 = 0x02B10;
pub const E1000_82542_RDT3: u32 = 0x02B18;
pub const E1000_82542_RDBAL2: u32 = 0x02A00;
pub const E1000_82542_RDBAH2: u32 = 0x02A04;
pub const E1000_82542_RDLEN2: u32 = 0x02A08;
pub const E1000_82542_RDH2: u32 = 0x02A10;
pub const E1000_82542_RDT2: u32 = 0x02A18;
pub const E1000_82542_RDTR1: u32 = 0x00130;
pub const E1000_82542_RDBAL1: u32 = 0x00138;
pub const E1000_82542_RDBAH1: u32 = 0x0013C;
pub const E1000_82542_RDLEN1: u32 = 0x00140;
pub const E1000_82542_RDH1: u32 = 0x00148;
pub const E1000_82542_RDT1: u32 = 0x00150;
pub const E1000_82542_FCRTH: u32 = 0x00160;
pub const E1000_82542_FCRTL: u32 = 0x00168;
pub const E1000_82542_FCTTV: u32 = E1000_FCTTV;
pub const E1000_82542_TXCW: u32 = E1000_TXCW;
pub const E1000_82542_RXCW: u32 = E1000_RXCW;
pub const E1000_82542_MTA: u32 = 0x00200;
pub const E1000_82542_TCTL: u32 = E1000_TCTL;
pub const E1000_82542_TCTL_EXT: u32 = E1000_TCTL_EXT;
pub const E1000_82542_TIPG: u32 = E1000_TIPG;
pub const E1000_82542_TDBAL: u32 = 0x00420;
pub const E1000_82542_TDBAH: u32 = 0x00424;
pub const E1000_82542_TDLEN: u32 = 0x00428;
pub const E1000_82542_TDH: u32 = 0x00430;
pub const E1000_82542_TDT: u32 = 0x00438;
pub const E1000_82542_TIDV: u32 = 0x00440;
pub const E1000_82542_TBT: u32 = E1000_TBT;
pub const E1000_82542_AIT: u32 = E1000_AIT;
pub const E1000_82542_VFTA: u32 = 0x00600;
pub const E1000_82542_LEDCTL: u32 = E1000_LEDCTL;
pub const E1000_82542_PBA: u32 = E1000_PBA;
pub const E1000_82542_PBS: u32 = E1000_PBS;
pub const E1000_82542_EEMNGCTL: u32 = E1000_EEMNGCTL;
pub const E1000_82542_EEARBC: u32 = E1000_EEARBC;
pub const E1000_82542_FLASHT: u32 = E1000_FLASHT;
pub const E1000_82542_EEWR: u32 = E1000_EEWR;
pub const E1000_82542_FLSWCTL: u32 = E1000_FLSWCTL;
pub const E1000_82542_FLSWDATA: u32 = E1000_FLSWDATA;
pub const E1000_82542_FLSWCNT: u32 = E1000_FLSWCNT;
pub const E1000_82542_FLOP: u32 = E1000_FLOP;
pub const E1000_82542_EXTCNF_CTRL: u32 = E1000_EXTCNF_CTRL;
pub const E1000_82542_EXTCNF_SIZE: u32 = E1000_EXTCNF_SIZE;
pub const E1000_82542_PHY_CTRL: u32 = E1000_PHY_CTRL;
pub const E1000_82542_ERT: u32 = E1000_ERT;
pub const E1000_82542_RXDCTL: u32 = E1000_RXDCTL;
pub const E1000_82542_RXDCTL1: u32 = E1000_RXDCTL1;
pub const E1000_82542_RADV: u32 = E1000_RADV;
pub const E1000_82542_RSRPD: u32 = E1000_RSRPD;
pub const E1000_82542_TXDMAC: u32 = E1000_TXDMAC;
pub const E1000_82542_KABGTXD: u32 = E1000_KABGTXD;
pub const E1000_82542_TDFHS: u32 = E1000_TDFHS;
pub const E1000_82542_TDFTS: u32 = E1000_TDFTS;
pub const E1000_82542_TDFPC: u32 = E1000_TDFPC;
pub const E1000_82542_TXDCTL: u32 = E1000_TXDCTL;
pub const E1000_82542_TADV: u32 = E1000_TADV;
pub const E1000_82542_TSPMT: u32 = E1000_TSPMT;
pub const E1000_82542_CRCERRS: u32 = E1000_CRCERRS;
pub const E1000_82542_ALGNERRC: u32 = E1000_ALGNERRC;
pub const E1000_82542_SYMERRS: u32 = E1000_SYMERRS;
pub const E1000_82542_RXERRC: u32 = E1000_RXERRC;
pub const E1000_82542_MPC: u32 = E1000_MPC;
pub const E1000_82542_SCC: u32 = E1000_SCC;
pub const E1000_82542_ECOL: u32 = E1000_ECOL;
pub const E1000_82542_MCC: u32 = E1000_MCC;
pub const E1000_82542_LATECOL: u32 = E1000_LATECOL;
pub const E1000_82542_COLC: u32 = E1000_COLC;
pub const E1000_82542_DC: u32 = E1000_DC;
pub const E1000_82542_TNCRS: u32 = E1000_TNCRS;
pub const E1000_82542_SEC: u32 = E1000_SEC;
pub const E1000_82542_CEXTERR: u32 = E1000_CEXTERR;
pub const E1000_82542_RLEC: u32 = E1000_RLEC;
pub const E1000_82542_XONRXC: u32 = E1000_XONRXC;
pub const E1000_82542_XONTXC: u32 = E1000_XONTXC;
pub const E1000_82542_XOFFRXC: u32 = E1000_XOFFRXC;
pub const E1000_82542_XOFFTXC: u32 = E1000_XOFFTXC;
pub const E1000_82542_FCRUC: u32 = E1000_FCRUC;
pub const E1000_82542_PRC64: u32 = E1000_PRC64;
pub const E1000_82542_PRC127: u32 = E1000_PRC127;
pub const E1000_82542_PRC255: u32 = E1000_PRC255;
pub const E1000_82542_PRC511: u32 = E1000_PRC511;
pub const E1000_82542_PRC1023: u32 = E1000_PRC1023;
pub const E1000_82542_PRC1522: u32 = E1000_PRC1522;
pub const E1000_82542_GPRC: u32 = E1000_GPRC;
pub const E1000_82542_BPRC: u32 = E1000_BPRC;
pub const E1000_82542_MPRC: u32 = E1000_MPRC;
pub const E1000_82542_GPTC: u32 = E1000_GPTC;
pub const E1000_82542_GORCL: u32 = E1000_GORCL;
pub const E1000_82542_GORCH: u32 = E1000_GORCH;
pub const E1000_82542_GOTCL: u32 = E1000_GOTCL;
pub const E1000_82542_GOTCH: u32 = E1000_GOTCH;
pub const E1000_82542_RNBC: u32 = E1000_RNBC;
pub const E1000_82542_RUC: u32 = E1000_RUC;
pub const E1000_82542_RFC: u32 = E1000_RFC;
pub const E1000_82542_ROC: u32 = E1000_ROC;
pub const E1000_82542_RJC: u32 = E1000_RJC;
pub const E1000_82542_MGTPRC: u32 = E1000_MGTPRC;
pub const E1000_82542_MGTPDC: u32 = E1000_MGTPDC;
pub const E1000_82542_MGTPTC: u32 = E1000_MGTPTC;
pub const E1000_82542_TORL: u32 = E1000_TORL;
pub const E1000_82542_TORH: u32 = E1000_TORH;
pub const E1000_82542_TOTL: u32 = E1000_TOTL;
pub const E1000_82542_TOTH: u32 = E1000_TOTH;
pub const E1000_82542_TPR: u32 = E1000_TPR;
pub const E1000_82542_TPT: u32 = E1000_TPT;
pub const E1000_82542_PTC64: u32 = E1000_PTC64;
pub const E1000_82542_PTC127: u32 = E1000_PTC127;
pub const E1000_82542_PTC255: u32 = E1000_PTC255;
pub const E1000_82542_PTC511: u32 = E1000_PTC511;
pub const E1000_82542_PTC1023: u32 = E1000_PTC1023;
pub const E1000_82542_PTC1522: u32 = E1000_PTC1522;
pub const E1000_82542_MPTC: u32 = E1000_MPTC;
pub const E1000_82542_BPTC: u32 = E1000_BPTC;
pub const E1000_82542_TSCTC: u32 = E1000_TSCTC;
pub const E1000_82542_TSCTFC: u32 = E1000_TSCTFC;
pub const E1000_82542_RXCSUM: u32 = E1000_RXCSUM;
pub const E1000_82542_WUC: u32 = E1000_WUC;
pub const E1000_82542_WUFC: u32 = E1000_WUFC;
pub const E1000_82542_WUS: u32 = E1000_WUS;
pub const E1000_82542_MANC: u32 = E1000_MANC;
pub const E1000_82542_IPAV: u32 = E1000_IPAV;
pub const E1000_82542_IP4AT: u32 = E1000_IP4AT;
pub const E1000_82542_IP6AT: u32 = E1000_IP6AT;
pub const E1000_82542_WUPL: u32 = E1000_WUPL;
pub const E1000_82542_WUPM: u32 = E1000_WUPM;
pub const E1000_82542_FFLT: u32 = E1000_FFLT;
pub const E1000_82542_TDFH: u32 = 0x08010;
pub const E1000_82542_TDFT: u32 = 0x08018;
pub const E1000_82542_FFMT: u32 = E1000_FFMT;
pub const E1000_82542_FFVT: u32 = E1000_FFVT;
pub const E1000_82542_HOST_IF: u32 = E1000_HOST_IF;
pub const E1000_82542_IAM: u32 = E1000_IAM;
pub const E1000_82542_PSRCTL: u32 = E1000_PSRCTL;
pub const E1000_82542_RAID: u32 = E1000_RAID;
pub const E1000_82542_TARC0: u32 = E1000_TARC0;
pub const E1000_82542_TDBAL1: u32 = E1000_TDBAL1;
pub const E1000_82542_TDBAH1: u32 = E1000_TDBAH1;
pub const E1000_82542_TDLEN1: u32 = E1000_TDLEN1;
pub const E1000_82542_TDH1: u32 = E1000_TDH1;
pub const E1000_82542_TDT1: u32 = E1000_TDT1;
pub const E1000_82542_TXDCTL1: u32 = E1000_TXDCTL1;
pub const E1000_82542_TARC1: u32 = E1000_TARC1;
pub const E1000_82542_RFCTL: u32 = E1000_RFCTL;
pub const E1000_82542_GCR: u32 = E1000_GCR;
pub const E1000_82542_GSCL_1: u32 = E1000_GSCL_1;
pub const E1000_82542_GSCL_2: u32 = E1000_GSCL_2;
pub const E1000_82542_GSCL_3: u32 = E1000_GSCL_3;
pub const E1000_82542_GSCL_4: u32 = E1000_GSCL_4;
pub const E1000_82542_FACTPS: u32 = E1000_FACTPS;
pub const E1000_82542_SWSM: u32 = E1000_SWSM;
pub const E1000_82542_FWSM: u32 = E1000_FWSM;
pub const E1000_82542_FFLT_DBG: u32 = E1000_FFLT_DBG;
pub const E1000_82542_IAC: u32 = E1000_IAC;
pub const E1000_82542_ICRXPTC: u32 = E1000_ICRXPTC;
pub const E1000_82542_ICRXATC: u32 = E1000_ICRXATC;
pub const E1000_82542_ICTXPTC: u32 = E1000_ICTXPTC;
pub const E1000_82542_ICTXATC: u32 = E1000_ICTXATC;
pub const E1000_82542_ICTXQEC: u32 = E1000_ICTXQEC;
pub const E1000_82542_ICTXQMTC: u32 = E1000_ICTXQMTC;
pub const E1000_82542_ICRXDMTC: u32 = E1000_ICRXDMTC;
pub const E1000_82542_ICRXOC: u32 = E1000_ICRXOC;
pub const E1000_82542_HICR: u32 = E1000_HICR;

pub const E1000_82542_CPUVEC: u32 = E1000_CPUVEC;
pub const E1000_82542_MRQC: u32 = E1000_MRQC;
pub const E1000_82542_RETA: u32 = E1000_RETA;
pub const E1000_82542_RSSRK: u32 = E1000_RSSRK;
pub const E1000_82542_RSSIM: u32 = E1000_RSSIM;
pub const E1000_82542_RSSIR: u32 = E1000_RSSIR;
pub const E1000_82542_KUMCTRLSTA: u32 = E1000_KUMCTRLSTA;
pub const E1000_82542_SW_FW_SYNC: u32 = E1000_SW_FW_SYNC;

pub const E1000_EEPROM_SWDPIN0: u32 = 0x0001;
pub const E1000_EEPROM_LED_LOGIC: u32 = 0x0020;
pub const E1000_EEPROM_RW_REG_DATA: u32 = 16;
pub const E1000_EEPROM_RW_REG_DONE: u32 = 2;
pub const E1000_EEPROM_RW_REG_START: u32 = 1;
pub const E1000_EEPROM_RW_ADDR_SHIFT: u32 = 2;
pub const E1000_EEPROM_POLL_WRITE: u32 = 1;
pub const E1000_EEPROM_POLL_READ: u32 = 0;

pub const E1000_CTRL_FD: u32 = 0x00000001;
pub const E1000_CTRL_BEM: u32 = 0x00000002;
pub const E1000_CTRL_PRIOR: u32 = 0x00000004;
pub const E1000_CTRL_GIO_MASTER_DISABLE: u32 = 0x00000004;
pub const E1000_CTRL_LRST: u32 = 0x00000008;
pub const E1000_CTRL_TME: u32 = 0x00000010;
pub const E1000_CTRL_SLE: u32 = 0x00000020;
pub const E1000_CTRL_ASDE: u32 = 0x00000020;
pub const E1000_CTRL_SLU: u32 = 0x00000040;
pub const E1000_CTRL_ILOS: u32 = 0x00000080;
pub const E1000_CTRL_SPD_SEL: u32 = 0x00000300;
pub const E1000_CTRL_SPD_10: u32 = 0x00000000;
pub const E1000_CTRL_SPD_100: u32 = 0x00000100;
pub const E1000_CTRL_SPD_1000: u32 = 0x00000200;
pub const E1000_CTRL_BEM32: u32 = 0x00000400;
pub const E1000_CTRL_FRCSPD: u32 = 0x00000800;
pub const E1000_CTRL_FRCDPX: u32 = 0x00001000;
pub const E1000_CTRL_D_UD_EN: u32 = 0x00002000;
pub const E1000_CTRL_D_UD_POLARITY: u32 = 0x00004000;
pub const E1000_CTRL_FORCE_PHY_RESET: u32 = 0x00008000;
pub const E1000_CTRL_EXT_LINK_EN: u32 = 0x00010000;
pub const E1000_CTRL_SWDPIN0: u32 = 0x00040000;
pub const E1000_CTRL_SWDPIN1: u32 = 0x00080000;
pub const E1000_CTRL_SWDPIN2: u32 = 0x00100000;
pub const E1000_CTRL_SWDPIN3: u32 = 0x00200000;
pub const E1000_CTRL_SWDPIO0: u32 = 0x00400000;
pub const E1000_CTRL_SWDPIO1: u32 = 0x00800000;
pub const E1000_CTRL_SWDPIO2: u32 = 0x01000000;
pub const E1000_CTRL_SWDPIO3: u32 = 0x02000000;
pub const E1000_CTRL_RST: u32 = 0x04000000;
pub const E1000_CTRL_RFCE: u32 = 0x08000000;
pub const E1000_CTRL_TFCE: u32 = 0x10000000;
pub const E1000_CTRL_RTE: u32 = 0x20000000;
pub const E1000_CTRL_VME: u32 = 0x40000000;
pub const E1000_CTRL_PHY_RST: u32 = 0x80000000;
pub const E1000_CTRL_SW2FW_INT: u32 = 0x02000000;

pub const E1000_STATUS_FD: u32 = 0x00000001;
pub const E1000_STATUS_LU: u32 = 0x00000002;
pub const E1000_STATUS_FUNC_MASK: u32 = 0x0000000C;
pub const E1000_STATUS_FUNC_SHIFT: u32 = 2;
pub const E1000_STATUS_FUNC_0: u32 = 0x00000000;
pub const E1000_STATUS_FUNC_1: u32 = 0x00000004;
pub const E1000_STATUS_TXOFF: u32 = 0x00000010;
pub const E1000_STATUS_TBIMODE: u32 = 0x00000020;
pub const E1000_STATUS_SPEED_MASK: u32 = 0x000000C0;
pub const E1000_STATUS_SPEED_10: u32 = 0x00000000;
pub const E1000_STATUS_SPEED_100: u32 = 0x00000040;
pub const E1000_STATUS_SPEED_1000: u32 = 0x00000080;
pub const E1000_STATUS_LAN_INIT_DONE: u32 = 0x00000200;
pub const E1000_STATUS_ASDV: u32 = 0x00000300;
pub const E1000_STATUS_DOCK_CI: u32 = 0x00000800;
pub const E1000_STATUS_GIO_MASTER_ENABLE: u32 = 0x00080000;
pub const E1000_STATUS_MTXCKOK: u32 = 0x00000400;
pub const E1000_STATUS_PCI66: u32 = 0x00000800;
pub const E1000_STATUS_BUint64_t: u32 = 0x00001000;
pub const E1000_STATUS_PCIX_MODE: u32 = 0x00002000;
pub const E1000_STATUS_PCIX_SPEED: u32 = 0x0000C000;
pub const E1000_STATUS_BMC_SKU_0: u32 = 0x00100000;
pub const E1000_STATUS_BMC_SKU_1: u32 = 0x00200000;
pub const E1000_STATUS_BMC_SKU_2: u32 = 0x00400000;
pub const E1000_STATUS_BMC_CRYPTO: u32 = 0x00800000;
pub const E1000_STATUS_BMC_LITE: u32 = 0x01000000;
pub const E1000_STATUS_RGMII_ENABLE: u32 = 0x02000000;
pub const E1000_STATUS_FUSE_8: u32 = 0x04000000;
pub const E1000_STATUS_FUSE_9: u32 = 0x08000000;
pub const E1000_STATUS_SERDES0_DIS: u32 = 0x10000000;
pub const E1000_STATUS_SERDES1_DIS: u32 = 0x20000000;

pub const E1000_STATUS_PCIX_SPEED_66: u32 = 0x00000000;
pub const E1000_STATUS_PCIX_SPEED_100: u32 = 0x00004000;
pub const E1000_STATUS_PCIX_SPEED_133: u32 = 0x00008000;

pub const E1000_EECD_SK: u32 = 0x00000001;
pub const E1000_EECD_CS: u32 = 0x00000002;
pub const E1000_EECD_DI: u32 = 0x00000004;
pub const E1000_EECD_DO: u32 = 0x00000008;
pub const E1000_EECD_FWE_MASK: u32 = 0x00000030;
pub const E1000_EECD_FWE_DIS: u32 = 0x00000010;
pub const E1000_EECD_FWE_EN: u32 = 0x00000020;
pub const E1000_EECD_FWE_SHIFT: u32 = 4;
pub const E1000_EECD_REQ: u32 = 0x00000040;
pub const E1000_EECD_GNT: u32 = 0x00000080;
pub const E1000_EECD_PRES: u32 = 0x00000100;
pub const E1000_EECD_SIZE: u32 = 0x00000200;
pub const E1000_EECD_ADDR_BITS: u32 = 0x00000400;
pub const E1000_EECD_TYPE: u32 = 0x00002000;

pub const E1000_EEPROM_GRANT_ATTEMPTS: u32 = 1000;

pub const E1000_EECD_AUTO_RD: u32 = 0x00000200;
pub const E1000_EECD_SIZE_EX_MASK: u32 = 0x00007800;
pub const E1000_EECD_SIZE_EX_SHIFT: u32 = 11;
pub const E1000_EECD_NVADDS: u32 = 0x00018000;
pub const E1000_EECD_SELSHAD: u32 = 0x00020000;
pub const E1000_EECD_INITSRAM: u32 = 0x00040000;
pub const E1000_EECD_FLUPD: u32 = 0x00080000;
pub const E1000_EECD_AUPDEN: u32 = 0x00100000;
pub const E1000_EECD_SHADV: u32 = 0x00200000;
pub const E1000_EECD_SEC1VAL: u32 = 0x00400000;
pub const E1000_EECD_SECVAL_SHIFT: u32 = 22;
pub const E1000_STM_OPCODE: u32 = 0xDB00;
pub const E1000_HICR_FW_RESET: u32 = 0xC0;

pub const E1000_SHADOW_RAM_WORDS: u32 = 2048;
pub const E1000_ICH_NVM_SIG_WORD: u32 = 0x13;
pub const E1000_ICH_NVM_SIG_MASK: u32 = 0xC0;

pub const E1000_EERD_START: u32 = 0x00000001;
pub const E1000_EERD_DONE: u32 = 0x00000010;
pub const E1000_EERD_ADDR_SHIFT: u32 = 8;
pub const E1000_EERD_ADDR_MASK: u32 = 0x0000FF00;
pub const E1000_EERD_DATA_SHIFT: u32 = 16;
pub const E1000_EERD_DATA_MASK: u32 = 0xFFFF0000;

pub const EEPROM_STATUS_RDY_SPI: u32 = 0x01;
pub const EEPROM_STATUS_WEN_SPI: u32 = 0x02;
pub const EEPROM_STATUS_BP0_SPI: u32 = 0x04;
pub const EEPROM_STATUS_BP1_SPI: u32 = 0x08;
pub const EEPROM_STATUS_WPEN_SPI: u32 = 0x80;

pub const E1000_CTRL_EXT_GPI0_EN: u32 = 0x00000001;
pub const E1000_CTRL_EXT_GPI1_EN: u32 = 0x00000002;
pub const E1000_CTRL_EXT_PHYINT_EN: u32 = E1000_CTRL_EXT_GPI1_EN;
pub const E1000_CTRL_EXT_GPI2_EN: u32 = 0x00000004;
pub const E1000_CTRL_EXT_GPI3_EN: u32 = 0x00000008;
pub const E1000_CTRL_EXT_SDP4_DATA: u32 = 0x00000010;
pub const E1000_CTRL_EXT_SDP5_DATA: u32 = 0x00000020;
pub const E1000_CTRL_EXT_PHY_INT: u32 = E1000_CTRL_EXT_SDP5_DATA;
pub const E1000_CTRL_EXT_SDP6_DATA: u32 = 0x00000040;
pub const E1000_CTRL_EXT_SDP7_DATA: u32 = 0x00000080;
pub const E1000_CTRL_EXT_SDP4_DIR: u32 = 0x00000100;
pub const E1000_CTRL_EXT_SDP5_DIR: u32 = 0x00000200;
pub const E1000_CTRL_EXT_SDP6_DIR: u32 = 0x00000400;
pub const E1000_CTRL_EXT_SDP7_DIR: u32 = 0x00000800;
pub const E1000_CTRL_EXT_ASDCHK: u32 = 0x00001000;
pub const E1000_CTRL_EXT_EE_RST: u32 = 0x00002000;
pub const E1000_CTRL_EXT_IPS: u32 = 0x00004000;
pub const E1000_CTRL_EXT_SPD_BYPS: u32 = 0x00008000;
pub const E1000_CTRL_EXT_RO_DIS: u32 = 0x00020000;
pub const E1000_CTRL_EXT_LINK_MODE_MASK: u32 = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_GMII: u32 = 0x00000000;
pub const E1000_CTRL_EXT_LINK_MODE_TBI: u32 = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_KMRN: u32 = 0x00000000;
pub const E1000_CTRL_EXT_LINK_MODE_SERDES: u32 = 0x00C00000;
pub const E1000_CTRL_EXT_LINK_MODE_SGMII: u32 = 0x00800000;
pub const E1000_CTRL_EXT_WR_WMARK_MASK: u32 = 0x03000000;
pub const E1000_CTRL_EXT_WR_WMARK_256: u32 = 0x00000000;
pub const E1000_CTRL_EXT_WR_WMARK_320: u32 = 0x01000000;
pub const E1000_CTRL_EXT_WR_WMARK_384: u32 = 0x02000000;
pub const E1000_CTRL_EXT_WR_WMARK_448: u32 = 0x03000000;
pub const E1000_CTRL_EXT_DRV_LOAD: u32 = 0x10000000;
pub const E1000_CTRL_EXT_IAME: u32 = 0x08000000;
pub const E1000_CTRL_EXT_INT_TIMER_CLR: u32 = 0x20000000;
pub const E1000_CRTL_EXT_PB_PAREN: u32 = 0x01000000;
pub const E1000_CTRL_EXT_DF_PAREN: u32 = 0x02000000;
pub const E1000_CTRL_EXT_GHOST_PAREN: u32 = 0x40000000;

pub const E1000_MDIC_DATA_MASK: u32 = 0x0000FFFF;
pub const E1000_MDIC_REG_MASK: u32 = 0x001F0000;
pub const E1000_MDIC_REG_SHIFT: u32 = 16;
pub const E1000_MDIC_PHY_MASK: u32 = 0x03E00000;
pub const E1000_MDIC_PHY_SHIFT: u32 = 21;
pub const E1000_MDIC_OP_WRITE: u32 = 0x04000000;
pub const E1000_MDIC_OP_READ: u32 = 0x08000000;
pub const E1000_MDIC_READY: u32 = 0x10000000;
pub const E1000_MDIC_INT_EN: u32 = 0x20000000;
pub const E1000_MDIC_ERROR: u32 = 0x40000000;

pub const INTEL_CE_GBE_MDIC_OP_WRITE: u32 = 0x04000000;
pub const INTEL_CE_GBE_MDIC_OP_READ: u32 = 0x00000000;
pub const INTEL_CE_GBE_MDIC_GO: u32 = 0x80000000;
pub const INTEL_CE_GBE_MDIC_READ_ERROR: u32 = 0x80000000;

pub const E1000_KUMCTRLSTA_MASK: u32 = 0x0000FFFF;
pub const E1000_KUMCTRLSTA_OFFSET: u32 = 0x001F0000;
pub const E1000_KUMCTRLSTA_OFFSET_SHIFT: u32 = 16;
pub const E1000_KUMCTRLSTA_REN: u32 = 0x00200000;

pub const E1000_KUMCTRLSTA_OFFSET_FIFO_CTRL: u32 = 0x00000000;
pub const E1000_KUMCTRLSTA_OFFSET_CTRL: u32 = 0x00000001;
pub const E1000_KUMCTRLSTA_OFFSET_INB_CTRL: u32 = 0x00000002;
pub const E1000_KUMCTRLSTA_OFFSET_DIAG: u32 = 0x00000003;
pub const E1000_KUMCTRLSTA_OFFSET_TIMEOUTS: u32 = 0x00000004;
pub const E1000_KUMCTRLSTA_OFFSET_INB_PARAM: u32 = 0x00000009;
pub const E1000_KUMCTRLSTA_OFFSET_HD_CTRL: u32 = 0x00000010;
pub const E1000_KUMCTRLSTA_OFFSET_M2P_SERDES: u32 = 0x0000001E;
pub const E1000_KUMCTRLSTA_OFFSET_M2P_MODES: u32 = 0x0000001F;

pub const E1000_KUMCTRLSTA_FIFO_CTRL_RX_BYPASS: u32 = 0x00000008;
pub const E1000_KUMCTRLSTA_FIFO_CTRL_TX_BYPASS: u32 = 0x00000800;

pub const E1000_KUMCTRLSTA_INB_CTRL_LINK_STATUS_TX_TIMEOUT_DEFAULT: u32 = 0x00000500;
pub const E1000_KUMCTRLSTA_INB_CTRL_DIS_PADDING: u32 = 0x00000010;

pub const E1000_KUMCTRLSTA_HD_CTRL_10_100_DEFAULT: u32 = 0x00000004;
pub const E1000_KUMCTRLSTA_HD_CTRL_1000_DEFAULT: u32 = 0x00000000;

pub const E1000_KUMCTRLSTA_OFFSET_K0S_CTRL: u32 = 0x0000001E;

pub const E1000_KUMCTRLSTA_DIAG_FELPBK: u32 = 0x2000;
pub const E1000_KUMCTRLSTA_DIAG_NELPBK: u32 = 0x1000;

pub const E1000_KUMCTRLSTA_K0S_100_EN: u32 = 0x2000;
pub const E1000_KUMCTRLSTA_K0S_GBE_EN: u32 = 0x1000;
pub const E1000_KUMCTRLSTA_K0S_ENTRY_LATENCY_MASK: u32 = 0x0003;

pub const E1000_KABGTXD_BGSQLBIAS: u32 = 0x00050000;

pub const E1000_PHY_CTRL_SPD_EN: u32 = 0x00000001;
pub const E1000_PHY_CTRL_D0A_LPLU: u32 = 0x00000002;
pub const E1000_PHY_CTRL_NOND0A_LPLU: u32 = 0x00000004;
pub const E1000_PHY_CTRL_NOND0A_GBE_DISABLE: u32 = 0x00000008;
pub const E1000_PHY_CTRL_GBE_DISABLE: u32 = 0x00000040;
pub const E1000_PHY_CTRL_B2B_EN: u32 = 0x00000080;

pub const E1000_LEDCTL_LED0_MODE_MASK: u32 = 0x0000000F;
pub const E1000_LEDCTL_LED0_MODE_SHIFT: u32 = 0;
pub const E1000_LEDCTL_LED0_BLINK_RATE: u32 = 0x0000020;
pub const E1000_LEDCTL_LED0_IVRT: u32 = 0x00000040;
pub const E1000_LEDCTL_LED0_BLINK: u32 = 0x00000080;
pub const E1000_LEDCTL_LED1_MODE_MASK: u32 = 0x00000F00;
pub const E1000_LEDCTL_LED1_MODE_SHIFT: u32 = 8;
pub const E1000_LEDCTL_LED1_BLINK_RATE: u32 = 0x0002000;
pub const E1000_LEDCTL_LED1_IVRT: u32 = 0x00004000;
pub const E1000_LEDCTL_LED1_BLINK: u32 = 0x00008000;
pub const E1000_LEDCTL_LED2_MODE_MASK: u32 = 0x000F0000;
pub const E1000_LEDCTL_LED2_MODE_SHIFT: u32 = 16;
pub const E1000_LEDCTL_LED2_BLINK_RATE: u32 = 0x00200000;
pub const E1000_LEDCTL_LED2_IVRT: u32 = 0x00400000;
pub const E1000_LEDCTL_LED2_BLINK: u32 = 0x00800000;
pub const E1000_LEDCTL_LED3_MODE_MASK: u32 = 0x0F000000;
pub const E1000_LEDCTL_LED3_MODE_SHIFT: u32 = 24;
pub const E1000_LEDCTL_LED3_BLINK_RATE: u32 = 0x20000000;
pub const E1000_LEDCTL_LED3_IVRT: u32 = 0x40000000;
pub const E1000_LEDCTL_LED3_BLINK: u32 = 0x80000000;

pub const E1000_LEDCTL_MODE_LINK_10_1000: u32 = 0x0;
pub const E1000_LEDCTL_MODE_LINK_100_1000: u32 = 0x1;
pub const E1000_LEDCTL_MODE_LINK_UP: u32 = 0x2;
pub const E1000_LEDCTL_MODE_ACTIVITY: u32 = 0x3;
pub const E1000_LEDCTL_MODE_LINK_ACTIVITY: u32 = 0x4;
pub const E1000_LEDCTL_MODE_LINK_10: u32 = 0x5;
pub const E1000_LEDCTL_MODE_LINK_100: u32 = 0x6;
pub const E1000_LEDCTL_MODE_LINK_1000: u32 = 0x7;
pub const E1000_LEDCTL_MODE_PCIX_MODE: u32 = 0x8;
pub const E1000_LEDCTL_MODE_FULL_DUPLEX: u32 = 0x9;
pub const E1000_LEDCTL_MODE_COLLISION: u32 = 0xA;
pub const E1000_LEDCTL_MODE_BUS_SPEED: u32 = 0xB;
pub const E1000_LEDCTL_MODE_BUS_SIZE: u32 = 0xC;
pub const E1000_LEDCTL_MODE_PAUSED: u32 = 0xD;
pub const E1000_LEDCTL_MODE_LED_ON: u32 = 0xE;
pub const E1000_LEDCTL_MODE_LED_OFF: u32 = 0xF;

pub const E1000_RAH_AV: u32 = 0x80000000;

pub const E1000_ICR_TXDW: u32 = 0x00000001;
pub const E1000_ICR_TXQE: u32 = 0x00000002;
pub const E1000_ICR_LSC: u32 = 0x00000004;
pub const E1000_ICR_RXSEQ: u32 = 0x00000008;
pub const E1000_ICR_RXDMT0: u32 = 0x00000010;
pub const E1000_ICR_RXO: u32 = 0x00000040;
pub const E1000_ICR_RXT0: u32 = 0x00000080;
pub const E1000_ICR_MDAC: u32 = 0x00000200;
pub const E1000_ICR_RXCFG: u32 = 0x00000400;
pub const E1000_ICR_GPI_EN0: u32 = 0x00000800;
pub const E1000_ICR_GPI_EN1: u32 = 0x00001000;
pub const E1000_ICR_GPI_EN2: u32 = 0x00002000;
pub const E1000_ICR_GPI_EN3: u32 = 0x00004000;
pub const E1000_ICR_TXD_LOW: u32 = 0x00008000;
pub const E1000_ICR_SRPD: u32 = 0x00010000;
pub const E1000_ICR_ACK: u32 = 0x00020000;
pub const E1000_ICR_MNG: u32 = 0x00040000;
pub const E1000_ICR_DOCK: u32 = 0x00080000;
pub const E1000_ICR_INT_ASSERTED: u32 = 0x80000000;
pub const E1000_ICR_RXD_FIFO_PAR0: u32 = 0x00100000;
pub const E1000_ICR_TXD_FIFO_PAR0: u32 = 0x00200000;
pub const E1000_ICR_HOST_ARB_PAR: u32 = 0x00400000;
pub const E1000_ICR_PB_PAR: u32 = 0x00800000;
pub const E1000_ICR_RXD_FIFO_PAR1: u32 = 0x01000000;
pub const E1000_ICR_TXD_FIFO_PAR1: u32 = 0x02000000;
pub const E1000_ICR_ALL_PARITY: u32 = 0x03F00000;
pub const E1000_ICR_DSW: u32 = 0x00000020;
pub const E1000_ICR_PHYINT: u32 = 0x00001000;
pub const E1000_ICR_EPRST: u32 = 0x00100000;

pub const E1000_ICS_TXDW: u32 = E1000_ICR_TXDW;
pub const E1000_ICS_TXQE: u32 = E1000_ICR_TXQE;
pub const E1000_ICS_LSC: u32 = E1000_ICR_LSC;
pub const E1000_ICS_RXSEQ: u32 = E1000_ICR_RXSEQ;
pub const E1000_ICS_RXDMT0: u32 = E1000_ICR_RXDMT0;
pub const E1000_ICS_RXO: u32 = E1000_ICR_RXO;
pub const E1000_ICS_RXT0: u32 = E1000_ICR_RXT0;
pub const E1000_ICS_MDAC: u32 = E1000_ICR_MDAC;
pub const E1000_ICS_RXCFG: u32 = E1000_ICR_RXCFG;
pub const E1000_ICS_GPI_EN0: u32 = E1000_ICR_GPI_EN0;
pub const E1000_ICS_GPI_EN1: u32 = E1000_ICR_GPI_EN1;
pub const E1000_ICS_GPI_EN2: u32 = E1000_ICR_GPI_EN2;
pub const E1000_ICS_GPI_EN3: u32 = E1000_ICR_GPI_EN3;
pub const E1000_ICS_TXD_LOW: u32 = E1000_ICR_TXD_LOW;
pub const E1000_ICS_SRPD: u32 = E1000_ICR_SRPD;
pub const E1000_ICS_ACK: u32 = E1000_ICR_ACK;
pub const E1000_ICS_MNG: u32 = E1000_ICR_MNG;
pub const E1000_ICS_DOCK: u32 = E1000_ICR_DOCK;
pub const E1000_ICS_RXD_FIFO_PAR0: u32 = E1000_ICR_RXD_FIFO_PAR0;
pub const E1000_ICS_TXD_FIFO_PAR0: u32 = E1000_ICR_TXD_FIFO_PAR0;
pub const E1000_ICS_HOST_ARB_PAR: u32 = E1000_ICR_HOST_ARB_PAR;
pub const E1000_ICS_PB_PAR: u32 = E1000_ICR_PB_PAR;
pub const E1000_ICS_RXD_FIFO_PAR1: u32 = E1000_ICR_RXD_FIFO_PAR1;
pub const E1000_ICS_TXD_FIFO_PAR1: u32 = E1000_ICR_TXD_FIFO_PAR1;
pub const E1000_ICS_DSW: u32 = E1000_ICR_DSW;
pub const E1000_ICS_PHYINT: u32 = E1000_ICR_PHYINT;
pub const E1000_ICS_EPRST: u32 = E1000_ICR_EPRST;

pub const E1000_IMS_TXDW: u32 = E1000_ICR_TXDW;
pub const E1000_IMS_TXQE: u32 = E1000_ICR_TXQE;
pub const E1000_IMS_LSC: u32 = E1000_ICR_LSC;
pub const E1000_IMS_RXSEQ: u32 = E1000_ICR_RXSEQ;
pub const E1000_IMS_RXDMT0: u32 = E1000_ICR_RXDMT0;
pub const E1000_IMS_RXO: u32 = E1000_ICR_RXO;
pub const E1000_IMS_RXT0: u32 = E1000_ICR_RXT0;
pub const E1000_IMS_MDAC: u32 = E1000_ICR_MDAC;
pub const E1000_IMS_RXCFG: u32 = E1000_ICR_RXCFG;
pub const E1000_IMS_GPI_EN0: u32 = E1000_ICR_GPI_EN0;
pub const E1000_IMS_GPI_EN1: u32 = E1000_ICR_GPI_EN1;
pub const E1000_IMS_GPI_EN2: u32 = E1000_ICR_GPI_EN2;
pub const E1000_IMS_GPI_EN3: u32 = E1000_ICR_GPI_EN3;
pub const E1000_IMS_TXD_LOW: u32 = E1000_ICR_TXD_LOW;
pub const E1000_IMS_SRPD: u32 = E1000_ICR_SRPD;
pub const E1000_IMS_ACK: u32 = E1000_ICR_ACK;
pub const E1000_IMS_MNG: u32 = E1000_ICR_MNG;
pub const E1000_IMS_DOCK: u32 = E1000_ICR_DOCK;
pub const E1000_IMS_RXD_FIFO_PAR0: u32 = E1000_ICR_RXD_FIFO_PAR0;
pub const E1000_IMS_TXD_FIFO_PAR0: u32 = E1000_ICR_TXD_FIFO_PAR0;
pub const E1000_IMS_HOST_ARB_PAR: u32 = E1000_ICR_HOST_ARB_PAR;
pub const E1000_IMS_PB_PAR: u32 = E1000_ICR_PB_PAR;
pub const E1000_IMS_RXD_FIFO_PAR1: u32 = E1000_ICR_RXD_FIFO_PAR1;
pub const E1000_IMS_TXD_FIFO_PAR1: u32 = E1000_ICR_TXD_FIFO_PAR1;
pub const E1000_IMS_DSW: u32 = E1000_ICR_DSW;
pub const E1000_IMS_PHYINT: u32 = E1000_ICR_PHYINT;
pub const E1000_IMS_EPRST: u32 = E1000_ICR_EPRST;

pub const E1000_IMC_TXDW: u32 = E1000_ICR_TXDW;
pub const E1000_IMC_TXQE: u32 = E1000_ICR_TXQE;
pub const E1000_IMC_LSC: u32 = E1000_ICR_LSC;
pub const E1000_IMC_RXSEQ: u32 = E1000_ICR_RXSEQ;
pub const E1000_IMC_RXDMT0: u32 = E1000_ICR_RXDMT0;
pub const E1000_IMC_RXO: u32 = E1000_ICR_RXO;
pub const E1000_IMC_RXT0: u32 = E1000_ICR_RXT0;
pub const E1000_IMC_MDAC: u32 = E1000_ICR_MDAC;
pub const E1000_IMC_RXCFG: u32 = E1000_ICR_RXCFG;
pub const E1000_IMC_GPI_EN0: u32 = E1000_ICR_GPI_EN0;
pub const E1000_IMC_GPI_EN1: u32 = E1000_ICR_GPI_EN1;
pub const E1000_IMC_GPI_EN2: u32 = E1000_ICR_GPI_EN2;
pub const E1000_IMC_GPI_EN3: u32 = E1000_ICR_GPI_EN3;
pub const E1000_IMC_TXD_LOW: u32 = E1000_ICR_TXD_LOW;
pub const E1000_IMC_SRPD: u32 = E1000_ICR_SRPD;
pub const E1000_IMC_ACK: u32 = E1000_ICR_ACK;
pub const E1000_IMC_MNG: u32 = E1000_ICR_MNG;
pub const E1000_IMC_DOCK: u32 = E1000_ICR_DOCK;
pub const E1000_IMC_RXD_FIFO_PAR0: u32 = E1000_ICR_RXD_FIFO_PAR0;
pub const E1000_IMC_TXD_FIFO_PAR0: u32 = E1000_ICR_TXD_FIFO_PAR0;
pub const E1000_IMC_HOST_ARB_PAR: u32 = E1000_ICR_HOST_ARB_PAR;
pub const E1000_IMC_PB_PAR: u32 = E1000_ICR_PB_PAR;
pub const E1000_IMC_RXD_FIFO_PAR1: u32 = E1000_ICR_RXD_FIFO_PAR1;
pub const E1000_IMC_TXD_FIFO_PAR1: u32 = E1000_ICR_TXD_FIFO_PAR1;
pub const E1000_IMC_DSW: u32 = E1000_ICR_DSW;
pub const E1000_IMC_PHYINT: u32 = E1000_ICR_PHYINT;
pub const E1000_IMC_EPRST: u32 = E1000_ICR_EPRST;

pub const E1000_RCTL_RST: u32 = 0x00000001;
pub const E1000_RCTL_EN: u32 = 0x00000002;
pub const E1000_RCTL_SBP: u32 = 0x00000004;
pub const E1000_RCTL_UPE: u32 = 0x00000008;
pub const E1000_RCTL_MPE: u32 = 0x00000010;
pub const E1000_RCTL_LPE: u32 = 0x00000020;
pub const E1000_RCTL_LBM_NO: u32 = 0x00000000;
pub const E1000_RCTL_LBM_MAC: u32 = 0x00000040;
pub const E1000_RCTL_LBM_SLP: u32 = 0x00000080;
pub const E1000_RCTL_LBM_TCVR: u32 = 0x000000C0;
pub const E1000_RCTL_DTYP_MASK: u32 = 0x00000C00;
pub const E1000_RCTL_DTYP_PS: u32 = 0x00000400;
pub const E1000_RCTL_RDMTS_HALF: u32 = 0x00000000;
pub const E1000_RCTL_RDMTS_QUAT: u32 = 0x00000100;
pub const E1000_RCTL_RDMTS_EIGTH: u32 = 0x00000200;
pub const E1000_RCTL_MO_SHIFT: u32 = 12;
pub const E1000_RCTL_MO_0: u32 = 0x00000000;
pub const E1000_RCTL_MO_1: u32 = 0x00001000;
pub const E1000_RCTL_MO_2: u32 = 0x00002000;
pub const E1000_RCTL_MO_3: u32 = 0x00003000;
pub const E1000_RCTL_MDR: u32 = 0x00004000;
pub const E1000_RCTL_BAM: u32 = 0x00008000;

pub const E1000_RCTL_SZ_2048: u32 = 0x00000000;
pub const E1000_RCTL_SZ_1024: u32 = 0x00010000;
pub const E1000_RCTL_SZ_512: u32 = 0x00020000;
pub const E1000_RCTL_SZ_256: u32 = 0x00030000;

pub const E1000_RCTL_SZ_16384: u32 = 0x00010000;
pub const E1000_RCTL_SZ_8192: u32 = 0x00020000;
pub const E1000_RCTL_SZ_4096: u32 = 0x00030000;
pub const E1000_RCTL_VFE: u32 = 0x00040000;
pub const E1000_RCTL_CFIEN: u32 = 0x00080000;
pub const E1000_RCTL_CFI: u32 = 0x00100000;
pub const E1000_RCTL_DPF: u32 = 0x00400000;
pub const E1000_RCTL_PMCF: u32 = 0x00800000;
pub const E1000_RCTL_BSEX: u32 = 0x02000000;
pub const E1000_RCTL_SECRC: u32 = 0x04000000;
pub const E1000_RCTL_FLXBUF_MASK: u32 = 0x78000000;
pub const E1000_RCTL_FLXBUF_SHIFT: u32 = 27;
pub const E1000_PSRCTL_BSIZE0_MASK: u32 = 0x0000007F;
pub const E1000_PSRCTL_BSIZE1_MASK: u32 = 0x00003F00;
pub const E1000_PSRCTL_BSIZE2_MASK: u32 = 0x003F0000;
pub const E1000_PSRCTL_BSIZE3_MASK: u32 = 0x3F000000;

pub const E1000_PSRCTL_BSIZE0_SHIFT: u32 = 7;
pub const E1000_PSRCTL_BSIZE1_SHIFT: u32 = 2;
pub const E1000_PSRCTL_BSIZE2_SHIFT: u32 = 6;
pub const E1000_PSRCTL_BSIZE3_SHIFT: u32 = 14;

pub const E1000_SWFW_EEP_SM: u32 = 0x0001;
pub const E1000_SWFW_PHY0_SM: u32 = 0x0002;
pub const E1000_SWFW_PHY1_SM: u32 = 0x0004;
pub const E1000_SWFW_MAC_CSR_SM: u32 = 0x0008;

pub const E1000_RDT_DELAY: u32 = 0x0000ffff;
pub const E1000_RDT_FPDB: u32 = 0x80000000;
pub const E1000_RDLEN_LEN: u32 = 0x0007ff80;
pub const E1000_RDH_RDH: u32 = 0x0000ffff;
pub const E1000_RDT_RDT: u32 = 0x0000ffff;

pub const E1000_FCRTH_RTH: u32 = 0x0000FFF8;
pub const E1000_FCRTH_XFCE: u32 = 0x80000000;
pub const E1000_FCRTL_RTL: u32 = 0x0000FFF8;
pub const E1000_FCRTL_XONE: u32 = 0x80000000;

pub const E1000_RFCTL_ISCSI_DIS: u32 = 0x00000001;
pub const E1000_RFCTL_ISCSI_DWC_MASK: u32 = 0x0000003E;
pub const E1000_RFCTL_ISCSI_DWC_SHIFT: u32 = 1;
pub const E1000_RFCTL_NFSW_DIS: u32 = 0x00000040;
pub const E1000_RFCTL_NFSR_DIS: u32 = 0x00000080;
pub const E1000_RFCTL_NFS_VER_MASK: u32 = 0x00000300;
pub const E1000_RFCTL_NFS_VER_SHIFT: u32 = 8;
pub const E1000_RFCTL_IPV6_DIS: u32 = 0x00000400;
pub const E1000_RFCTL_IPV6_XSUM_DIS: u32 = 0x00000800;
pub const E1000_RFCTL_ACK_DIS: u32 = 0x00001000;
pub const E1000_RFCTL_ACKD_DIS: u32 = 0x00002000;
pub const E1000_RFCTL_IPFRSP_DIS: u32 = 0x00004000;
pub const E1000_RFCTL_EXTEN: u32 = 0x00008000;
pub const E1000_RFCTL_IPV6_EX_DIS: u32 = 0x00010000;
pub const E1000_RFCTL_NEW_IPV6_EXT_DIS: u32 = 0x00020000;

pub const E1000_RXDCTL_PTHRESH: u32 = 0x0000003F;
pub const E1000_RXDCTL_HTHRESH: u32 = 0x00003F00;
pub const E1000_RXDCTL_WTHRESH: u32 = 0x003F0000;
pub const E1000_RXDCTL_GRAN: u32 = 0x01000000;

pub const E1000_TXDCTL_PTHRESH: u32 = 0x0000003F;
pub const E1000_TXDCTL_HTHRESH: u32 = 0x00003F00;
pub const E1000_TXDCTL_WTHRESH: u32 = 0x003F0000;
pub const E1000_TXDCTL_GRAN: u32 = 0x01000000;
pub const E1000_TXDCTL_LWTHRESH: u32 = 0xFE000000;
pub const E1000_TXDCTL_FULL_TX_DESC_WB: u32 = 0x01010000;
pub const E1000_TXDCTL_COUNT_DESC: u32 = 0x00400000;

pub const E1000_TXCW_FD: u32 = 0x00000020;
pub const E1000_TXCW_HD: u32 = 0x00000040;
pub const E1000_TXCW_PAUSE: u32 = 0x00000080;
pub const E1000_TXCW_ASM_DIR: u32 = 0x00000100;
pub const E1000_TXCW_PAUSE_MASK: u32 = 0x00000180;
pub const E1000_TXCW_RF: u32 = 0x00003000;
pub const E1000_TXCW_NP: u32 = 0x00008000;
pub const E1000_TXCW_CW: u32 = 0x0000ffff;
pub const E1000_TXCW_TXC: u32 = 0x40000000;
pub const E1000_TXCW_ANE: u32 = 0x80000000;

pub const E1000_RXCW_CW: u32 = 0x0000ffff;
pub const E1000_RXCW_NC: u32 = 0x04000000;
pub const E1000_RXCW_IV: u32 = 0x08000000;
pub const E1000_RXCW_CC: u32 = 0x10000000;
pub const E1000_RXCW_C: u32 = 0x20000000;
pub const E1000_RXCW_SYNCH: u32 = 0x40000000;
pub const E1000_RXCW_ANC: u32 = 0x80000000;

pub const E1000_TCTL_RST: u32 = 0x00000001;
pub const E1000_TCTL_EN: u32 = 0x00000002;
pub const E1000_TCTL_BCE: u32 = 0x00000004;
pub const E1000_TCTL_PSP: u32 = 0x00000008;
pub const E1000_TCTL_CT: u32 = 0x00000ff0;
pub const E1000_TCTL_COLD: u32 = 0x003ff000;
pub const E1000_TCTL_SWXOFF: u32 = 0x00400000;
pub const E1000_TCTL_PBE: u32 = 0x00800000;
pub const E1000_TCTL_RTLC: u32 = 0x01000000;
pub const E1000_TCTL_NRTU: u32 = 0x02000000;
pub const E1000_TCTL_MULR: u32 = 0x10000000;

pub const E1000_TCTL_EXT_BST_MASK: u32 = 0x000003FF;
pub const E1000_TCTL_EXT_GCEX_MASK: u32 = 0x000FFC00;

pub const E1000_RXCSUM_PCSS_MASK: u32 = 0x000000FF;
pub const E1000_RXCSUM_IPOFL: u32 = 0x00000100;
pub const E1000_RXCSUM_TUOFL: u32 = 0x00000200;
pub const E1000_RXCSUM_IPV6OFL: u32 = 0x00000400;
pub const E1000_RXCSUM_IPPCSE: u32 = 0x00001000;
pub const E1000_RXCSUM_PCSD: u32 = 0x00002000;

pub const E1000_MRQC_ENABLE_MASK: u32 = 0x00000003;
pub const E1000_MRQC_ENABLE_RSS_2Q: u32 = 0x00000001;
pub const E1000_MRQC_ENABLE_RSS_INT: u32 = 0x00000004;
pub const E1000_MRQC_RSS_FIELD_MASK: u32 = 0xFFFF0000;
pub const E1000_MRQC_RSS_FIELD_IPV4_TCP: u32 = 0x00010000;
pub const E1000_MRQC_RSS_FIELD_IPV4: u32 = 0x00020000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP_EX: u32 = 0x00040000;
pub const E1000_MRQC_RSS_FIELD_IPV6_EX: u32 = 0x00080000;
pub const E1000_MRQC_RSS_FIELD_IPV6: u32 = 0x00100000;
pub const E1000_MRQC_RSS_FIELD_IPV6_TCP: u32 = 0x00200000;

pub const E1000_WUC_APME: u32 = 0x00000001;
pub const E1000_WUC_PME_EN: u32 = 0x00000002;
pub const E1000_WUC_PME_STATUS: u32 = 0x00000004;
pub const E1000_WUC_APMPME: u32 = 0x00000008;
pub const E1000_WUC_SPM: u32 = 0x80000000;

pub const E1000_WUFC_LNKC: u32 = 0x00000001;
pub const E1000_WUFC_MAG: u32 = 0x00000002;
pub const E1000_WUFC_EX: u32 = 0x00000004;
pub const E1000_WUFC_MC: u32 = 0x00000008;
pub const E1000_WUFC_BC: u32 = 0x00000010;
pub const E1000_WUFC_ARP: u32 = 0x00000020;
pub const E1000_WUFC_IPV4: u32 = 0x00000040;
pub const E1000_WUFC_IPV6: u32 = 0x00000080;
pub const E1000_WUFC_IGNORE_TCO: u32 = 0x00008000;
pub const E1000_WUFC_FLX0: u32 = 0x00010000;
pub const E1000_WUFC_FLX1: u32 = 0x00020000;
pub const E1000_WUFC_FLX2: u32 = 0x00040000;
pub const E1000_WUFC_FLX3: u32 = 0x00080000;
pub const E1000_WUFC_ALL_FILTERS: u32 = 0x000F00FF;
pub const E1000_WUFC_FLX_OFFSET: u32 = 16;
pub const E1000_WUFC_FLX_FILTERS: u32 = 0x000F0000;

pub const E1000_WUS_LNKC: u32 = 0x00000001;
pub const E1000_WUS_MAG: u32 = 0x00000002;
pub const E1000_WUS_EX: u32 = 0x00000004;
pub const E1000_WUS_MC: u32 = 0x00000008;
pub const E1000_WUS_BC: u32 = 0x00000010;
pub const E1000_WUS_ARP: u32 = 0x00000020;
pub const E1000_WUS_IPV4: u32 = 0x00000040;
pub const E1000_WUS_IPV6: u32 = 0x00000080;
pub const E1000_WUS_FLX0: u32 = 0x00010000;
pub const E1000_WUS_FLX1: u32 = 0x00020000;
pub const E1000_WUS_FLX2: u32 = 0x00040000;
pub const E1000_WUS_FLX3: u32 = 0x00080000;
pub const E1000_WUS_FLX_FILTERS: u32 = 0x000F0000;

pub const E1000_MANC_SMBUS_EN: u32 = 0x00000001;
pub const E1000_MANC_ASF_EN: u32 = 0x00000002;
pub const E1000_MANC_R_ON_FORCE: u32 = 0x00000004;
pub const E1000_MANC_RMCP_EN: u32 = 0x00000100;
pub const E1000_MANC_0298_EN: u32 = 0x00000200;
pub const E1000_MANC_IPV4_EN: u32 = 0x00000400;
pub const E1000_MANC_IPV6_EN: u32 = 0x00000800;
pub const E1000_MANC_SNAP_EN: u32 = 0x00001000;
pub const E1000_MANC_ARP_EN: u32 = 0x00002000;
pub const E1000_MANC_NEIGHBOR_EN: u32 = 0x00004000;

pub const E1000_MANC_ARP_RES_EN: u32 = 0x00008000;
pub const E1000_MANC_TCO_RESET: u32 = 0x00010000;
pub const E1000_MANC_RCV_TCO_EN: u32 = 0x00020000;
pub const E1000_MANC_REPORT_STATUS: u32 = 0x00040000;
pub const E1000_MANC_RCV_ALL: u32 = 0x00080000;
pub const E1000_MANC_BLK_PHY_RST_ON_IDE: u32 = 0x00040000;
pub const E1000_MANC_EN_MAC_ADDR_FILTER: u32 = 0x00100000;
pub const E1000_MANC_EN_MNG2HOST: u32 = 0x00200000;

pub const E1000_MANC_EN_IP_ADDR_FILTER: u32 = 0x00400000;

pub const E1000_MANC_EN_XSUM_FILTER: u32 = 0x00800000;
pub const E1000_MANC_BR_EN: u32 = 0x01000000;
pub const E1000_MANC_SMB_REQ: u32 = 0x01000000;
pub const E1000_MANC_SMB_GNT: u32 = 0x02000000;
pub const E1000_MANC_SMB_CLK_IN: u32 = 0x04000000;
pub const E1000_MANC_SMB_DATA_IN: u32 = 0x08000000;
pub const E1000_MANC_SMB_DATA_OUT: u32 = 0x10000000;
pub const E1000_MANC_SMB_CLK_OUT: u32 = 0x20000000;

pub const E1000_MANC_SMB_DATA_OUT_SHIFT: u32 = 28;
pub const E1000_MANC_SMB_CLK_OUT_SHIFT: u32 = 29;

pub const E1000_SWSM_SMBI: u32 = 0x00000001;
pub const E1000_SWSM_SWESMBI: u32 = 0x00000002;
pub const E1000_SWSM_WMNG: u32 = 0x00000004;
pub const E1000_SWSM_DRV_LOAD: u32 = 0x00000008;

pub const E1000_FWSM_MODE_MASK: u32 = 0x0000000E;
pub const E1000_FWSM_MODE_SHIFT: u32 = 1;
pub const E1000_FWSM_FW_VALID: u32 = 0x00008000;

pub const E1000_FWSM_RSPCIPHY: u32 = 0x00000040;
pub const E1000_FWSM_DISSW: u32 = 0x10000000;
pub const E1000_FWSM_SKUSEL_MASK: u32 = 0x60000000;
pub const E1000_FWSM_SKUEL_SHIFT: u32 = 29;
pub const E1000_FWSM_SKUSEL_EMB: u32 = 0x0;
pub const E1000_FWSM_SKUSEL_CONS: u32 = 0x1;
pub const E1000_FWSM_SKUSEL_PERF_100: u32 = 0x2;
pub const E1000_FWSM_SKUSEL_PERF_GBE: u32 = 0x3;

pub const E1000_FFLT_DBG_INVC: u32 = 0x00100000;

pub const E1000_HICR_EN: u32 = 0x00000001;
pub const E1000_HICR_C: u32 = 0x00000002;
pub const E1000_HICR_SV: u32 = 0x00000004;
pub const E1000_HICR_FWR: u32 = 0x00000080;

pub const E1000_HI_MAX_DATA_LENGTH: u32 = 252;
pub const E1000_HI_MAX_BLOCK_BYTE_LENGTH: u32 = 1792;
pub const E1000_HI_MAX_BLOCK_DWORD_LENGTH: u32 = 448;
pub const E1000_HI_COMMAND_TIMEOUT: u32 = 500;

pub const E1000_HSMC0R_CLKIN: u32 = 0x00000001;
pub const E1000_HSMC0R_DATAIN: u32 = 0x00000002;
pub const E1000_HSMC0R_DATAOUT: u32 = 0x00000004;
pub const E1000_HSMC0R_CLKOUT: u32 = 0x00000008;

pub const E1000_HSMC1R_CLKIN: u32 = E1000_HSMC0R_CLKIN;
pub const E1000_HSMC1R_DATAIN: u32 = E1000_HSMC0R_DATAIN;
pub const E1000_HSMC1R_DATAOUT: u32 = E1000_HSMC0R_DATAOUT;
pub const E1000_HSMC1R_CLKOUT: u32 = E1000_HSMC0R_CLKOUT;

pub const E1000_FWSTS_FWS_MASK: u32 = 0x000000FF;

pub const E1000_WUPL_LENGTH_MASK: u32 = 0x0FFF;

pub const E1000_MDALIGN: u32 = 4096;

pub const E1000_GCR_RXD_NO_SNOOP: u32 = 0x00000001;
pub const E1000_GCR_RXDSCW_NO_SNOOP: u32 = 0x00000002;
pub const E1000_GCR_RXDSCR_NO_SNOOP: u32 = 0x00000004;
pub const E1000_GCR_TXD_NO_SNOOP: u32 = 0x00000008;
pub const E1000_GCR_TXDSCW_NO_SNOOP: u32 = 0x00000010;
pub const E1000_GCR_TXDSCR_NO_SNOOP: u32 = 0x00000020;
pub const PCI_EX_NO_SNOOP_ALL: u32 = 63;

pub const PCI_EX_82566_SNOOP_ALL: u32 = PCI_EX_NO_SNOOP_ALL;

pub const E1000_GCR_L1_ACT_WITHOUT_L0S_RX: u32 = 0x08000000;

pub const E1000_FACTPS_FUNC0_POWER_STATE_MASK: u32 = 0x00000003;
pub const E1000_FACTPS_LAN0_VALID: u32 = 0x00000004;
pub const E1000_FACTPS_FUNC0_AUX_EN: u32 = 0x00000008;
pub const E1000_FACTPS_FUNC1_POWER_STATE_MASK: u32 = 0x000000C0;
pub const E1000_FACTPS_FUNC1_POWER_STATE_SHIFT: u32 = 6;
pub const E1000_FACTPS_LAN1_VALID: u32 = 0x00000100;
pub const E1000_FACTPS_FUNC1_AUX_EN: u32 = 0x00000200;
pub const E1000_FACTPS_FUNC2_POWER_STATE_MASK: u32 = 0x00003000;
pub const E1000_FACTPS_FUNC2_POWER_STATE_SHIFT: u32 = 12;
pub const E1000_FACTPS_IDE_ENABLE: u32 = 0x00004000;
pub const E1000_FACTPS_FUNC2_AUX_EN: u32 = 0x00008000;
pub const E1000_FACTPS_FUNC3_POWER_STATE_MASK: u32 = 0x000C0000;
pub const E1000_FACTPS_FUNC3_POWER_STATE_SHIFT: u32 = 18;
pub const E1000_FACTPS_SP_ENABLE: u32 = 0x00100000;
pub const E1000_FACTPS_FUNC3_AUX_EN: u32 = 0x00200000;
pub const E1000_FACTPS_FUNC4_POWER_STATE_MASK: u32 = 0x03000000;
pub const E1000_FACTPS_FUNC4_POWER_STATE_SHIFT: u32 = 24;
pub const E1000_FACTPS_IPMI_ENABLE: u32 = 0x04000000;
pub const E1000_FACTPS_FUNC4_AUX_EN: u32 = 0x08000000;
pub const E1000_FACTPS_MNGCG: u32 = 0x20000000;
pub const E1000_FACTPS_LAN_FUNC_SEL: u32 = 0x40000000;
pub const E1000_FACTPS_PM_STATE_CHANGED: u32 = 0x80000000;

pub const PCI_EX_LINK_STATUS: u32 = 0x12;
pub const PCI_EX_LINK_WIDTH_MASK: u32 = 0x3F0;
pub const PCI_EX_LINK_WIDTH_SHIFT: u32 = 4;

pub const EEPROM_READ_OPCODE_MICROWIRE: u32 = 0x6;
pub const EEPROM_WRITE_OPCODE_MICROWIRE: u32 = 0x5;
pub const EEPROM_ERASE_OPCODE_MICROWIRE: u32 = 0x7;
pub const EEPROM_EWEN_OPCODE_MICROWIRE: u32 = 0x13;
pub const EEPROM_EWDS_OPCODE_MICROWIRE: u32 = 0x10;

pub const EEPROM_MAX_RETRY_SPI: u32 = 5000;
pub const EEPROM_READ_OPCODE_SPI: u32 = 0x03;
pub const EEPROM_WRITE_OPCODE_SPI: u32 = 0x02;
pub const EEPROM_A8_OPCODE_SPI: u32 = 0x08;
pub const EEPROM_WREN_OPCODE_SPI: u32 = 0x06;
pub const EEPROM_WRDI_OPCODE_SPI: u32 = 0x04;
pub const EEPROM_RDSR_OPCODE_SPI: u32 = 0x05;
pub const EEPROM_WRSR_OPCODE_SPI: u32 = 0x01;
pub const EEPROM_ERASE4K_OPCODE_SPI: u32 = 0x20;
pub const EEPROM_ERASE64K_OPCODE_SPI: u32 = 0xD8;
pub const EEPROM_ERASE256_OPCODE_SPI: u32 = 0xDB;

pub const EEPROM_WORD_SIZE_SHIFT: u32 = 6;
pub const EEPROM_SIZE_SHIFT: u32 = 10;
pub const EEPROM_SIZE_MASK: u32 = 0x1C00;

pub const EEPROM_COMPAT: u32 = 0x0003;
pub const EEPROM_ID_LED_SETTINGS: u32 = 0x0004;
pub const EEPROM_VERSION: u32 = 0x0005;
pub const EEPROM_SERDES_AMPLITUDE: u32 = 0x0006;
pub const EEPROM_PHY_CLASS_WORD: u32 = 0x0007;
pub const EEPROM_INIT_CONTROL1_REG: u32 = 0x000A;
pub const EEPROM_INIT_CONTROL2_REG: u32 = 0x000F;
pub const EEPROM_SWDEF_PINS_CTRL_PORT_1: u32 = 0x0010;
pub const EEPROM_INIT_CONTROL3_PORT_B: u32 = 0x0014;
pub const EEPROM_INIT_3GIO_3: u32 = 0x001A;
pub const EEPROM_SWDEF_PINS_CTRL_PORT_0: u32 = 0x0020;
pub const EEPROM_INIT_CONTROL3_PORT_A: u32 = 0x0024;
pub const EEPROM_CFG: u32 = 0x0012;
pub const EEPROM_FLASH_VERSION: u32 = 0x0032;
pub const EEPROM_CHECKSUM_REG: u32 = 0x003F;

pub const E1000_EEPROM_CFG_DONE: u32 = 0x00040000;
pub const E1000_EEPROM_CFG_DONE_PORT_1: u32 = 0x00080000;

pub const ID_LED_DEF1_DEF2: u32 = 0x1;
pub const ID_LED_DEF1_ON2: u32 = 0x2;
pub const ID_LED_DEF1_OFF2: u32 = 0x3;
pub const ID_LED_ON1_DEF2: u32 = 0x4;
pub const ID_LED_ON1_ON2: u32 = 0x5;
pub const ID_LED_ON1_OFF2: u32 = 0x6;
pub const ID_LED_OFF1_DEF2: u32 = 0x7;
pub const ID_LED_OFF1_ON2: u32 = 0x8;
pub const ID_LED_OFF1_OFF2: u32 = 0x9;

pub const IGP_ACTIVITY_LED_MASK: u32 = 0xFFFFF0FF;
pub const IGP_ACTIVITY_LED_ENABLE: u32 = 0x0300;
pub const IGP_LED3_MODE: u32 = 0x07000000;

pub const EEPROM_SERDES_AMPLITUDE_MASK: u32 = 0x000F;

pub const EEPROM_PHY_CLASS_A: u32 = 0x8000;

pub const EEPROM_WORD0A_ILOS: u32 = 0x0010;
pub const EEPROM_WORD0A_SWDPIO: u32 = 0x01E0;
pub const EEPROM_WORD0A_LRST: u32 = 0x0200;
pub const EEPROM_WORD0A_FD: u32 = 0x0400;
pub const EEPROM_WORD0A_66MHZ: u32 = 0x0800;

pub const EEPROM_WORD0F_PAUSE_MASK: u32 = 0x3000;
pub const EEPROM_WORD0F_PAUSE: u32 = 0x1000;
pub const EEPROM_WORD0F_ASM_DIR: u32 = 0x2000;
pub const EEPROM_WORD0F_ANE: u32 = 0x0800;
pub const EEPROM_WORD0F_SWPDIO_EXT: u32 = 0x00F0;
pub const EEPROM_WORD0F_LPLU: u32 = 0x0001;

pub const EEPROM_WORD1020_GIGA_DISABLE: u32 = 0x0010;
pub const EEPROM_WORD1020_GIGA_DISABLE_NON_D0A: u32 = 0x0008;

pub const EEPROM_WORD1A_ASPM_MASK: u32 = 0x000C;

pub const EEPROM_SUM: u32 = 0xBABA;

pub const EEPROM_NODE_ADDRESS_BYTE_0: u32 = 0;
pub const EEPROM_PBA_BYTE_1: u32 = 8;

pub const EEPROM_RESERVED_WORD: u32 = 0xFFFF;

pub const PBA_SIZE: u32 = 4;

pub const E1000_COLLISION_THRESHOLD: u32 = 15;
pub const E1000_CT_SHIFT: u32 = 4;

pub const E1000_COLLISION_DISTANCE: u32 = 63;
pub const E1000_COLLISION_DISTANCE_82542: u32 = 64;
pub const E1000_FDX_COLLISION_DISTANCE: u32 = E1000_COLLISION_DISTANCE;
pub const E1000_HDX_COLLISION_DISTANCE: u32 = E1000_COLLISION_DISTANCE;
pub const E1000_COLD_SHIFT: u32 = 12;

pub const REQ_TX_DESCRIPTOR_MULTIPLE: u32 = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: u32 = 8;

pub const DEFAULT_82542_TIPG_IPGT: u32 = 10;
pub const DEFAULT_82543_TIPG_IPGT_FIBER: u32 = 9;
pub const DEFAULT_82543_TIPG_IPGT_COPPER: u32 = 8;

pub const E1000_TIPG_IPGT_MASK: u32 = 0x000003FF;
pub const E1000_TIPG_IPGR1_MASK: u32 = 0x000FFC00;
pub const E1000_TIPG_IPGR2_MASK: u32 = 0x3FF00000;

pub const DEFAULT_82542_TIPG_IPGR1: u32 = 2;
pub const DEFAULT_82543_TIPG_IPGR1: u32 = 8;
pub const E1000_TIPG_IPGR1_SHIFT: u32 = 10;

pub const DEFAULT_82542_TIPG_IPGR2: u32 = 10;
pub const DEFAULT_82543_TIPG_IPGR2: u32 = 6;
pub const E1000_TIPG_IPGR2_SHIFT: u32 = 20;

pub const E1000_TXDMAC_DPP: u32 = 0x00000001;

pub const TX_THRESHOLD_START: u32 = 8;
pub const TX_THRESHOLD_INCREMENT: u32 = 10;
pub const TX_THRESHOLD_DECREMENT: u32 = 1;
pub const TX_THRESHOLD_STOP: u32 = 190;
pub const TX_THRESHOLD_DISABLE: u32 = 0;
pub const TX_THRESHOLD_TIMER_MS: u32 = 10000;
pub const MIN_NUM_XMITS: u32 = 1000;
pub const IFS_MAX: u32 = 80;
pub const IFS_STEP: u32 = 10;
pub const IFS_MIN: u32 = 40;
pub const IFS_RATIO: u32 = 4;

pub const E1000_EXTCNF_CTRL_PCIE_WRITE_ENABLE: u32 = 0x00000001;
pub const E1000_EXTCNF_CTRL_PHY_WRITE_ENABLE: u32 = 0x00000002;
pub const E1000_EXTCNF_CTRL_D_UD_ENABLE: u32 = 0x00000004;
pub const E1000_EXTCNF_CTRL_D_UD_LATENCY: u32 = 0x00000008;
pub const E1000_EXTCNF_CTRL_D_UD_OWNER: u32 = 0x00000010;
pub const E1000_EXTCNF_CTRL_MDIO_SW_OWNERSHIP: u32 = 0x00000020;
pub const E1000_EXTCNF_CTRL_MDIO_HW_OWNERSHIP: u32 = 0x00000040;
pub const E1000_EXTCNF_CTRL_EXT_CNF_POINTER: u32 = 0x0FFF0000;

pub const E1000_EXTCNF_SIZE_EXT_PHY_LENGTH: u32 = 0x000000FF;
pub const E1000_EXTCNF_SIZE_EXT_DOCK_LENGTH: u32 = 0x0000FF00;
pub const E1000_EXTCNF_SIZE_EXT_PCIE_LENGTH: u32 = 0x00FF0000;
pub const E1000_EXTCNF_CTRL_LCD_WRITE_ENABLE: u32 = 0x00000001;
pub const E1000_EXTCNF_CTRL_SWFLAG: u32 = 0x00000020;

pub const E1000_PBA_8K: u32 = 0x0008;
pub const E1000_PBA_12K: u32 = 0x000C;
pub const E1000_PBA_16K: u32 = 0x0010;
pub const E1000_PBA_20K: u32 = 0x0014;
pub const E1000_PBA_22K: u32 = 0x0016;
pub const E1000_PBA_24K: u32 = 0x0018;
pub const E1000_PBA_30K: u32 = 0x001E;
pub const E1000_PBA_32K: u32 = 0x0020;
pub const E1000_PBA_34K: u32 = 0x0022;
pub const E1000_PBA_38K: u32 = 0x0026;
pub const E1000_PBA_40K: u32 = 0x0028;
pub const E1000_PBA_48K: u32 = 0x0030;

pub const E1000_PBS_16K: u32 = E1000_PBA_16K;

pub const FLOW_CONTROL_ADDRESS_LOW: u32 = 0x00C28001;
pub const FLOW_CONTROL_ADDRESS_HIGH: u32 = 0x00000100;
pub const FLOW_CONTROL_TYPE: u32 = 0x8808;

pub const FC_DEFAULT_HI_THRESH: u32 = (0x8000);
pub const FC_DEFAULT_LO_THRESH: u32 = (0x4000);
pub const FC_DEFAULT_TX_TIMER: u32 = (0x100);

pub const PCIX_COMMAND_REGISTER: u32 = 0xE6;
pub const PCIX_STATUS_REGISTER_LO: u32 = 0xE8;
pub const PCIX_STATUS_REGISTER_HI: u32 = 0xEA;

pub const PCIX_COMMAND_MMRBC_MASK: u32 = 0x000C;
pub const PCIX_COMMAND_MMRBC_SHIFT: u32 = 0x2;
pub const PCIX_STATUS_HI_MMRBC_MASK: u32 = 0x0060;
pub const PCIX_STATUS_HI_MMRBC_SHIFT: u32 = 0x5;
pub const PCIX_STATUS_HI_MMRBC_4K: u32 = 0x3;
pub const PCIX_STATUS_HI_MMRBC_2K: u32 = 0x2;

pub const PAUSE_SHIFT: u32 = 5;

pub const SWDPIO_SHIFT: u32 = 17;

pub const SWDPIO__EXT_SHIFT: u32 = 4;

pub const ILOS_SHIFT: u32 = 3;

pub const RECEIVE_BUFFER_ALIGN_SIZE: u32 = (256);

pub const LINK_UP_TIMEOUT: u32 = 500;

pub const AUTO_READ_DONE_TIMEOUT: u32 = 10;

pub const PHY_CFG_TIMEOUT: u32 = 100;

pub const E1000_TX_BUFFER_SIZE: u32 = 1514;

pub const CARRIER_EXTENSION: u32 = 0x0F;

pub const E1000_CTRL_PHY_RESET_DIR: u32 = E1000_CTRL_SWDPIO0;
pub const E1000_CTRL_PHY_RESET: u32 = E1000_CTRL_SWDPIN0;
pub const E1000_CTRL_MDIO_DIR: u32 = E1000_CTRL_SWDPIO2;
pub const E1000_CTRL_MDIO: u32 = E1000_CTRL_SWDPIN2;
pub const E1000_CTRL_MDC_DIR: u32 = E1000_CTRL_SWDPIO3;
pub const E1000_CTRL_MDC: u32 = E1000_CTRL_SWDPIN3;
pub const E1000_CTRL_PHY_RESET_DIR4: u32 = E1000_CTRL_EXT_SDP4_DIR;
pub const E1000_CTRL_PHY_RESET4: u32 = E1000_CTRL_EXT_SDP4_DATA;

pub const PHY_CTRL: u32 = 0x00;
pub const PHY_STATUS: u32 = 0x01;
pub const PHY_ID1: u32 = 0x02;
pub const PHY_ID2: u32 = 0x03;
pub const PHY_AUTONEG_ADV: u32 = 0x04;
pub const PHY_LP_ABILITY: u32 = 0x05;
pub const PHY_AUTONEG_EXP: u32 = 0x06;
pub const PHY_NEXT_PAGE_TX: u32 = 0x07;
pub const PHY_LP_NEXT_PAGE: u32 = 0x08;
pub const PHY_1000T_CTRL: u32 = 0x09;
pub const PHY_1000T_STATUS: u32 = 0x0A;
pub const PHY_EXT_STATUS: u32 = 0x0F;

pub const MAX_PHY_REG_ADDRESS: u32 = 0x1F;
pub const MAX_PHY_MULTI_PAGE_REG: u32 = 0xF;

pub const M88E1000_PHY_SPEC_CTRL: u32 = 0x10;
pub const M88E1000_PHY_SPEC_STATUS: u32 = 0x11;
pub const M88E1000_INT_ENABLE: u32 = 0x12;
pub const M88E1000_INT_STATUS: u32 = 0x13;
pub const M88E1000_EXT_PHY_SPEC_CTRL: u32 = 0x14;
pub const M88E1000_RX_ERR_CNTR: u32 = 0x15;

pub const M88E1000_PHY_EXT_CTRL: u32 = 0x1A;
pub const M88E1000_PHY_PAGE_SELECT: u32 = 0x1D;
pub const M88E1000_PHY_GEN_CONTROL: u32 = 0x1E;
pub const M88E1000_PHY_VCO_REG_BIT8: u32 = 0x100;
pub const M88E1000_PHY_VCO_REG_BIT11: u32 = 0x800;

pub const IGP01E1000_IEEE_REGS_PAGE: u32 = 0x0000;
pub const IGP01E1000_IEEE_RESTART_AUTONEG: u32 = 0x3300;
pub const IGP01E1000_IEEE_FORCE_GIGA: u32 = 0x0140;

pub const IGP01E1000_PHY_PORT_CONFIG: u32 = 0x10;
pub const IGP01E1000_PHY_PORT_STATUS: u32 = 0x11;
pub const IGP01E1000_PHY_PORT_CTRL: u32 = 0x12;
pub const IGP01E1000_PHY_LINK_HEALTH: u32 = 0x13;
pub const IGP01E1000_GMII_FIFO: u32 = 0x14;
pub const IGP01E1000_PHY_CHANNEL_QUALITY: u32 = 0x15;
pub const IGP02E1000_PHY_POWER_MGMT: u32 = 0x19;
pub const IGP01E1000_PHY_PAGE_SELECT: u32 = 0x1F;

pub const IGP01E1000_PHY_AGC_A: u32 = 0x1172;
pub const IGP01E1000_PHY_AGC_B: u32 = 0x1272;
pub const IGP01E1000_PHY_AGC_C: u32 = 0x1472;
pub const IGP01E1000_PHY_AGC_D: u32 = 0x1872;

pub const IGP02E1000_PHY_AGC_A: u32 = 0x11B1;
pub const IGP02E1000_PHY_AGC_B: u32 = 0x12B1;
pub const IGP02E1000_PHY_AGC_C: u32 = 0x14B1;
pub const IGP02E1000_PHY_AGC_D: u32 = 0x18B1;

pub const IGP01E1000_PHY_DSP_RESET: u32 = 0x1F33;
pub const IGP01E1000_PHY_DSP_SET: u32 = 0x1F71;
pub const IGP01E1000_PHY_DSP_FFE: u32 = 0x1F35;

pub const IGP01E1000_PHY_CHANNEL_NUM: u32 = 4;
pub const IGP02E1000_PHY_CHANNEL_NUM: u32 = 4;

pub const IGP01E1000_PHY_AGC_PARAM_A: u32 = 0x1171;
pub const IGP01E1000_PHY_AGC_PARAM_B: u32 = 0x1271;
pub const IGP01E1000_PHY_AGC_PARAM_C: u32 = 0x1471;
pub const IGP01E1000_PHY_AGC_PARAM_D: u32 = 0x1871;

pub const IGP01E1000_PHY_EDAC_MU_INDEX: u32 = 0xC000;
pub const IGP01E1000_PHY_EDAC_SIGN_EXT_9_BITS: u32 = 0x8000;

pub const IGP01E1000_PHY_ANALOG_TX_STATE: u32 = 0x2890;
pub const IGP01E1000_PHY_ANALOG_CLASS_A: u32 = 0x2000;
pub const IGP01E1000_PHY_FORCE_ANALOG_ENABLE: u32 = 0x0004;
pub const IGP01E1000_PHY_DSP_FFE_CM_CP: u32 = 0x0069;

pub const IGP01E1000_PHY_DSP_FFE_DEFAULT: u32 = 0x002A;

pub const IGP01E1000_PHY_PCS_INIT_REG: u32 = 0x00B4;
pub const IGP01E1000_PHY_PCS_CTRL_REG: u32 = 0x00B5;

pub const IGP01E1000_ANALOG_REGS_PAGE: u32 = 0x20C0;

pub const MII_CR_SPEED_SELECT_MSB: u32 = 0x0040;
pub const MII_CR_COLL_TEST_ENABLE: u32 = 0x0080;
pub const MII_CR_FULL_DUPLEX: u32 = 0x0100;
pub const MII_CR_RESTART_AUTO_NEG: u32 = 0x0200;
pub const MII_CR_ISOLATE: u32 = 0x0400;
pub const MII_CR_POWER_DOWN: u32 = 0x0800;
pub const MII_CR_AUTO_NEG_EN: u32 = 0x1000;
pub const MII_CR_SPEED_SELECT_LSB: u32 = 0x2000;
pub const MII_CR_LOOPBACK: u32 = 0x4000;
pub const MII_CR_RESET: u32 = 0x8000;

pub const MII_SR_EXTENDED_CAPS: u32 = 0x0001;
pub const MII_SR_JABBER_DETECT: u32 = 0x0002;
pub const MII_SR_LINK_STATUS: u32 = 0x0004;
pub const MII_SR_AUTONEG_CAPS: u32 = 0x0008;
pub const MII_SR_REMOTE_FAULT: u32 = 0x0010;
pub const MII_SR_AUTONEG_COMPLETE: u32 = 0x0020;
pub const MII_SR_PREAMBLE_SUPPRESS: u32 = 0x0040;
pub const MII_SR_EXTENDED_STATUS: u32 = 0x0100;
pub const MII_SR_100T2_HD_CAPS: u32 = 0x0200;
pub const MII_SR_100T2_FD_CAPS: u32 = 0x0400;
pub const MII_SR_10T_HD_CAPS: u32 = 0x0800;
pub const MII_SR_10T_FD_CAPS: u32 = 0x1000;
pub const MII_SR_100X_HD_CAPS: u32 = 0x2000;
pub const MII_SR_100X_FD_CAPS: u32 = 0x4000;
pub const MII_SR_100T4_CAPS: u32 = 0x8000;

pub const NWAY_AR_SELECTOR_FIELD: u32 = 0x0001;
pub const NWAY_AR_10T_HD_CAPS: u32 = 0x0020;
pub const NWAY_AR_10T_FD_CAPS: u32 = 0x0040;
pub const NWAY_AR_100TX_HD_CAPS: u32 = 0x0080;
pub const NWAY_AR_100TX_FD_CAPS: u32 = 0x0100;
pub const NWAY_AR_100T4_CAPS: u32 = 0x0200;
pub const NWAY_AR_PAUSE: u32 = 0x0400;
pub const NWAY_AR_ASM_DIR: u32 = 0x0800;
pub const NWAY_AR_REMOTE_FAULT: u32 = 0x2000;
pub const NWAY_AR_NEXT_PAGE: u32 = 0x8000;

pub const NWAY_LPAR_SELECTOR_FIELD: u32 = 0x0000;
pub const NWAY_LPAR_10T_HD_CAPS: u32 = 0x0020;
pub const NWAY_LPAR_10T_FD_CAPS: u32 = 0x0040;
pub const NWAY_LPAR_100TX_HD_CAPS: u32 = 0x0080;
pub const NWAY_LPAR_100TX_FD_CAPS: u32 = 0x0100;
pub const NWAY_LPAR_100T4_CAPS: u32 = 0x0200;
pub const NWAY_LPAR_PAUSE: u32 = 0x0400;
pub const NWAY_LPAR_ASM_DIR: u32 = 0x0800;
pub const NWAY_LPAR_REMOTE_FAULT: u32 = 0x2000;
pub const NWAY_LPAR_ACKNOWLEDGE: u32 = 0x4000;
pub const NWAY_LPAR_NEXT_PAGE: u32 = 0x8000;

pub const NWAY_ER_LP_NWAY_CAPS: u32 = 0x0001;
pub const NWAY_ER_PAGE_RXD: u32 = 0x0002;
pub const NWAY_ER_NEXT_PAGE_CAPS: u32 = 0x0004;
pub const NWAY_ER_LP_NEXT_PAGE_CAPS: u32 = 0x0008;
pub const NWAY_ER_PAR_DETECT_FAULT: u32 = 0x0010;

pub const NPTX_MSG_CODE_FIELD: u32 = 0x0001;
pub const NPTX_TOGGLE: u32 = 0x0800;
pub const NPTX_ACKNOWLDGE2: u32 = 0x1000;
pub const NPTX_MSG_PAGE: u32 = 0x2000;
pub const NPTX_NEXT_PAGE: u32 = 0x8000;

pub const LP_RNPR_MSG_CODE_FIELD: u32 = 0x0001;
pub const LP_RNPR_TOGGLE: u32 = 0x0800;
pub const LP_RNPR_ACKNOWLDGE2: u32 = 0x1000;
pub const LP_RNPR_MSG_PAGE: u32 = 0x2000;
pub const LP_RNPR_ACKNOWLDGE: u32 = 0x4000;
pub const LP_RNPR_NEXT_PAGE: u32 = 0x8000;

pub const CR_1000T_ASYM_PAUSE: u32 = 0x0080;
pub const CR_1000T_HD_CAPS: u32 = 0x0100;
pub const CR_1000T_FD_CAPS: u32 = 0x0200;
pub const CR_1000T_REPEATER_DTE: u32 = 0x0400;

pub const CR_1000T_MS_VALUE: u32 = 0x0800;

pub const CR_1000T_MS_ENABLE: u32 = 0x1000;

pub const CR_1000T_TEST_MODE_NORMAL: u32 = 0x0000;
pub const CR_1000T_TEST_MODE_1: u32 = 0x2000;
pub const CR_1000T_TEST_MODE_2: u32 = 0x4000;
pub const CR_1000T_TEST_MODE_3: u32 = 0x6000;
pub const CR_1000T_TEST_MODE_4: u32 = 0x8000;

pub const SR_1000T_IDLE_ERROR_CNT: u32 = 0x00FF;
pub const SR_1000T_ASYM_PAUSE_DIR: u32 = 0x0100;
pub const SR_1000T_LP_HD_CAPS: u32 = 0x0400;
pub const SR_1000T_LP_FD_CAPS: u32 = 0x0800;
pub const SR_1000T_REMOTE_RX_STATUS: u32 = 0x1000;
pub const SR_1000T_LOCAL_RX_STATUS: u32 = 0x2000;
pub const SR_1000T_MS_CONFIG_RES: u32 = 0x4000;
pub const SR_1000T_MS_CONFIG_FAULT: u32 = 0x8000;
pub const SR_1000T_REMOTE_RX_STATUS_SHIFT: u32 = 12;
pub const SR_1000T_LOCAL_RX_STATUS_SHIFT: u32 = 13;
pub const SR_1000T_PHY_EXCESSIVE_IDLE_ERR_COUNT: u32 = 5;
pub const FFE_IDLE_ERR_COUNT_TIMEOUT_20: u32 = 20;
pub const FFE_IDLE_ERR_COUNT_TIMEOUT_100: u32 = 100;

pub const IEEE_ESR_1000T_HD_CAPS: u32 = 0x1000;
pub const IEEE_ESR_1000T_FD_CAPS: u32 = 0x2000;
pub const IEEE_ESR_1000X_HD_CAPS: u32 = 0x4000;
pub const IEEE_ESR_1000X_FD_CAPS: u32 = 0x8000;

pub const PHY_TX_POLARITY_MASK: u32 = 0x0100;
pub const PHY_TX_NORMAL_POLARITY: u32 = 0;

pub const AUTO_POLARITY_DISABLE: u32 = 0x0010;

pub const M88E1000_PSCR_JABBER_DISABLE: u32 = 0x0001;
pub const M88E1000_PSCR_POLARITY_REVERSAL: u32 = 0x0002;
pub const M88E1000_PSCR_SQE_TEST: u32 = 0x0004;
pub const M88E1000_PSCR_CLK125_DISABLE: u32 = 0x0010;
pub const M88E1000_PSCR_MDI_MANUAL_MODE: u32 = 0x0000;

pub const M88E1000_PSCR_MDIX_MANUAL_MODE: u32 = 0x0020;
pub const M88E1000_PSCR_AUTO_X_1000T: u32 = 0x0040;

pub const M88E1000_PSCR_AUTO_X_MODE: u32 = 0x0060;

pub const M88E1000_PSCR_10BT_EXT_DIST_ENABLE: u32 = 0x0080;

pub const M88E1000_PSCR_MII_5BIT_ENABLE: u32 = 0x0100;

pub const M88E1000_PSCR_SCRAMBLER_DISABLE: u32 = 0x0200;
pub const M88E1000_PSCR_FORCE_LINK_GOOD: u32 = 0x0400;
pub const M88E1000_PSCR_ASSERT_CRS_ON_TX: u32 = 0x0800;

pub const M88E1000_PSCR_POLARITY_REVERSAL_SHIFT: u32 = 1;
pub const M88E1000_PSCR_AUTO_X_MODE_SHIFT: u32 = 5;
pub const M88E1000_PSCR_10BT_EXT_DIST_ENABLE_SHIFT: u32 = 7;

pub const M88E1000_PSSR_JABBER: u32 = 0x0001;
pub const M88E1000_PSSR_REV_POLARITY: u32 = 0x0002;
pub const M88E1000_PSSR_DOWNSHIFT: u32 = 0x0020;
pub const M88E1000_PSSR_MDIX: u32 = 0x0040;
pub const M88E1000_PSSR_CABLE_LENGTH: u32 = 0x0380;

pub const M88E1000_PSSR_LINK: u32 = 0x0400;
pub const M88E1000_PSSR_SPD_DPLX_RESOLVED: u32 = 0x0800;
pub const M88E1000_PSSR_PAGE_RCVD: u32 = 0x1000;
pub const M88E1000_PSSR_DPLX: u32 = 0x2000;
pub const M88E1000_PSSR_SPEED: u32 = 0xC000;
pub const M88E1000_PSSR_10MBS: u32 = 0x0000;
pub const M88E1000_PSSR_100MBS: u32 = 0x4000;
pub const M88E1000_PSSR_1000MBS: u32 = 0x8000;

pub const M88E1000_PSSR_REV_POLARITY_SHIFT: u32 = 1;
pub const M88E1000_PSSR_DOWNSHIFT_SHIFT: u32 = 5;
pub const M88E1000_PSSR_MDIX_SHIFT: u32 = 6;
pub const M88E1000_PSSR_CABLE_LENGTH_SHIFT: u32 = 7;

pub const M88E1000_EPSCR_FIBER_LOOPBACK: u32 = 0x4000;
pub const M88E1000_EPSCR_DOWN_NO_IDLE: u32 = 0x8000;

pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_MASK: u32 = 0x0C00;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_1X: u32 = 0x0000;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_2X: u32 = 0x0400;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_3X: u32 = 0x0800;
pub const M88E1000_EPSCR_MASTER_DOWNSHIFT_4X: u32 = 0x0C00;

pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_MASK: u32 = 0x0300;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_DIS: u32 = 0x0000;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_1X: u32 = 0x0100;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_2X: u32 = 0x0200;
pub const M88E1000_EPSCR_SLAVE_DOWNSHIFT_3X: u32 = 0x0300;
pub const M88E1000_EPSCR_TX_CLK_2_5: u32 = 0x0060;
pub const M88E1000_EPSCR_TX_CLK_25: u32 = 0x0070;
pub const M88E1000_EPSCR_TX_CLK_0: u32 = 0x0000;

pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_MASK: u32 = 0x0E00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_1X: u32 = 0x0000;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_2X: u32 = 0x0200;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_3X: u32 = 0x0400;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_4X: u32 = 0x0600;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_5X: u32 = 0x0800;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_6X: u32 = 0x0A00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_7X: u32 = 0x0C00;
pub const M88EC018_EPSCR_DOWNSHIFT_COUNTER_8X: u32 = 0x0E00;

pub const IGP01E1000_PSCFR_AUTO_MDIX_PAR_DETECT: u32 = 0x0010;
pub const IGP01E1000_PSCFR_PRE_EN: u32 = 0x0020;
pub const IGP01E1000_PSCFR_SMART_SPEED: u32 = 0x0080;
pub const IGP01E1000_PSCFR_DISABLE_TPLOOPBACK: u32 = 0x0100;
pub const IGP01E1000_PSCFR_DISABLE_JABBER: u32 = 0x0400;
pub const IGP01E1000_PSCFR_DISABLE_TRANSMIT: u32 = 0x2000;

pub const IGP01E1000_PSSR_AUTONEG_FAILED: u32 = 0x0001;
pub const IGP01E1000_PSSR_POLARITY_REVERSED: u32 = 0x0002;
pub const IGP01E1000_PSSR_CABLE_LENGTH: u32 = 0x007C;
pub const IGP01E1000_PSSR_FULL_DUPLEX: u32 = 0x0200;
pub const IGP01E1000_PSSR_LINK_UP: u32 = 0x0400;
pub const IGP01E1000_PSSR_MDIX: u32 = 0x0800;
pub const IGP01E1000_PSSR_SPEED_MASK: u32 = 0xC000;
pub const IGP01E1000_PSSR_SPEED_10MBPS: u32 = 0x4000;
pub const IGP01E1000_PSSR_SPEED_100MBPS: u32 = 0x8000;
pub const IGP01E1000_PSSR_SPEED_1000MBPS: u32 = 0xC000;
pub const IGP01E1000_PSSR_CABLE_LENGTH_SHIFT: u32 = 0x0002;
pub const IGP01E1000_PSSR_MDIX_SHIFT: u32 = 0x000B;

pub const IGP01E1000_PSCR_TP_LOOPBACK: u32 = 0x0010;
pub const IGP01E1000_PSCR_CORRECT_NC_SCMBLR: u32 = 0x0200;
pub const IGP01E1000_PSCR_TEN_CRS_SELECT: u32 = 0x0400;
pub const IGP01E1000_PSCR_FLIP_CHIP: u32 = 0x0800;
pub const IGP01E1000_PSCR_AUTO_MDIX: u32 = 0x1000;
pub const IGP01E1000_PSCR_FORCE_MDI_MDIX: u32 = 0x2000;

pub const IGP01E1000_PLHR_SS_DOWNGRADE: u32 = 0x8000;
pub const IGP01E1000_PLHR_GIG_SCRAMBLER_ERROR: u32 = 0x4000;
pub const IGP01E1000_PLHR_MASTER_FAULT: u32 = 0x2000;
pub const IGP01E1000_PLHR_MASTER_RESOLUTION: u32 = 0x1000;
pub const IGP01E1000_PLHR_GIG_REM_RCVR_NOK: u32 = 0x0800;
pub const IGP01E1000_PLHR_IDLE_ERROR_CNT_OFLOW: u32 = 0x0400;
pub const IGP01E1000_PLHR_DATA_ERR_1: u32 = 0x0200;
pub const IGP01E1000_PLHR_DATA_ERR_0: u32 = 0x0100;
pub const IGP01E1000_PLHR_AUTONEG_FAULT: u32 = 0x0040;
pub const IGP01E1000_PLHR_AUTONEG_ACTIVE: u32 = 0x0010;
pub const IGP01E1000_PLHR_VALID_CHANNEL_D: u32 = 0x0008;
pub const IGP01E1000_PLHR_VALID_CHANNEL_C: u32 = 0x0004;
pub const IGP01E1000_PLHR_VALID_CHANNEL_B: u32 = 0x0002;
pub const IGP01E1000_PLHR_VALID_CHANNEL_A: u32 = 0x0001;

pub const IGP01E1000_MSE_CHANNEL_D: u32 = 0x000F;
pub const IGP01E1000_MSE_CHANNEL_C: u32 = 0x00F0;
pub const IGP01E1000_MSE_CHANNEL_B: u32 = 0x0F00;
pub const IGP01E1000_MSE_CHANNEL_A: u32 = 0xF000;

pub const IGP02E1000_PM_SPD: u32 = 0x0001;
pub const IGP02E1000_PM_D3_LPLU: u32 = 0x0004;
pub const IGP02E1000_PM_D0_LPLU: u32 = 0x0002;

pub const DSP_RESET_ENABLE: u32 = 0x0;
pub const DSP_RESET_DISABLE: u32 = 0x2;
pub const E1000_MAX_DSP_RESETS: u32 = 10;

pub const IGP01E1000_AGC_LENGTH_SHIFT: u32 = 7;
pub const IGP02E1000_AGC_LENGTH_SHIFT: u32 = 9;

pub const IGP02E1000_AGC_LENGTH_MASK: u32 = 0x7F;

pub const IGP01E1000_AGC_LENGTH_TABLE_SIZE: u32 = 128;
pub const IGP02E1000_AGC_LENGTH_TABLE_SIZE: u32 = 113;

pub const IGP01E1000_AGC_RANGE: u32 = 10;
pub const IGP02E1000_AGC_RANGE: u32 = 15;

pub const IGP01E1000_PHY_POLARITY_MASK: u32 = 0x0078;

pub const IGP01E1000_GMII_FLEX_SPD: u32 = 0x10;
pub const IGP01E1000_GMII_SPD: u32 = 0x20;

pub const IGP01E1000_ANALOG_SPARE_FUSE_STATUS: u32 = 0x20D1;
pub const IGP01E1000_ANALOG_FUSE_STATUS: u32 = 0x20D0;
pub const IGP01E1000_ANALOG_FUSE_CONTROL: u32 = 0x20DC;
pub const IGP01E1000_ANALOG_FUSE_BYPASS: u32 = 0x20DE;

pub const IGP01E1000_ANALOG_FUSE_POLY_MASK: u32 = 0xF000;
pub const IGP01E1000_ANALOG_FUSE_FINE_MASK: u32 = 0x0F80;
pub const IGP01E1000_ANALOG_FUSE_COARSE_MASK: u32 = 0x0070;
pub const IGP01E1000_ANALOG_SPARE_FUSE_ENABLED: u32 = 0x0100;
pub const IGP01E1000_ANALOG_FUSE_ENABLE_SW_CONTROL: u32 = 0x0002;

pub const IGP01E1000_ANALOG_FUSE_COARSE_THRESH: u32 = 0x0040;
pub const IGP01E1000_ANALOG_FUSE_COARSE_10: u32 = 0x0010;
pub const IGP01E1000_ANALOG_FUSE_FINE_1: u32 = 0x0080;
pub const IGP01E1000_ANALOG_FUSE_FINE_10: u32 = 0x0500;

pub const M88_VENDOR: u32 = 0x0141;
pub const M88E1000_E_PHY_ID: u32 = 0x01410C50;
pub const M88E1000_I_PHY_ID: u32 = 0x01410C30;
pub const M88E1011_I_PHY_ID: u32 = 0x01410C20;
pub const IGP01E1000_I_PHY_ID: u32 = 0x02A80380;
pub const M88E1000_12_PHY_ID: u32 = M88E1000_E_PHY_ID;
pub const M88E1000_14_PHY_ID: u32 = M88E1000_E_PHY_ID;
pub const M88E1011_I_REV_4: u32 = 0x04;
pub const M88E1111_I_PHY_ID: u32 = 0x01410CC0;
pub const M88E1118_E_PHY_ID: u32 = 0x01410E40;
pub const L1LXT971A_PHY_ID: u32 = 0x001378E0;

pub const RTL8211B_PHY_ID: u32 = 0x001CC910;
pub const RTL8201N_PHY_ID: u32 = 0x8200;
pub const RTL_PHY_CTRL_FD: u32 = 0x0100;
pub const RTL_PHY_CTRL_SPD_100: u32 = 0x200000;

pub const IGP3_CAP_INITIATE_TEAM: u32 = 0x0001;
pub const IGP3_CAP_WFM: u32 = 0x0002;
pub const IGP3_CAP_ASF: u32 = 0x0004;
pub const IGP3_CAP_LPLU: u32 = 0x0008;
pub const IGP3_CAP_DC_AUTO_SPEED: u32 = 0x0010;
pub const IGP3_CAP_SPD: u32 = 0x0020;
pub const IGP3_CAP_MULT_QUEUE: u32 = 0x0040;
pub const IGP3_CAP_RSS: u32 = 0x0080;
pub const IGP3_CAP_8021PQ: u32 = 0x0100;
pub const IGP3_CAP_AMT_CB: u32 = 0x0200;

pub const IGP3_PPC_JORDAN_EN: u32 = 0x0001;
pub const IGP3_PPC_JORDAN_GIGA_SPEED: u32 = 0x0002;

pub const IGP3_KMRN_PMC_EE_IDLE_LINK_DIS: u32 = 0x0001;
pub const IGP3_KMRN_PMC_K0S_ENTRY_LATENCY_MASK: u32 = 0x001E;
pub const IGP3_KMRN_PMC_K0S_MODE1_EN_GIGA: u32 = 0x0020;
pub const IGP3_KMRN_PMC_K0S_MODE1_EN_100: u32 = 0x0040;

pub const IGP3E1000_PHY_MISC_CTRL: u32 = 0x1B;
pub const IGP3_PHY_MISC_DUPLEX_MANUAL_SET: u32 = 0x1000;

// pub const IGP3_KMRN_EXT_CTRL : u32 = PHY_REG(770, 18);
pub const IGP3_KMRN_EC_DIS_INBAND: u32 = 0x0080;

pub const IGP03E1000_E_PHY_ID: u32 = 0x02A80390;
pub const IFE_E_PHY_ID: u32 = 0x02A80330;
pub const IFE_PLUS_E_PHY_ID: u32 = 0x02A80320;
pub const IFE_C_E_PHY_ID: u32 = 0x02A80310;

pub const IFE_PHY_EXTENDED_STATUS_CONTROL: u32 = 0x10;
pub const IFE_PHY_SPECIAL_CONTROL: u32 = 0x11;
pub const IFE_PHY_RCV_FALSE_CARRIER: u32 = 0x13;
pub const IFE_PHY_RCV_DISCONNECT: u32 = 0x14;
pub const IFE_PHY_RCV_ERROT_FRAME: u32 = 0x15;
pub const IFE_PHY_RCV_SYMBOL_ERR: u32 = 0x16;
pub const IFE_PHY_PREM_EOF_ERR: u32 = 0x17;
pub const IFE_PHY_RCV_EOF_ERR: u32 = 0x18;
pub const IFE_PHY_TX_JABBER_DETECT: u32 = 0x19;
pub const IFE_PHY_EQUALIZER: u32 = 0x1A;
pub const IFE_PHY_SPECIAL_CONTROL_LED: u32 = 0x1B;
pub const IFE_PHY_MDIX_CONTROL: u32 = 0x1C;
pub const IFE_PHY_HWI_CONTROL: u32 = 0x1D;

pub const IFE_PESC_REDUCED_POWER_DOWN_DISABLE: u32 = 0x2000;
pub const IFE_PESC_100BTX_POWER_DOWN: u32 = 0x0400;
pub const IFE_PESC_10BTX_POWER_DOWN: u32 = 0x0200;
pub const IFE_PESC_POLARITY_REVERSED: u32 = 0x0100;
pub const IFE_PESC_PHY_ADDR_MASK: u32 = 0x007C;
pub const IFE_PESC_SPEED: u32 = 0x0002;
pub const IFE_PESC_DUPLEX: u32 = 0x0001;
pub const IFE_PESC_POLARITY_REVERSED_SHIFT: u32 = 8;

pub const IFE_PSC_DISABLE_DYNAMIC_POWER_DOWN: u32 = 0x0100;
pub const IFE_PSC_FORCE_POLARITY: u32 = 0x0020;
pub const IFE_PSC_AUTO_POLARITY_DISABLE: u32 = 0x0010;
pub const IFE_PSC_JABBER_FUNC_DISABLE: u32 = 0x0001;
pub const IFE_PSC_FORCE_POLARITY_SHIFT: u32 = 5;
pub const IFE_PSC_AUTO_POLARITY_DISABLE_SHIFT: u32 = 4;

pub const IFE_PMC_AUTO_MDIX: u32 = 0x0080;
pub const IFE_PMC_FORCE_MDIX: u32 = 0x0040;
pub const IFE_PMC_MDIX_STATUS: u32 = 0x0020;
pub const IFE_PMC_AUTO_MDIX_COMPLETE: u32 = 0x0010;
pub const IFE_PMC_MDIX_MODE_SHIFT: u32 = 6;
pub const IFE_PHC_MDIX_RESET_ALL_MASK: u32 = 0x0000;

pub const IFE_PHC_HWI_ENABLE: u32 = 0x8000;
pub const IFE_PHC_ABILITY_CHECK: u32 = 0x4000;
pub const IFE_PHC_TEST_EXEC: u32 = 0x2000;
pub const IFE_PHC_HIGHZ: u32 = 0x0200;
pub const IFE_PHC_LOWZ: u32 = 0x0400;
pub const IFE_PHC_LOW_HIGH_Z_MASK: u32 = 0x0600;
pub const IFE_PHC_DISTANCE_MASK: u32 = 0x01FF;
pub const IFE_PHC_RESET_ALL_MASK: u32 = 0x0000;
pub const IFE_PSCL_PROBE_MODE: u32 = 0x0020;
pub const IFE_PSCL_PROBE_LEDS_OFF: u32 = 0x0006;
pub const IFE_PSCL_PROBE_LEDS_ON: u32 = 0x0007;

pub const ICH_FLASH_COMMAND_TIMEOUT: u32 = 5000;
pub const ICH_FLASH_ERASE_TIMEOUT: u32 = 3000000;
pub const ICH_FLASH_CYCLE_REPEAT_COUNT: u32 = 10;
pub const ICH_FLASH_SEG_SIZE_256: u32 = 256;
pub const ICH_FLASH_SEG_SIZE_4K: u32 = 4096;
pub const ICH_FLASH_SEG_SIZE_64K: u32 = 65536;

pub const ICH_CYCLE_READ: u32 = 0x0;
pub const ICH_CYCLE_RESERVED: u32 = 0x1;
pub const ICH_CYCLE_WRITE: u32 = 0x2;
pub const ICH_CYCLE_ERASE: u32 = 0x3;

pub const ICH_FLASH_GFPREG: u32 = 0x0000;
pub const ICH_FLASH_HSFSTS: u32 = 0x0004;
pub const ICH_FLASH_HSFCTL: u32 = 0x0006;
pub const ICH_FLASH_FADDR: u32 = 0x0008;
pub const ICH_FLASH_FDATA0: u32 = 0x0010;
pub const ICH_FLASH_FRACC: u32 = 0x0050;
pub const ICH_FLASH_FREG0: u32 = 0x0054;
pub const ICH_FLASH_FREG1: u32 = 0x0058;
pub const ICH_FLASH_FREG2: u32 = 0x005C;
pub const ICH_FLASH_FREG3: u32 = 0x0060;
pub const ICH_FLASH_FPR0: u32 = 0x0074;
pub const ICH_FLASH_FPR1: u32 = 0x0078;
pub const ICH_FLASH_SSFSTS: u32 = 0x0090;
pub const ICH_FLASH_SSFCTL: u32 = 0x0092;
pub const ICH_FLASH_PREOP: u32 = 0x0094;
pub const ICH_FLASH_OPTYPE: u32 = 0x0096;
pub const ICH_FLASH_OPMENU: u32 = 0x0098;

pub const ICH_FLASH_REG_MAPSIZE: u32 = 0x00A0;
pub const ICH_FLASH_SECTOR_SIZE: u32 = 4096;
pub const ICH_GFPREG_BASE_MASK: u32 = 0x1FFF;
pub const ICH_FLASH_LINEAR_ADDR_MASK: u32 = 0x00FFFFFF;

pub const PHY_PREAMBLE: u32 = 0xFFFFFFFF;
pub const PHY_SOF: u32 = 0x01;
pub const PHY_OP_READ: u32 = 0x02;
pub const PHY_OP_WRITE: u32 = 0x01;
pub const PHY_TURNAROUND: u32 = 0x02;
pub const PHY_PREAMBLE_SIZE: u32 = 32;
pub const MII_CR_SPEED_1000: u32 = 0x0040;
pub const MII_CR_SPEED_100: u32 = 0x2000;
pub const MII_CR_SPEED_10: u32 = 0x0000;
pub const E1000_PHY_ADDRESS: u32 = 0x01;
pub const PHY_AUTO_NEG_TIME: u32 = 45;
pub const PHY_FORCE_TIME: u32 = 20;
pub const PHY_REVISION_MASK: u32 = 0xFFFFFFF0;
pub const DEVICE_SPEED_MASK: u32 = 0x00000300;
pub const REG4_SPEED_MASK: u32 = 0x01E0;
pub const REG9_SPEED_MASK: u32 = 0x0300;
pub const ADVERTISE_10_HALF: u32 = 0x0001;
pub const ADVERTISE_10_FULL: u32 = 0x0002;
pub const ADVERTISE_100_HALF: u32 = 0x0004;
pub const ADVERTISE_100_FULL: u32 = 0x0008;
pub const ADVERTISE_1000_HALF: u32 = 0x0010;
pub const ADVERTISE_1000_FULL: u32 = 0x0020;
pub const AUTONEG_ADVERTISE_SPEED_DEFAULT: u32 = 0x002F;
pub const AUTONEG_ADVERTISE_10_100_ALL: u32 = 0x000F;
pub const AUTONEG_ADVERTISE_10_ALL: u32 = 0x0003;
