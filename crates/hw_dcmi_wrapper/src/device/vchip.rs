//! Virtual chip of the DCMI

use crate::device::chip::Chip;
use crate::enums::{VChipCreateParam, VChipPowerSplittingMode};
use crate::error::{DCMIError, DCMIResult};
use crate::structs::VChipOutput;
use crate::DCMI;

/// Virtual chip of the DCMI
#[derive(Debug)]
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
    pub fn new_unchecked(chip: &'a Chip<'b, 'c>, id: u32, vfg_id: u32) -> Self {
        VChip { id, vfg_id, chip }
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

    /// Create a virtual chip
    ///
    /// # Warning
    /// if param is VChipCreateParam::SpecificId, make sure that the id is not 65535
    ///
    /// # Parameters
    /// - chip: the chip you want to create a virtual chip on
    /// - param: virtual chip creation parameters
    ///
    /// # Returns
    /// - out: output virtual chip info
    pub fn create(
        chip: &'a Chip<'b, 'c>,
        param: VChipCreateParam,
    ) -> DCMIResult<(VChipOutput, Self)> {
        if let VChipCreateParam::SpecificId { id, .. } = &param {
            if *id == 65535 {
                return Err(DCMIError::InvalidParameter);
            }
        }

        let mut vchip_out = unsafe { std::mem::zeroed() };

        let mut vchip_res = param.into();
        call_dcmi_function!(
            dcmi_create_vdevice,
            chip.card.dcmi.lib,
            chip.card.id as i32,
            chip.id as i32,
            &mut vchip_res,
            &mut vchip_out
        );

        let chip = VChip::new_unchecked(chip, vchip_out.vdev_id, vchip_out.vfg_id);

        Ok((vchip_out.into(), chip))
    }

    /// Destroy all virtual chips
    ///
    /// # Parameters
    /// - chip: the chip you want to destroy all virtual chips on
    pub fn destory_all(chip: &'a Chip<'b, 'c>) -> DCMIResult<()> {
        call_dcmi_function!(
            dcmi_set_destroy_vdevice,
            chip.card.dcmi.lib,
            chip.card.id as i32,
            chip.id as i32,
            65535
        );
        Ok(())
    }
}

impl VChip<'_, '_, '_> {
    /// Set the computing power splitting mode
    ///
    /// # Parameters
    /// - mode: computing power splitting mode
    ///
    /// # Warning
    /// make sure that no vchip is created before calling this function
    pub fn set_compute_power_splitting_mode(
        _dcmi: &DCMI,
        mode: VChipPowerSplittingMode,
    ) -> DCMIResult<()> {
        call_dcmi_function!(
            dcmi_set_vdevice_mode,
            _dcmi.lib,
            match mode {
                VChipPowerSplittingMode::Container => 0,
                VChipPowerSplittingMode::VM => 1,
            }
        );
        Ok(())
    }

    /// Query the computing power splitting mode
    ///
    /// # Returns
    /// computing power splitting mode
    pub fn get_compute_power_splitting_mode(_dcmi: &DCMI) -> DCMIResult<VChipPowerSplittingMode> {
        let mut mode = 0i32;
        call_dcmi_function!(dcmi_get_vdevice_mode, _dcmi.lib, &mut mode);
        Ok(match mode {
            0 => VChipPowerSplittingMode::Container,
            1 => VChipPowerSplittingMode::VM,
            _ => unreachable!("Not mentioned in the reference manual"),
        })
    }

    /// Set the vchip configuration recover mode
    ///
    /// # Parameters
    /// - mode: vchip configuration recover mode (0: disable, 1: enable)
    pub fn set_recovery_mode(_dcmi: &DCMI, mode: bool) -> DCMIResult<()> {
        call_dcmi_function!(dcmi_set_vnpu_config_recover_mode, _dcmi.lib, mode as u32);
        Ok(())
    }

    /// Query the vchip configuration recover mode
    ///
    /// # Returns
    /// vchip configuration recover mode (0: disable, 1: enable)
    pub fn get_recovery_mode(_dcmi: &DCMI) -> DCMIResult<bool> {
        let mut mode = 0u32;
        call_dcmi_function!(dcmi_get_vnpu_config_recover_mode, _dcmi.lib, &mut mode);
        Ok(match mode {
            0 => false,
            1 => true,
            _ => unreachable!("Not mentioned in the reference manual"),
        })
    }
}

impl VChip<'_, '_, '_> {
    /// Destroy VChip
    pub fn destroy(self) -> DCMIResult<()> {
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
