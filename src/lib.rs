#![no_std]

pub const NV_CONFIG_START_ADDR: usize = 0x0800_0000 + 10 * 1024;
#[allow(dead_code)]
const SIZE_OF_NVCONFIG: usize = 2048;
#[allow(dead_code)]
const SIZE_OF_BOARD_CONFIG: usize = 256;

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

pub struct NVConfig {
    /// CRC of all the bytes except CRC itself
    pub config_crc: u64,
    /// Various information about the board and firmware it runs
    pub bootloader: BoardConfig,
    /// Area for firmware configuration storage
    pub module_specific: [u8; 256],
    /// VHL bytecode describing API of the board
    pub vhl_bytecode: [u8; 1528],
}

#[allow(unreachable_code)]
unsafe fn _nv_config_size_is_valid() { core::mem::transmute::<NVConfig, [u8; SIZE_OF_NVCONFIG]>(panic!()); }

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
pub struct BoardConfig {
    /// Name of the board, for example: vhrd.strain_gauge
    pub hw_name: [u8; 32],
    /// Variant of the board, for example LMP or HX or empty (\[0\] = 0)
    pub hw_variant: [u8; 4],
    /// Version of the board
    pub hw_version: Version,

    /// Version of the firmware
    pub sw_version: Version,
    /// Variant of the firmware or empty (\[0\] = 0)
    pub sw_variant: [u8; 4],
    /// Git hash or similar
    pub sw_vcs_id: [u8; 8],
    /// CRC of the firmware
    pub sw_crc: u64,
    // Globally unique identifier for each hardware unit, use STM UID
    // pub uuid: [u8; 16],

    pub reserved: [u8; 190]
}
#[allow(unreachable_code)]
unsafe fn _board_config_size_is_valid() { core::mem::transmute::<BoardConfig, [u8; SIZE_OF_BOARD_CONFIG]>(panic!()); }
