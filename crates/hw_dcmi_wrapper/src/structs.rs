//! Wrapped structs for the DCMI peripheral

use hw_dcmi_wrapper_sys::bindings as ffi;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
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

/// Create VChip output
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VChipOutput {
    /// VChip ID
    pub vchip_id: u32,
    /// PCIE bus
    pub pcie_bus: u32,
    /// PCIE device
    pub pcie_device: u32,
    /// PCIE function
    pub pcie_func: u32,
    /// VChip group ID
    pub vfg_id: u32,
}

impl From<ffi::dcmi_create_vdev_out> for VChipOutput {
    fn from(vchip_out: ffi::dcmi_create_vdev_out) -> Self {
        VChipOutput {
            vchip_id: vchip_out.vdev_id,
            pcie_bus: vchip_out.pcie_bus,
            pcie_device: vchip_out.pcie_device,
            pcie_func: vchip_out.pcie_func,
            vfg_id: vchip_out.vfg_id,
        }
    }
}

/// Manager Sensor ID
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ManagerSensorId {
    /// Cluster temperature
    ClusterTemperature = 0,
    /// Peripheral temperature
    PeripheralTemperature,
    /// AI core 0 temperature
    AiCore0Temperature,
    /// AI core 1 temperature
    AiCore1Temperature,
    /// AI core Limit
    AiCoreLimit,
    /// AI Core Total Per
    AiCoreTotalPer,
    /// AI Core Elim Per
    AiCoreElimPer,
    /// AI Core Base Frequency
    AiCoreBaseFreq,
    /// NPU DDR Frequency
    NpuDdrFreq,
    /// Thermal Threshold
    ThermalThreshold,
    /// NTC Temperature
    NtcTemperature,
    /// SOC Temperature
    SocTemperature,
    /// FP Temperature
    FpTemperature,
    /// N Die Temperature
    NDieTemperature,
    /// HBM Temperature
    HbmTemperature,
    /// Invalid
    Invalid = 255,
}

impl ManagerSensorId {
    /// Query result type
    ///
    /// [DCMI doc reference](https://support.huawei.com/enterprise/zh/doc/EDOC1100349020/8b969022)
    pub fn query_result_type(&self) -> SensorInfoType {
        match self {
            ManagerSensorId::ClusterTemperature
            | ManagerSensorId::PeripheralTemperature
            | ManagerSensorId::AiCore0Temperature
            | ManagerSensorId::AiCore1Temperature
            | ManagerSensorId::AiCoreLimit
            | ManagerSensorId::AiCoreTotalPer
            | ManagerSensorId::AiCoreElimPer
            | ManagerSensorId::SocTemperature => SensorInfoType::UChar,
            ManagerSensorId::AiCoreBaseFreq | ManagerSensorId::NpuDdrFreq => SensorInfoType::UShort,
            ManagerSensorId::ThermalThreshold => SensorInfoType::Temp,
            ManagerSensorId::NtcTemperature => SensorInfoType::NtcTmp,
            ManagerSensorId::FpTemperature
            | ManagerSensorId::NDieTemperature
            | ManagerSensorId::HbmTemperature => SensorInfoType::Int,
            _ => unreachable!("Invalid ManagerSensorId should never be used"),
        }
    }
}

impl From<ManagerSensorId> for ffi::dcmi_manager_sensor_id {
    fn from(value: ManagerSensorId) -> Self {
        match value {
            ManagerSensorId::ClusterTemperature => ffi::dcmi_manager_sensor_id_DCMI_CLUSTER_TEMP_ID,
            ManagerSensorId::PeripheralTemperature => ffi::dcmi_manager_sensor_id_DCMI_PERI_TEMP_ID,
            ManagerSensorId::AiCore0Temperature => ffi::dcmi_manager_sensor_id_DCMI_AICORE0_TEMP_ID,
            ManagerSensorId::AiCore1Temperature => ffi::dcmi_manager_sensor_id_DCMI_AICORE1_TEMP_ID,
            ManagerSensorId::AiCoreLimit => ffi::dcmi_manager_sensor_id_DCMI_AICORE_LIMIT_ID,
            ManagerSensorId::AiCoreTotalPer => ffi::dcmi_manager_sensor_id_DCMI_AICORE_TOTAL_PER_ID,
            ManagerSensorId::AiCoreElimPer => ffi::dcmi_manager_sensor_id_DCMI_AICORE_ELIM_PER_ID,
            ManagerSensorId::AiCoreBaseFreq => ffi::dcmi_manager_sensor_id_DCMI_AICORE_BASE_FREQ_ID,
            ManagerSensorId::NpuDdrFreq => ffi::dcmi_manager_sensor_id_DCMI_NPU_DDR_FREQ_ID,
            ManagerSensorId::ThermalThreshold => {
                ffi::dcmi_manager_sensor_id_DCMI_THERMAL_THRESHOLD_ID
            }
            ManagerSensorId::NtcTemperature => ffi::dcmi_manager_sensor_id_DCMI_NTC_TEMP_ID,
            ManagerSensorId::SocTemperature => ffi::dcmi_manager_sensor_id_DCMI_SOC_TEMP_ID,
            ManagerSensorId::FpTemperature => ffi::dcmi_manager_sensor_id_DCMI_FP_TEMP_ID,
            ManagerSensorId::NDieTemperature => ffi::dcmi_manager_sensor_id_DCMI_N_DIE_TEMP_ID,
            ManagerSensorId::HbmTemperature => ffi::dcmi_manager_sensor_id_DCMI_HBM_TEMP_ID,
            ManagerSensorId::Invalid => ffi::dcmi_manager_sensor_id_DCMI_SENSOR_INVALID_ID,
        }
    }
}

/// Sensor information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SensorInfo {
    /// Unsigned char sensor info
    UChar(u8),
    /// Unsigned short sensor info
    UShort(u16),
    /// Unsigned int sensor info
    UInt(u32),
    /// Int sensor info
    Int(i32),
    /// Temperature sensor info
    Temp([i8; 2]),
    /// NTC temperature sensor info
    NtcTmp([i32; 4]),
    /// Data sensor info
    Data([u32; 16]),
}

/// Sensor information type
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SensorInfoType {
    /// UChar
    UChar,
    /// UShort
    UShort,
    /// UInt
    UInt,
    /// Int
    Int,
    /// Temp
    Temp,
    /// NtcTmp
    NtcTmp,
    /// Data
    Data,
}

impl SensorInfo {
    /// Unsafe create a new SensorInfo from ffi::dcmi_sensor_info
    ///
    /// use this only when check
    /// [dcmi doc(API: dcmi_get_device_sensor_info)](https://support.huawei.com/enterprise/zh/doc/EDOC1100349020/8b969022)
    pub(crate) unsafe fn from_ffi_raw(
        sensor_info: ffi::dcmi_sensor_info,
        sensor_info_type: SensorInfoType,
    ) -> Self {
        match sensor_info_type {
            SensorInfoType::UChar => SensorInfo::UChar(sensor_info.uchar),
            SensorInfoType::UShort => SensorInfo::UShort(sensor_info.ushort),
            SensorInfoType::UInt => SensorInfo::UInt(sensor_info.uint),
            SensorInfoType::Int => SensorInfo::Int(sensor_info.iint),
            SensorInfoType::Temp => SensorInfo::Temp(sensor_info.temp),
            SensorInfoType::NtcTmp => SensorInfo::NtcTmp(sensor_info.ntc_tmp),
            SensorInfoType::Data => SensorInfo::Data(sensor_info.data),
        }
    }
}

/// Component type
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComponentType {
    /// Nve
    Nve,
    /// XLoader
    XLoader,
    /// M3FW
    M3FW,
    /// UEFI
    UEFI,
    /// TEE
    TEE,
    /// Kernel
    Kernel,
    /// DTB
    DTB,
    /// RootFS
    RootFS,
    /// IMU
    IMU,
    /// IMP
    IMP,
    /// AI CPU
    AiCPU,
    /// HBoot 1A
    HBoot1A,
    /// HBoot 1B
    HBoot1B,
    /// HBoot 2
    HBoot2,
    /// DDR
    DDR,
    /// LP
    LP,
    /// HSM
    HSM,
    /// Safety Island
    SafetyIsland,
    /// HiLink
    HiLink,
    /// Raw Data
    RawData,
    /// SysDrv
    SysDrv,
    /// Ads App
    AdsApp,
    /// Com Isolator
    ComIsolator,
    /// Cluster
    Cluster,
    /// Customized
    Customized,
    /// SysBaseConfig
    SysBaseConfig,
    /// SysRecovery
    Recovery,
    /// HiLink2
    HiLink2,
    /// Logic Bist
    LogicBist,
    /// Memory Bist
    MemoryBist,
    /// ATF
    ATF,
    /// User Base Config
    UserBaseConfig,
    /// BootROM
    BootROM,
    /// Max
    Max,
    /// Upgrade and reset all component
    UpgradeAndResetAllComponent,
    /// Upgrade all image component
    UpgradeAllImageComponent,
    /// Upgrade all firmware component
    UpgradeAllFirmwareComponent,
    /// Upgrade all component
    UpgradeAllComponent,
}

impl From<ffi::dcmi_component_type> for ComponentType {
    fn from(value: ffi::dcmi_component_type) -> Self {
        match value {
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_NVE => ComponentType::Nve,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_XLOADER => ComponentType::XLoader,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_M3FW => ComponentType::M3FW,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_UEFI => ComponentType::UEFI,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_TEE => ComponentType::TEE,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_KERNEL => ComponentType::Kernel,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_DTB => ComponentType::DTB,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ROOTFS => ComponentType::RootFS,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_IMU => ComponentType::IMU,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_IMP => ComponentType::IMP,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_AICPU => ComponentType::AiCPU,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT1_A => ComponentType::HBoot1A,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT1_B => ComponentType::HBoot1B,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT2 => ComponentType::HBoot2,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_DDR => ComponentType::DDR,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_LP => ComponentType::LP,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HSM => ComponentType::HSM,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SAFETY_ISLAND => {
                ComponentType::SafetyIsland
            }
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HILINK => ComponentType::HiLink,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_RAWDATA => ComponentType::RawData,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SYSDRV => ComponentType::SysDrv,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ADSAPP => ComponentType::AdsApp,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_COMISOLATOR => ComponentType::ComIsolator,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_CLUSTER => ComponentType::Cluster,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_CUSTOMIZED => ComponentType::Customized,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SYS_BASE_CONFIG => {
                ComponentType::SysBaseConfig
            }
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_RECOVERY => ComponentType::Recovery,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HILINK2 => ComponentType::HiLink2,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_LOGIC_BIST => ComponentType::LogicBist,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_MEMORY_BIST => ComponentType::MemoryBist,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ATF => ComponentType::ATF,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_USER_BASE_CONFIG => {
                ComponentType::UserBaseConfig
            }
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_BOOTROM => ComponentType::BootROM,
            ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_MAX => ComponentType::Max,
            ffi::dcmi_component_type_DCMI_UPGRADE_AND_RESET_ALL_COMPONENT => {
                ComponentType::UpgradeAndResetAllComponent
            }
            ffi::dcmi_component_type_DCMI_UPGRADE_ALL_IMAGE_COMPONENT => {
                ComponentType::UpgradeAllImageComponent
            }
            ffi::dcmi_component_type_DCMI_UPGRADE_ALL_FIRMWARE_COMPONENT => {
                ComponentType::UpgradeAllFirmwareComponent
            }
            ffi::dcmi_component_type_DCMI_UPGRADE_ALL_COMPONENT => {
                ComponentType::UpgradeAllComponent
            }
            _ => unreachable!("Invalid dcmi_component_type value"),
        }
    }
}

impl From<ComponentType> for ffi::dcmi_component_type {
    fn from(value: ComponentType) -> Self {
        match value {
            ComponentType::Nve => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_NVE,
            ComponentType::XLoader => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_XLOADER,
            ComponentType::M3FW => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_M3FW,
            ComponentType::UEFI => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_UEFI,
            ComponentType::TEE => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_TEE,
            ComponentType::Kernel => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_KERNEL,
            ComponentType::DTB => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_DTB,
            ComponentType::RootFS => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ROOTFS,
            ComponentType::IMU => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_IMU,
            ComponentType::IMP => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_IMP,
            ComponentType::AiCPU => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_AICPU,
            ComponentType::HBoot1A => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT1_A,
            ComponentType::HBoot1B => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT1_B,
            ComponentType::HBoot2 => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HBOOT2,
            ComponentType::DDR => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_DDR,
            ComponentType::LP => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_LP,
            ComponentType::HSM => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HSM,
            ComponentType::SafetyIsland => {
                ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SAFETY_ISLAND
            }
            ComponentType::HiLink => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HILINK,
            ComponentType::RawData => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_RAWDATA,
            ComponentType::SysDrv => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SYSDRV,
            ComponentType::AdsApp => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ADSAPP,
            ComponentType::ComIsolator => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_COMISOLATOR,
            ComponentType::Cluster => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_CLUSTER,
            ComponentType::Customized => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_CUSTOMIZED,
            ComponentType::SysBaseConfig => {
                ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_SYS_BASE_CONFIG
            }
            ComponentType::Recovery => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_RECOVERY,
            ComponentType::HiLink2 => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_HILINK2,
            ComponentType::LogicBist => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_LOGIC_BIST,
            ComponentType::MemoryBist => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_MEMORY_BIST,
            ComponentType::ATF => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_ATF,
            ComponentType::UserBaseConfig => {
                ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_USER_BASE_CONFIG
            }
            ComponentType::BootROM => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_BOOTROM,
            ComponentType::Max => ffi::dcmi_component_type_DCMI_COMPONENT_TYPE_MAX,
            ComponentType::UpgradeAndResetAllComponent => {
                ffi::dcmi_component_type_DCMI_UPGRADE_AND_RESET_ALL_COMPONENT
            }
            ComponentType::UpgradeAllImageComponent => {
                ffi::dcmi_component_type_DCMI_UPGRADE_ALL_IMAGE_COMPONENT
            }
            ComponentType::UpgradeAllFirmwareComponent => {
                ffi::dcmi_component_type_DCMI_UPGRADE_ALL_FIRMWARE_COMPONENT
            }
            ComponentType::UpgradeAllComponent => {
                ffi::dcmi_component_type_DCMI_UPGRADE_ALL_COMPONENT
            }
        }
    }
}

/// CGroup information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CGroupInfo {
    /// Memory limit in bytes
    pub limit_in_bytes: u64,
    /// Maximum usage in bytes
    pub max_usage_in_bytes: u64,
    /// Usage in bytes
    pub usage_in_bytes: u64,
}

impl From<ffi::dcmi_cgroup_info> for CGroupInfo {
    fn from(cgroup_info: ffi::dcmi_cgroup_info) -> Self {
        CGroupInfo {
            limit_in_bytes: cgroup_info.limit_in_bytes,
            max_usage_in_bytes: cgroup_info.max_usage_in_bytes,
            usage_in_bytes: cgroup_info.usage_in_bytes,
        }
    }
}

/// LLC Performance
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LLCPerf {
    /// LLC write hit rate, unit: %
    pub write_hit_rate: u32,
    /// LLC read hit rate, unit: %
    pub read_hit_rate: u32,
    /// LLC throughput, unit: KB/s
    pub throughput: u32,
}

impl From<ffi::dcmi_llc_perf> for LLCPerf {
    fn from(llc_pref: ffi::dcmi_llc_perf) -> Self {
        LLCPerf {
            write_hit_rate: llc_pref.wr_hit_rate,
            read_hit_rate: llc_pref.rd_hit_rate,
            throughput: llc_pref.throughput,
        }
    }
}
