//! NPU management unit

use crate::device::chip::Chip;
use crate::enums::UnitType;
use crate::error::DCMIResult;
use crate::DCMI;

/// Npu management unit
#[derive(Debug)]
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
    /// Query the number of NPU units and the id of each NPU unit
    ///
    /// # Returns
    /// NPU management unit ID list
    pub fn query_cards(dcmi: &DCMI) -> DCMIResult<Vec<Card>> {
        let mut card_num = 0i32;
        let mut card_list = [-1i32; 64];
        let len = card_list.len() as i32;

        call_dcmi_function!(
            dcmi_get_card_list,
            dcmi.lib,
            &mut card_num,
            card_list.as_mut_ptr(),
            len
        );

        Ok(card_list
            .into_iter()
            .take(card_num as usize)
            .map(|id| Card {
                dcmi,
                id: id as u32,
            })
            .collect())
    }

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
