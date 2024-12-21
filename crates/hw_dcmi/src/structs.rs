//! Wrapped structs for the DCMI peripheral

use crate::error::{DCMIError, DCMIResult};
use hw_dcmi_sys::bindings as ffi;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use std::ffi::CStr;

/// Chip information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChipInfo {
    /// Chip type
    pub chip_type: String,
    /// Chip name
    pub chip_name: String,
    /// Chip version
    pub chip_version: String,
    /// Chip AI core count, for MCU and CPU, this field makes no sense
    pub ai_core_count: u32,
}

impl From<ffi::dcmi_chip_info> for ChipInfo {
    fn from(chip_info: ffi::dcmi_chip_info) -> Self {
        ChipInfo {
            chip_type: CStr::from_bytes_until_nul(&chip_info.chip_type)
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            chip_name: CStr::from_bytes_until_nul(&chip_info.chip_name)
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            chip_version: CStr::from_bytes_until_nul(&chip_info.chip_ver)
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            ai_core_count: chip_info.aicore_cnt as u32,
        }
    }
}

/// PCIE information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PCIEInfo {
    /// Device ID
    pub device_id: u32,
    /// Vender ID
    pub vender_id: u32,
    /// Subvender ID
    pub subvender_id: u32,
    /// Subdevice ID
    pub subdevice_id: u32,
    /// BDF device ID
    pub bdf_device_id: u32,
    /// BDF bus ID
    pub bdf_bus_id: u32,
    /// BDF function ID
    pub bdf_func_id: u32,
}

macro_rules! impl_from_pcie_info {
    ($src:ty, $dst:ty) => {
        impl From<$src> for $dst {
            fn from(pcie_info: $src) -> Self {
                PCIEInfo {
                    device_id: pcie_info.deviceid as u32,
                    vender_id: pcie_info.venderid as u32,
                    subvender_id: pcie_info.subvenderid as u32,
                    subdevice_id: pcie_info.subdeviceid as u32,
                    bdf_device_id: pcie_info.bdf_deviceid as u32,
                    bdf_bus_id: pcie_info.bdf_busid as u32,
                    bdf_func_id: pcie_info.bdf_funcid as u32,
                }
            }
        }
    };
}

impl_from_pcie_info!(ffi::dcmi_pcie_info, PCIEInfo);
impl_from_pcie_info!(ffi::dcmi_pcie_info_all, PCIEInfo);

/// PCIE information with domain
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DomainPCIEInfo {
    /// PCIE information
    pub pcie_info: PCIEInfo,
    /// Domain
    pub domain: i32,
}

impl From<ffi::dcmi_pcie_info_all> for DomainPCIEInfo {
    fn from(pcie_info: ffi::dcmi_pcie_info_all) -> Self {
        DomainPCIEInfo {
            pcie_info: pcie_info.into(),
            domain: pcie_info.domain as i32,
        }
    }
}

/// Board information
///
/// # Notes
/// when chip is NPU, only board_id and slot_id is valid, slot_id tagged the pcie slot where chip is located
///
/// when chip is MCU, all fields are valid, slot_id tagged the position of card where chip is located
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoardInfo {
    /// Board ID
    pub board_id: u32,
    /// PCB ID
    pub pcb_id: u32,
    /// BOM ID
    pub bom_id: u32,
    /// Slot ID
    pub slot_id: u32,
}

impl From<ffi::dcmi_board_info> for BoardInfo {
    fn from(board_info: ffi::dcmi_board_info) -> Self {
        BoardInfo {
            board_id: board_info.board_id as u32,
            pcb_id: board_info.pcb_id as u32,
            bom_id: board_info.bom_id as u32,
            slot_id: board_info.slot_id as u32,
        }
    }
}

/// ELabel information
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ELabelInfo {
    /// Product name
    pub product_name: String,
    /// Model
    pub model: String,
    /// Manufacturer
    pub manufacturer: String,
    /// Serial number
    pub serial_number: String,
}

impl From<ffi::dcmi_elabel_info> for ELabelInfo {
    fn from(elabel_info: ffi::dcmi_elabel_info) -> Self {
        ELabelInfo {
            product_name: CStr::from_bytes_until_nul(&elabel_info.product_name.map(|x| x as u8))
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            model: CStr::from_bytes_until_nul(&elabel_info.model.map(|x| x as u8))
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            manufacturer: CStr::from_bytes_until_nul(&elabel_info.manufacturer.map(|x| x as u8))
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            serial_number: CStr::from_bytes_until_nul(&elabel_info.serial_number.map(|x| x as u8))
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
        }
    }
}

/// Die ID
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DieInfo {
    /// SOC die
    pub soc_die: [u32; ffi::DIE_ID_COUNT as usize],
}

impl From<ffi::dcmi_die_id> for DieInfo {
    fn from(die_id: ffi::dcmi_die_id) -> Self {
        DieInfo {
            soc_die: die_id.soc_die,
        }
    }
}

/// Flash information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlashInfo {
    /// Flash ID
    pub flash_id: u64,
    /// Device ID
    pub device_id: u16,
    /// Vendor
    pub vendor: u16,
    /// Health status
    pub is_health: bool,
    /// Flash size
    pub size: u64,
    /// Sector count
    pub sector_count: u32,
    /// Manufacturer ID
    pub manufacturer_id: u16,
}

impl From<ffi::dcmi_flash_info> for FlashInfo {
    fn from(flash_info: ffi::dcmi_flash_info) -> Self {
        FlashInfo {
            flash_id: flash_info.flash_id,
            device_id: flash_info.device_id,
            vendor: flash_info.vendor,
            is_health: flash_info.state == 0x8,
            size: flash_info.size,
            sector_count: flash_info.sector_count,
            manufacturer_id: flash_info.manufacturer_id,
        }
    }
}

/// AI core information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AICoreInfo {
    /// Frequency, unit: MHz
    pub frequency: u32,
    /// Current frequency, unit: MHz
    pub current_frequency: u32,
}

impl From<ffi::dcmi_aicore_info> for AICoreInfo {
    fn from(ai_core_info: ffi::dcmi_aicore_info) -> Self {
        AICoreInfo {
            frequency: ai_core_info.freq as u32,
            current_frequency: ai_core_info.cur_freq as u32,
        }
    }
}

/// AI CPU information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AICPUInfo {
    /// Maximum frequency, unit: MHz
    pub max_frequency: u32,
    /// Current frequency, unit: MHz
    pub current_frequency: u32,
    /// AI CPU number
    pub aicpu_num: u32,
    /// Utilization rate
    pub util_rate: [u32; ffi::MAX_CORE_NUM as usize],
}

impl From<ffi::dcmi_aicpu_info> for AICPUInfo {
    fn from(aicpu_info: ffi::dcmi_aicpu_info) -> Self {
        AICPUInfo {
            max_frequency: aicpu_info.max_freq as u32,
            current_frequency: aicpu_info.cur_freq as u32,
            aicpu_num: aicpu_info.aicpu_num as u32,
            util_rate: aicpu_info.util_rate,
        }
    }
}

/// Memory information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryInfo {
    /// Memory size, unit: MB
    pub memory_size: u64,
    /// Memory available, unit: MB, free + huge_pages_free * huge_page_size
    pub memory_available: u64,
    /// Frequency
    pub freq: u32,
    /// Huge page size, unit: KB
    pub huge_page_size: u64,
    /// Huge pages total
    pub huge_pages_total: u64,
    /// Huge pages free
    pub huge_pages_free: u64,
    /// Utilization, DDR memory info usages
    pub utilization: u32,
}

impl From<ffi::dcmi_get_memory_info_stru> for MemoryInfo {
    fn from(memory_info: ffi::dcmi_get_memory_info_stru) -> Self {
        MemoryInfo {
            memory_size: memory_info.memory_size as u64,
            memory_available: memory_info.memory_available as u64,
            freq: memory_info.freq as u32,
            huge_page_size: memory_info.hugepagesize as u64,
            huge_pages_total: memory_info.hugepages_total as u64,
            huge_pages_free: memory_info.hugepages_free as u64,
            utilization: memory_info.utiliza as u32,
        }
    }
}

/// HBM information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HBMInfo {
    /// HBM total size, MB
    pub memory_size: u64,
    /// HBM frequency, MHz
    pub frequency: u32,
    /// HBM memory usage, MB
    pub memory_usage: u64,
    /// HBM temperature
    pub temperature: i32,
    /// HBM bandwidth utilization rate
    pub bandwidth_util_rate: u32,
}

impl From<ffi::dcmi_hbm_info> for HBMInfo {
    fn from(hbm_info: ffi::dcmi_hbm_info) -> Self {
        HBMInfo {
            memory_size: hbm_info.memory_size as u64,
            frequency: hbm_info.freq as u32,
            memory_usage: hbm_info.memory_usage as u64,
            temperature: hbm_info.temp as i32,
            bandwidth_util_rate: hbm_info.bandwith_util_rate as u32,
        }
    }
}

/// Chip PCIE error rate
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChipPCIEErrorRate {
    /// Deskew FIFO overflow interrupt status
    pub deskew_fifo_overflow_intr_status: bool,
    /// Symbol unlock interrupt status
    pub symbol_unlock_intr_status: bool,
    /// Deskew unlock interrupt status
    pub deskew_unlock_intr_status: bool,
    /// Phystatus timeout interrupt status
    pub phystatus_timeout_intr_status: bool,
    /// Symbol unlock counter
    pub symbol_unlock_counter: u32,
    /// PCS RX error count
    pub pcs_rx_err_cnt: u32,
    /// PHY lane error counter
    pub phy_lane_err_counter: u32,
    /// PCS receive error status, each bool maps to each used channel
    pub pcs_rcv_err_status: Vec<bool>,
    /// Symbol unlock error status, each bool maps to each used channel
    pub symbol_unlock_err_status: Vec<bool>,
    /// PHY lane error status, each bool maps to each used channel
    pub phy_lane_err_status: Vec<bool>,
    /// DL LCRC error number
    pub dl_lcrc_err_num: u32,
    /// DL DCRC error number
    pub dl_dcrc_err_num: u32,
}

impl From<ffi::dcmi_chip_pcie_err_rate> for ChipPCIEErrorRate {
    fn from(chip_pcie_err_rate: ffi::dcmi_chip_pcie_err_rate) -> Self {
        ChipPCIEErrorRate {
            deskew_fifo_overflow_intr_status: chip_pcie_err_rate
                .reg_deskew_fifo_overflow_intr_status
                != 0,
            symbol_unlock_intr_status: chip_pcie_err_rate.reg_symbol_unlock_intr_status != 0,
            deskew_unlock_intr_status: chip_pcie_err_rate.reg_deskew_unlock_intr_status != 0,
            phystatus_timeout_intr_status: chip_pcie_err_rate.reg_phystatus_timeout_intr_status
                != 0,
            symbol_unlock_counter: chip_pcie_err_rate.symbol_unlock_counter,
            pcs_rx_err_cnt: chip_pcie_err_rate.pcs_rx_err_cnt,
            phy_lane_err_counter: chip_pcie_err_rate.phy_lane_err_counter,
            pcs_rcv_err_status: (0..32usize)
                .map(|i| chip_pcie_err_rate.pcs_rcv_err_status & (1 << i) != 0)
                .collect(),
            symbol_unlock_err_status: (0..32usize)
                .map(|i| chip_pcie_err_rate.symbol_unlock_err_status & (1 << i) != 0)
                .collect(),
            phy_lane_err_status: (0..32usize)
                .map(|i| chip_pcie_err_rate.phy_lane_err_status & (1 << i) != 0)
                .collect(),
            dl_lcrc_err_num: chip_pcie_err_rate.dl_lcrc_err_num,
            dl_dcrc_err_num: chip_pcie_err_rate.dl_dcrc_err_num,
        }
    }
}

/// ECC information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ECCInfo {
    /// ECC enable flag
    pub enable_flag: bool,
    /// Single bit error count
    pub single_bit_error_cnt: u32,
    /// Double bit error count
    pub double_bit_error_cnt: u32,
    /// Total single bit error count
    pub total_single_bit_error_cnt: u32,
    /// Total double bit error count
    pub total_double_bit_error_cnt: u32,
    /// Single bit isolated pages count
    pub single_bit_isolated_pages_cnt: u32,
    /// Double bit isolated pages count
    pub double_bit_isolated_pages_cnt: u32,
}

impl From<ffi::dcmi_ecc_info> for ECCInfo {
    fn from(ecc_info: ffi::dcmi_ecc_info) -> Self {
        ECCInfo {
            enable_flag: ecc_info.enable_flag != 0,
            single_bit_error_cnt: ecc_info.single_bit_error_cnt,
            double_bit_error_cnt: ecc_info.double_bit_error_cnt,
            total_single_bit_error_cnt: ecc_info.total_single_bit_error_cnt,
            total_double_bit_error_cnt: ecc_info.total_double_bit_error_cnt,
            single_bit_isolated_pages_cnt: ecc_info.single_bit_isolated_pages_cnt,
            double_bit_isolated_pages_cnt: ecc_info.double_bit_isolated_pages_cnt,
        }
    }
}

/// VChip resource
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VChipRes {
    /// VChip ID
    pub vchip_id: u32,
    /// VChip Group ID
    pub vfg_id: u32,
    /// Template name
    pub template_name: String,
}

impl VChipRes {
    /// Create a new default VChipRes
    pub fn new(template_name: String) -> Self {
        VChipRes {
            vchip_id: 0xFFFFFFFF,
            vfg_id: 0xFFFFFFFF,
            template_name,
        }
    }

    /// Create a new VChipRes by ID
    pub fn new_by_id(vchip_id: u32, vfg_id: u32, template_name: String) -> Self {
        VChipRes {
            vchip_id,
            vfg_id,
            template_name,
        }
    }
}

impl From<VChipRes> for ffi::dcmi_create_vdev_res_stru {
    fn from(vchip_res: VChipRes) -> Self {
        let mut template_name = [0 as std::os::raw::c_char; 32];
        let reserved = [0 as std::os::raw::c_uchar; 64];
        let template_bytes = vchip_res.template_name.as_bytes();
        for (i, &byte) in template_bytes.iter().take(32).enumerate() {
            template_name[i] = byte as std::os::raw::c_char;
        }
        ffi::dcmi_create_vdev_res_stru {
            vdev_id: vchip_res.vchip_id,
            vfg_id: vchip_res.vfg_id,
            template_name,
            reserved,
        }
    }
}

/// Create VChip output
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VChipOutput {
    /// VChip ID
    pub vchip_id: u32,
    /// VChip group ID
    pub vfg_id: u32,
}

impl From<ffi::dcmi_create_vdev_out> for VChipOutput {
    fn from(vchip_out: ffi::dcmi_create_vdev_out) -> Self {
        VChipOutput {
            vchip_id: vchip_out.vdev_id,
            vfg_id: vchip_out.vfg_id,
        }
    }
}

/// Single device ID
///
/// Note:
/// ID cannot be 65535
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SingleDeviceId {
    /// ID
    id: u32,
}

impl SingleDeviceId {
    /// Create a new SingleDeviceId
    pub fn try_new(id: u32) -> DCMIResult<Self> {
        match id {
            65535 => Err(DCMIError::InvalidDeviceId),
            _ => Ok(SingleDeviceId { id }),
        }
    }

    /// Get the ID
    pub fn id(&self) -> u32 {
        self.id
    }
}
