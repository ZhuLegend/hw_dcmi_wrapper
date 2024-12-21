//! Device of the DCMI.

use crate::enums::{
    DestroyVChipTarget, DeviceType, DieType, FrequencyType, HealthState, UnitType, UtilizationType,
};
use crate::error::{dcmi_try, DCMIError, DCMIResult, GetDataError};
use crate::structs::{
    AICPUInfo, AICoreInfo, BoardInfo, ChipInfo, ChipPCIEErrorRate, DieInfo, DomainPCIEInfo,
    ECCInfo, ELabelInfo, FlashInfo, HBMInfo, MemoryInfo, PCIEInfo, VChipOutput, VChipRes,
};
use crate::DCMI;
#[cfg(not(feature = "load_dynamic"))]
use hw_dcmi_sys::bindings as ffi;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use std::ffi::CStr;

/// Npu management unit
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Card<'a> {
    #[cfg_attr(not(feature = "load_dynamic"), allow(dead_code))]
    pub(crate) dcmi: &'a DCMI,
    pub(crate) id: u32,
}

impl Card<'_> {
    /// Create a new card
    ///
    /// # Warning
    /// It is your responsibility to ensure that the card ID is valid
    pub fn new_unchecked(dcmi: &DCMI, id: u32) -> Card {
        Card { dcmi, id }
    }

    /// Query the ID of this card
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Card<'_> {
    /// Query number of NPU chip in specific NPU management unit
    ///
    /// # Returns
    /// number of NPU chip
    pub fn get_chip_num(&self) -> DCMIResult<u32> {
        let mut device_num = 0i32;

        call_dcmi_function!(
            dcmi_get_device_num_in_card,
            self.dcmi.lib,
            self.id as i32,
            &mut device_num
        );

        Ok(device_num as u32)
    }

    /// Get the (NPU chip list, MCU chip, CPU chip) of the specified NPU management unit
    ///
    /// # Returns
    /// each element of return tuple means:
    /// - Vec<Chip>: NPU chip list
    /// - Option<Chip>: MCU chip, if there is no MCU chip, it will be None
    /// - Option<Chip>: CPU chip, if there is no CPU chip, it will be None
    pub fn get_chips(&self) -> DCMIResult<(Vec<Chip>, Option<Chip>, Option<Chip>)> {
        let mut device_id_max = 0i32;
        let mut mcu_id = 0i32;
        let mut cpu_id = 0i32;

        call_dcmi_function!(
            dcmi_get_device_id_in_card,
            self.dcmi.lib,
            self.id as i32,
            &mut device_id_max,
            &mut mcu_id,
            &mut cpu_id
        );

        let npu_chips = (0..device_id_max)
            .into_iter()
            .map(|id| Chip {
                card: &self,
                id: id as u32,
                unit_type: Some(UnitType::NPU),
            })
            .collect::<Vec<_>>();
        let mcu_chip = if mcu_id != -1 {
            Some(Chip {
                card: &self,
                id: mcu_id as u32,
                unit_type: Some(UnitType::MCU),
            })
        } else {
            None
        };
        let cpu_chip = if cpu_id != -1 {
            Some(Chip {
                card: &self,
                id: cpu_id as u32,
                unit_type: Some(UnitType::CPU),
            })
        } else {
            None
        };

        Ok((npu_chips, mcu_chip, cpu_chip))
    }
}

/// Chip of the DCMI
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chip<'a, 'b>
where
    'b: 'a,
{
    pub(crate) id: u32,
    pub(crate) card: &'a Card<'b>,
    pub(crate) unit_type: Option<UnitType>,
}

impl<'a, 'b> Chip<'a, 'b>
where
    'b: 'a,
{
    /// Create a new chip
    ///
    /// # Warning
    /// It is your responsibility to ensure that the chip ID is valid
    pub fn new_unchecked(card: &'a Card<'b>, chip_id: u32) -> Self {
        Chip {
            id: chip_id,
            card,
            unit_type: None,
        }
    }

    /// Query the ID of this chip
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Query the card of this chip
    ///
    /// # Returns
    /// card
    pub fn card(&self) -> &Card {
        self.card
    }
}

impl Chip<'_, '_> {
    /// Query the type of this chip
    ///
    /// # Returns
    /// chip type
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_type(&self) -> DCMIResult<UnitType> {
        if let Some(unit_type) = self.unit_type {
            Ok(unit_type)
        } else {
            let mut unit_type = unsafe { std::mem::zeroed() };

            call_dcmi_function!(
                dcmi_get_device_type,
                self.card.dcmi.lib,
                self.card.id as i32,
                self.id as i32,
                &mut unit_type
            );

            Ok(unit_type.into())
        }
    }

    /// Query the chip information
    ///
    /// # Returns
    /// chip information
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_info(&self) -> DCMIResult<ChipInfo> {
        let mut chip_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_chip_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut chip_info
        );

        Ok(chip_info.into())
    }

    /// Query the PCIE information
    ///
    /// # Warning
    /// Only NPU chip has PCIE information
    ///
    /// # Returns
    /// PCIE information
    pub fn get_pcie_info(&self) -> DCMIResult<PCIEInfo> {
        let mut pcie_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_pcie_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut pcie_info
        );

        Ok(pcie_info.into())
    }

    /// Query the PCIE information with domain
    ///
    /// # Warning
    /// Only NPU chip has PCIE information
    ///
    /// # Returns
    /// PCIE information with domain
    ///
    /// # Notes
    /// Only NPU chip has domain information
    pub fn get_domain_pcie_info(&self) -> DCMIResult<DomainPCIEInfo> {
        let mut pcie_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_pcie_info_v2,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut pcie_info
        );

        Ok(pcie_info.into())
    }

    /// Query the board information
    ///
    /// # Returns
    /// board information
    ///
    /// # Notes
    /// when chip is NPU, only board_id and slot_id is valid, slot_id tagged the pcie slot where chip is located
    ///
    /// when chip is MCU, all fields are valid, slot_id tagged the position of card where chip is located
    ///
    /// Only NPU and MCU chip support this function
    pub fn get_board_info(&self) -> DCMIResult<BoardInfo> {
        let mut board_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_board_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut board_info
        );

        Ok(board_info.into())
    }

    /// Query the ELabel information
    ///
    /// # Returns
    /// ELabel information
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_elabel_info(&self) -> DCMIResult<ELabelInfo> {
        let mut elabel_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_elabel_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut elabel_info
        );

        Ok(elabel_info.into())
    }

    /// Query the power information
    ///
    /// # Returns
    /// power information, unit: 0.1W
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_power_info(&self) -> DCMIResult<u32> {
        let mut power_info = 0i32;

        call_dcmi_function!(
            dcmi_get_device_power_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut power_info
        );

        Ok(power_info as u32)
    }

    /// Query the die information
    ///
    /// # Parameters
    /// - target: Die type
    ///
    /// # Returns
    /// die information
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_die_info(&self, target: DieType) -> DCMIResult<DieInfo> {
        let mut die_id = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_die_v2,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            target.into(),
            &mut die_id
        );

        Ok(die_id.into())
    }

    /// Query the health information
    ///
    /// # Returns
    /// health information, [HealthState::DeviceNotFoundOrNotStarted]
    /// will not be returned in this function different from [DCMI::get_driver_health],
    ///
    /// Instead, [DCMIError::DeviceNotExist] error will be thrown (always because use [Chip::new_unchecked] to create chip)
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_health(&self) -> DCMIResult<HealthState> {
        let mut health = 0;

        call_dcmi_function!(
            dcmi_get_device_health,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut health
        );

        if health == 0xFFFFFFFF {
            Err(DCMIError::DeviceNotExist)
        } else {
            Ok(health.into())
        }
    }

    /// Query the error code list
    ///
    /// # Returns
    /// error code list
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_error_code(&self) -> DCMIResult<Vec<u32>> {
        let mut error_code_list = [0u32, 128];
        let mut error_count = 0i32;

        call_dcmi_function!(
            dcmi_get_device_errorcode_v2,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut error_count,
            error_code_list.as_mut_ptr(),
            128
        );

        Ok(error_code_list
            .into_iter()
            .take(error_count as usize)
            .collect())
    }

    /// Query the error code string
    ///
    /// # Parameters
    /// - error_code: error code
    /// - query_simplified_info: should query simplified information
    ///     - true: get simplified information
    ///     - false: get detailed information
    ///
    /// # Returns
    /// error code string
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_error_code_string(
        &self,
        error_code: u32,
        query_simplified_info: bool,
    ) -> DCMIResult<String> {
        let mut error_code_string = [0u8; 256];
        let len = if query_simplified_info { 48 } else { 256 };

        call_dcmi_function!(
            dcmi_get_device_errorcode_string,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            error_code,
            error_code_string.as_mut_ptr(),
            len
        );

        Ok(CStr::from_bytes_until_nul(&error_code_string)
            .unwrap()
            .to_str()?
            .into())
    }

    /// Query the flash count
    ///
    /// # Returns
    /// flash count
    ///
    /// # Notes
    /// Only NPU chip support this function
    ///
    /// This interface is not supported in the scenario of computing power splitting containers
    pub fn get_flash_count(&self) -> DCMIResult<u32> {
        let mut flash_count = 0u32;

        call_dcmi_function!(
            dcmi_get_device_flash_count,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut flash_count
        );

        Ok(flash_count)
    }

    /// Query the flash information
    ///
    /// # Parameters
    /// - flash_id: flash ID, range: `0..<flash_count`
    ///
    /// # Returns
    /// flash information
    ///
    /// # Notes
    /// Only NPU chip support this function
    ///
    /// This interface is not supported in the scenario of computing power splitting containers
    pub fn get_flash_info(&self, flash_id: u32) -> DCMIResult<FlashInfo> {
        let mut flash_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_flash_info_v2,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            flash_id,
            &mut flash_info
        );

        Ok(flash_info.into())
    }

    /// Query the AI core information
    ///
    /// # Returns
    /// AI core information
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_ai_core_info(&self) -> DCMIResult<AICoreInfo> {
        let mut ai_core_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_aicore_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut ai_core_info
        );

        Ok(ai_core_info.into())
    }

    /// Query the AI CPU information
    ///
    /// # Returns
    /// AI CPU information
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_ai_cpu_info(&self) -> DCMIResult<AICPUInfo> {
        let mut ai_cpu_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_aicpu_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut ai_cpu_info
        );

        Ok(ai_cpu_info.into())
    }

    /// Query the system time
    ///
    /// # Returns
    /// system time, the second value from 00:00:00 on January 1, 1970 to present
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_system_time(&self) -> DCMIResult<u32> {
        let mut system_time = 0u32;

        call_dcmi_function!(
            dcmi_get_device_system_time,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut system_time
        );

        Ok(system_time)
    }

    /// Query the temperature
    ///
    /// # Returns
    /// temperature, unit: 1 degree Celsius
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_temperature(&self) -> DCMIResult<i32> {
        let mut temperature = 0i32;

        call_dcmi_function!(
            dcmi_get_device_temperature,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut temperature
        );

        Ok(check_value!(temperature)?)
    }

    /// Query device voltage
    ///
    /// # Returns
    /// voltage, unit: 0.01V
    ///
    /// # Notes
    /// Only NPU and MCU chip support this function
    pub fn get_voltage(&self) -> DCMIResult<u32> {
        let mut voltage = 0u32;

        call_dcmi_function!(
            dcmi_get_device_voltage,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut voltage
        );

        Ok(check_value!(voltage)?)
    }

    /// Query the PCIE error count
    ///
    /// # Returns
    /// PCIE error count
    ///
    /// # Notes
    /// Only NPU chip support this function
    pub fn get_pcie_error_cnt(&self) -> DCMIResult<ChipPCIEErrorRate> {
        let mut pcie_error_rate = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_pcie_error_cnt,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut pcie_error_rate
        );

        Ok(pcie_error_rate.into())
    }

    /// Query the ECC information
    ///
    /// # Parameters
    /// - target: device type, only support [DDR](DeviceType::DDR) and [HBM](DeviceType::HBM)
    ///
    /// # Returns
    /// ECC information
    pub fn get_ecc_info(&self, target: DeviceType) -> DCMIResult<ECCInfo> {
        let mut ecc_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_ecc_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            target.into(),
            &mut ecc_info
        );

        Ok(ecc_info.into())
    }

    /// Query the frequency
    ///
    /// # Parameters
    /// - target: frequency type, only support
    /// [DDR](FrequencyType::DDR),
    /// [CtrlCpu](FrequencyType::CtrlCpu),
    /// [HBM](FrequencyType::HBM),
    /// [AICoreCurrent](FrequencyType::AICoreCurrent),
    /// [AICoreMax](FrequencyType::AICoreMax)
    /// currently
    /// # Returns
    /// frequency, unit: 1MHz
    pub fn get_frequency(&self, target: FrequencyType) -> DCMIResult<u32> {
        let mut frequency = 0u32;

        call_dcmi_function!(
            dcmi_get_device_frequency,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            target.into(),
            &mut frequency
        );

        Ok(frequency)
    }

    /// Query the HBM information
    ///
    /// # Warning
    /// Only some of NPU chip has HBM information
    ///
    /// # Returns
    /// HBM information
    pub fn get_hbm_info(&self) -> DCMIResult<HBMInfo> {
        let mut hbm_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(
            dcmi_get_device_hbm_info,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut hbm_info
        );

        Ok(hbm_info.into())
    }

    /// Query the memory information
    ///
    /// # Warning
    /// Only some of NPU chip has memory information
    ///
    /// # Returns
    /// memory information
    pub fn get_memory_info(&self) -> DCMIResult<MemoryInfo> {
        let mut memory_info = unsafe { std::mem::zeroed() };
        println!(
            "query memory with card id: {}, chip id: {}",
            self.card.id, self.id
        );
        call_dcmi_function!(
            dcmi_get_device_memory_info_v3,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut memory_info
        );

        Ok(memory_info.into())
    }

    /// Query the utilization rate
    ///
    /// # Parameters
    /// - target: utilization type, only support [Memory](UtilizationType::Memory),
    /// [AI Core](UtilizationType::AICore),
    /// [AI CPU](UtilizationType::AICpu),
    /// [Control CPU](UtilizationType::CtrlCpu),
    /// [Memory Bandwidth](UtilizationType::MemoryBandwidth),
    /// [HBM](UtilizationType::HBM),
    /// [HBM Bandwidth](UtilizationType::HbmBandwidth),
    ///
    /// # Returns
    /// utilization rate, unit: 1%
    ///
    /// # Notes
    /// When device type is AI CORE, profiling is enabled, and the utilization rate is queried as 0, which is actually meaningless.
    ///
    /// In the scenario of computing power splitting, only [Memory](UtilizationType::Memory) and [HBM Bandwidth](UtilizationType::HbmBandwidth) are supported in the container. Other parameters are not supported.
    /// In this scenario, the HBM bandwidth obtained is 0, which is actually meaningless.
    pub fn get_utilization_rate(&self, target: UtilizationType) -> DCMIResult<u32> {
        let mut utilization_rate = 0u32;

        call_dcmi_function!(
            dcmi_get_device_utilization_rate,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            target.to_raw_value(),
            &mut utilization_rate
        );

        Ok(utilization_rate)
    }

    /// Create a virtual chip
    ///
    /// # Parameters
    /// - vdev: virtual chip info
    ///
    /// # Returns
    /// - out: output virtual chip info
    pub fn create_virtual_chip(&self, vdev: VChipRes) -> DCMIResult<(VChipOutput, VChip)> {
        let mut vchip_out = unsafe { std::mem::zeroed() };

        let mut vchip_res = vdev.into();
        call_dcmi_function!(
            dcmi_create_vdevice,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            &mut vchip_res,
            &mut vchip_out
        );
        Ok((
            vchip_out.into(),
            VChip {
                id: vchip_out.vdev_id,
                vfg_id: vchip_out.vfg_id,
                chip: self,
            },
        ))
    }

    /// Destroy a virtual chip
    ///
    /// # Parameters
    /// - destroy_mode : destroy mode
    pub fn destroy_virtual_chip(&self, destroy_mode: DestroyVChipTarget) -> DCMIResult<()> {
        call_dcmi_function!(
            dcmi_set_destroy_vdevice,
            self.card.dcmi.lib,
            self.card.id as i32,
            self.id as i32,
            destroy_mode.to_raw_param()
        );
        Ok(())
    }
}

/// Virtual chip of the DCMI
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VChip<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    pub(crate) id: u32,
    pub(crate) vfg_id: u32,
    pub(crate) chip: &'a Chip<'b, 'c>,
}

impl<'a, 'b, 'c> VChip<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    /// Create a new virtual chip
    ///
    /// # Warning
    /// It is your responsibility to ensure that the virtual chip ID is valid
    pub fn new_unchecked(chip: &'a Chip<'b, 'c>, vchip_id: u32, vfg_id: u32) -> Self {
        VChip {
            id: vchip_id,
            vfg_id,
            chip,
        }
    }

    /// Query the ID of this virtual chip
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Query the group ID of this virtual chip
    pub fn vfg_id(&self) -> u32 {
        self.vfg_id
    }

    /// Query the chip of this virtual chip
    ///
    /// # Returns
    /// chip
    pub fn chip(&self) -> &Chip {
        self.chip
    }
}

impl VChip<'_, '_, '_> {
    /// Destroy self
    pub fn destroy_self(&self) -> DCMIResult<()> {
        call_dcmi_function!(
            dcmi_set_destroy_vdevice,
            self.chip.card.dcmi.lib,
            self.chip.card.id as i32,
            self.chip.id as i32,
            self.id
        );
        Ok(())
    }
}
