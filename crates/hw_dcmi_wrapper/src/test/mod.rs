use crate::device::card::Card;
use crate::device::vchip::VChip;
use crate::enums::VChipCreateParam;
use crate::DCMI;
use std::ops::Not;
use std::sync::{LazyLock, Mutex};

static DCMI_INSTANCE: LazyLock<Mutex<DCMI>> = LazyLock::new(|| Mutex::new(DCMI::init().unwrap()));
#[test]
#[ignore]
fn test_get_card_list() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    println!("card num: {}, card list: {:?}", card_list.len(), card_list);
}

#[test]
#[ignore]
fn test_get_memory_info() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    for card in card_list {
        let (chips, mcu_chip, cpu_chip) = card.get_chips().unwrap();
        println!(
            "chips: {:?}, mcu_chip: {:?}, cpu_chip: {:?}",
            chips, mcu_chip, cpu_chip
        );
        for chip in chips {
            let memory_info = chip.get_memory_info().unwrap();
            println!("chip memory info: {:?}", memory_info);
        }
    }
}

#[test]
#[ignore]
fn test_get_hbm_info() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    for card in card_list {
        let (chips, mcu_chip, cpu_chip) = card.get_chips().unwrap();
        println!(
            "chips: {:?}, mcu_chip: {:?}, cpu_chip: {:?}",
            chips, mcu_chip, cpu_chip
        );
        for chip in chips {
            let hbm_info = chip.get_hbm_info().unwrap();
            println!("chip hbm info: {:?}", hbm_info);
        }
    }
}

#[test]
#[ignore]
fn test_create_vchip() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    let vchip_out = VChip::create(
        chip,
        VChipCreateParam::TemplateName("vir03_1c_8g".to_string()),
    )
    .unwrap();
    println!("vchip_out: {:?}", vchip_out);
}

#[test]
#[ignore]
fn test_destroy_vchip() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    test_create_vchip();
    let vchips = VChip::new_unchecked(chip, 0, 0);
    vchips.destroy().unwrap();
}

#[test]
#[ignore]
fn test_destroy_self() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let card_list = Card::query_cards(dcmi).unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    let vchip_out = VChip::create(
        chip,
        VChipCreateParam::TemplateName("vir03_1c_8g".to_string()),
    )
    .unwrap();
    println!("vchip_out: {:?}", vchip_out);
    assert_eq!(vchip_out.0.vchip_id, vchip_out.1.id);
    assert_eq!(vchip_out.0.vfg_id, vchip_out.1.vfg_id);
    vchip_out.1.destroy().unwrap();
}

#[test]
#[ignore]
fn test_chip_mod() {
    let dcmi = &*DCMI_INSTANCE.lock().unwrap();
    let anti_mode = VChip::get_recovery_mode(dcmi).unwrap().not();
    VChip::set_recovery_mode(dcmi, anti_mode).unwrap();
    let new_mode = VChip::get_recovery_mode(dcmi).unwrap();
    assert_eq!(anti_mode, new_mode);
}
