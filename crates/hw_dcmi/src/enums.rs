//! Wrapped enums for the DCMI peripheral

use hw_dcmi_sys::bindings as ffi;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

/// Compute unit type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnitType {
    /// NPU
    NPU,
    /// MCU
    MCU,
    /// CPU
    CPU,
    /// Invalid type
    Invalid,
}

impl From<ffi::dcmi_unit_type> for UnitType {
    fn from(unit: ffi::dcmi_unit_type) -> Self {
        match unit {
            ffi::dcmi_unit_type_NPU_TYPE => UnitType::NPU,
            ffi::dcmi_unit_type_MCU_TYPE => UnitType::MCU,
            ffi::dcmi_unit_type_CPU_TYPE => UnitType::CPU,
            ffi::dcmi_unit_type_INVALID_TYPE => UnitType::Invalid,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

/// Die type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DieType {
    /// NDie
    NDie,
    /// VDie
    VDie,
}

impl From<ffi::dcmi_die_type> for DieType {
    fn from(die: ffi::dcmi_die_type) -> Self {
        match die {
            ffi::dcmi_die_type_NDIE => DieType::NDie,
            ffi::dcmi_die_type_VDIE => DieType::VDie,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

impl From<DieType> for ffi::dcmi_die_type {
    fn from(die: DieType) -> Self {
        match die {
            DieType::NDie => ffi::dcmi_die_type_NDIE,
            DieType::VDie => ffi::dcmi_die_type_VDIE,
        }
    }
}

/// Device type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceType {
    /// DDR
    DDR,
    /// SRAM
    SRAM,
    /// HBM
    HBM,
    /// NPU
    NPU,
    /// HBM recorded single address
    HBMRecordedSingleAddr,
    /// HBM recorded multi address
    HBMRecordedMultiAddr,
    /// None
    None,
}

impl From<ffi::dcmi_device_type> for DeviceType {
    fn from(device: ffi::dcmi_device_type) -> Self {
        match device {
            ffi::dcmi_device_type_DCMI_DEVICE_TYPE_DDR => DeviceType::DDR,
            ffi::dcmi_device_type_DCMI_DEVICE_TYPE_SRAM => DeviceType::SRAM,
            ffi::dcmi_device_type_DCMI_DEVICE_TYPE_HBM => DeviceType::HBM,
            ffi::dcmi_device_type_DCMI_DEVICE_TYPE_NPU => DeviceType::NPU,
            ffi::dcmi_device_type_DCMI_HBM_RECORDED_SINGLE_ADDR => {
                DeviceType::HBMRecordedSingleAddr
            }
            ffi::dcmi_device_type_DCMI_HBM_RECORDED_MULTI_ADDR => DeviceType::HBMRecordedMultiAddr,
            ffi::dcmi_device_type_DCMI_DEVICE_TYPE_NONE => DeviceType::None,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

impl From<DeviceType> for ffi::dcmi_device_type {
    fn from(device: DeviceType) -> Self {
        match device {
            DeviceType::DDR => ffi::dcmi_device_type_DCMI_DEVICE_TYPE_DDR,
            DeviceType::SRAM => ffi::dcmi_device_type_DCMI_DEVICE_TYPE_SRAM,
            DeviceType::HBM => ffi::dcmi_device_type_DCMI_DEVICE_TYPE_HBM,
            DeviceType::NPU => ffi::dcmi_device_type_DCMI_DEVICE_TYPE_NPU,
            DeviceType::HBMRecordedSingleAddr => {
                ffi::dcmi_device_type_DCMI_HBM_RECORDED_SINGLE_ADDR
            }
            DeviceType::HBMRecordedMultiAddr => ffi::dcmi_device_type_DCMI_HBM_RECORDED_MULTI_ADDR,
            DeviceType::None => ffi::dcmi_device_type_DCMI_DEVICE_TYPE_NONE,
        }
    }
}

/// Health state
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthState {
    /// Normal
    Normal,
    /// General alarm
    GeneralAlarm,
    /// Important alarm
    ImportantAlarm,
    /// Emergency alarm
    EmergencyAlarm,
    /// Device not found or not started
    DeviceNotFoundOrNotStarted,
}

impl From<u32> for HealthState {
    fn from(state: u32) -> Self {
        match state {
            0 => HealthState::Normal,
            1 => HealthState::GeneralAlarm,
            2 => HealthState::ImportantAlarm,
            3 => HealthState::EmergencyAlarm,
            0xffffffff => HealthState::DeviceNotFoundOrNotStarted,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

/// Frequency type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FrequencyType {
    /// DDR
    DDR,
    /// CtrlCpu
    CtrlCpu,
    /// HBM
    HBM,
    /// AICoreCurrent
    AICoreCurrent,
    /// AICoreMax, refers to the frequency at which it can operate continuously under TDP power consumption and scenarios
    AICoreMax,
    /// VectorCoreCurrent
    VectorCoreCurrent,
}

impl From<ffi::dcmi_freq_type> for FrequencyType {
    fn from(freq: ffi::dcmi_freq_type) -> Self {
        match freq {
            ffi::dcmi_freq_type_DCMI_FREQ_DDR => FrequencyType::DDR,
            ffi::dcmi_freq_type_DCMI_FREQ_CTRLCPU => FrequencyType::CtrlCpu,
            ffi::dcmi_freq_type_DCMI_FREQ_HBM => FrequencyType::HBM,
            ffi::dcmi_freq_type_DCMI_FREQ_AICORE_CURRENT_ => FrequencyType::AICoreCurrent,
            ffi::dcmi_freq_type_DCMI_FREQ_AICORE_MAX => FrequencyType::AICoreMax,
            ffi::dcmi_freq_type_DCMI_FREQ_VECTORCORE_CURRENT => FrequencyType::VectorCoreCurrent,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

impl From<FrequencyType> for ffi::dcmi_freq_type {
    fn from(freq: FrequencyType) -> Self {
        match freq {
            FrequencyType::DDR => ffi::dcmi_freq_type_DCMI_FREQ_DDR,
            FrequencyType::CtrlCpu => ffi::dcmi_freq_type_DCMI_FREQ_CTRLCPU,
            FrequencyType::HBM => ffi::dcmi_freq_type_DCMI_FREQ_HBM,
            FrequencyType::AICoreCurrent => ffi::dcmi_freq_type_DCMI_FREQ_AICORE_CURRENT_,
            FrequencyType::AICoreMax => ffi::dcmi_freq_type_DCMI_FREQ_AICORE_MAX,
            FrequencyType::VectorCoreCurrent => ffi::dcmi_freq_type_DCMI_FREQ_VECTORCORE_CURRENT,
        }
    }
}

/// Utilization type
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UtilizationType {
    /// Memory
    Memory,
    /// AI Core
    AICore,
    /// AI CPU
    AICpu,
    /// Control CPU
    CtrlCpu,
    /// Memory Bandwidth
    MemoryBandwidth,
    /// HBM
    HBM,
    /// DDR
    DDR,
    /// HBM Bandwidth
    HbmBandwidth,
    /// Vector Core
    VectorCore,
}

impl From<i32> for UtilizationType {
    fn from(util: i32) -> Self {
        match util {
            1 => UtilizationType::Memory,
            2 => UtilizationType::AICore,
            3 => UtilizationType::AICpu,
            4 => UtilizationType::CtrlCpu,
            5 => UtilizationType::MemoryBandwidth,
            6 => UtilizationType::HBM,
            8 => UtilizationType::DDR,
            10 => UtilizationType::HbmBandwidth,
            12 => UtilizationType::VectorCore,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}

impl UtilizationType {
    /// Convert the enum to the i32 which used by DCMI c library
    pub fn to_raw_value(&self) -> i32 {
        match self {
            UtilizationType::Memory => 1,
            UtilizationType::AICore => 2,
            UtilizationType::AICpu => 3,
            UtilizationType::CtrlCpu => 4,
            UtilizationType::MemoryBandwidth => 5,
            UtilizationType::HBM => 6,
            UtilizationType::DDR => 8,
            UtilizationType::HbmBandwidth => 10,
            UtilizationType::VectorCore => 12,
        }
    }
}
