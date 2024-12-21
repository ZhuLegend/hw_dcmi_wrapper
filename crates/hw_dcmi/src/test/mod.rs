use crate::enums::DestroyVChipTarget;
use crate::structs::VChipRes;
use crate::DCMI;
use std::ops::Not;
use std::sync::LazyLock;

static DCMI_INSTANCE: LazyLock<DCMI> = LazyLock::new(|| DCMI::init().unwrap());
#[test]
fn test_get_card_list() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
    println!("card num: {}, card list: {:?}", card_list.len(), card_list);
}

#[test]
fn test_get_memory_info() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
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
fn test_get_hbm_info() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
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
fn test_create_vchip() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    let vchip_res = VChipRes::new("vir03_1c_8g".to_string());
    let vchip_out = chip.create_virtual_chip(vchip_res).unwrap();
    println!("vchip_out: {:?}", vchip_out);
}

#[test]
fn test_destroy_vchip() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    test_create_vchip();
    let destroy_target = DestroyVChipTarget::single_device(100).unwrap();
    chip.destroy_virtual_chip(destroy_target).unwrap();
}

#[test]
fn test_destroy_self() {
    let dcmi = &*DCMI_INSTANCE;
    let card_list = dcmi.get_card_list().unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    let vchip_res = VChipRes::new("vir03_1c_8g".to_string());
    let vchip_out = chip.create_virtual_chip(vchip_res).unwrap();
    println!("vchip_out: {:?}", vchip_out);
    assert_eq!(vchip_out.0.vchip_id, vchip_out.1.id);
    assert_eq!(vchip_out.0.vfg_id, vchip_out.1.vfg_id);
    vchip_out.1.destroy_self().unwrap();
}

#[test]
fn test_chip_mod() {
    let dcmi = &*DCMI_INSTANCE;
    let anti_mode = dcmi.get_vchip_recover_mode().unwrap().not();
    dcmi.set_vchip_recover_mode(anti_mode).unwrap();
    let new_mode = dcmi.get_vchip_recover_mode().unwrap();
    assert_eq!(anti_mode, new_mode);
}
