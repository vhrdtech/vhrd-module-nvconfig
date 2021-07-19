#![no_std]

pub struct NVConfig {
    pub bootloader: BoardConfig,
    pub module_specific: [u8; 256],
    pub vhl_bytecode: [u8; 1536],
}

pub struct BoardConfig {
    pub hw_name: [u8; 32],
    pub hw_variant: [u8; 4],
    pub hw_version: Version,

    pub sw_version: Version,
    pub sw_vcs_id: [u8; 8],
    pub sw_crc: u64,
    // Globally unique identifier for each hardware unit, use STM UID
    // pub uuid: [u8; 16],
}
const SIZE_OF_BOARD_CONFIG: usize = 256;
#[allow(unreachable_code)]
unsafe fn _board_config_size_is_valid() { core::mem::transmute::<BoardConfig, [u8; SIZE_OF_BOARD_CONFIG]>(panic!()); }