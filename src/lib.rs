#![no_std]

pub const NV_CONFIG_START_ADDR: usize = 0x0800_0000 + 10 * 1024;
pub const SIZE_OF_NVCONFIG: usize = 2048;
#[allow(dead_code)]
const SIZE_OF_BOARD_CONFIG: usize = 256;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct NVConfig {
    /// CRC of all the bytes except CRC itself
    pub config_crc: u64,
    /// Various information about the board and firmware it runs
    pub board_config: BoardConfig,
    /// Area for firmware configuration storage
    pub firmware_specific: [u8; 256],
    /// VHL bytecode describing API of the board
    pub vhl_bytecode: [u8; 1528],
}

impl NVConfig {
    pub fn get() -> &'static Self {
        unsafe {
            let addr = NV_CONFIG_START_ADDR as *const NVConfig;
            &(*addr)
        }
    }
}

/// Non volatile config for bootloader, firmware and VHL bytecode storage.
/// Treat strings as null-terminated if null can fit, otherwise use whole array.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct BoardConfig {
    /// Name of the board, for example: vhrd.strain_gauge
    pub hw_name: [u8; 32],
    /// Variant of the board, for example LMP or HX or empty (\[0\] = 0)
    pub hw_variant: [u8; 4],
    /// Version of the board
    pub hw_version: Version,

    /// CRC of the bootloader, if incorrect, only SWD can be used to reprogram everything
    pub bootloader_crc: u64,
    /// Wait this amount of time before booting firmware
    pub bootloader_timeout_ms: u16,

    /// Version of the firmware
    pub fw_version: Version,
    /// Variant of the firmware or empty (\[0\] = 0)
    pub fw_variant: [u8; 4],
    /// Git hash or similar
    pub fw_vcs_id: [u8; 8],
    /// Length of the firmware
    pub fw_size: u32,
    /// CRC of the firmware
    pub fw_crc: u64,
    // Globally unique identifier for each hardware unit, use STM UID
    // pub uuid: [u8; 16],

    /// Mode of the CAN buses used by bootloader and firmware.
    /// If unknown, must be learned from the bus by passively listening first.
    pub canbus_mode: CANBusMode,
    /// Speed of the CAN buses used by bootloader and firmware
    /// If unknown, must be learned from the bus by passively listening first.
    pub canbus_speed: CANBusSpeed,
    /// UAVCAN node id of this board, set to 255 if unknown
    /// If unknown, communication can only happen with broadcast messages.
    /// Can be set by user manually or assigned by provisioning tool.
    pub uavcan_node_id: u8,

    pub reserved: [u8; 164]
}

#[allow(unreachable_code)]
unsafe fn _nv_config_size_is_valid() { core::mem::transmute::<NVConfig, [u8; SIZE_OF_NVCONFIG]>(panic!()); }

#[allow(unreachable_code)]
unsafe fn _board_config_size_is_valid() { core::mem::transmute::<BoardConfig, [u8; SIZE_OF_BOARD_CONFIG]>(panic!()); }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum CANBusSpeed {
    Unknown,
    _125kBps,
    _250kBps,
    _500kBps,
    _1Mbps,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum CANBusMode {
    Unknown,
    Classical,
    FD,
}