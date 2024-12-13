use crate::structs::VChipRes;
use crate::DCMI;

#[test]
fn test_get_card_list() {
    let dcmi = DCMI::init().unwrap();
    let card_list = dcmi.get_card_list().unwrap();
    println!("card num: {}, card list: {:?}", card_list.len(), card_list);
}

#[test]
fn test_get_memory_info() {
    let dcmi = DCMI::init().unwrap();
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
    let dcmi = DCMI::init().unwrap();
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
    let dcmi = DCMI::init().unwrap();
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
    let dcmi = DCMI::init().unwrap();
    let card_list = dcmi.get_card_list().unwrap();
    let card = card_list.first().unwrap();
    let (chips, _mcu_chip, _cpu_chip) = card.get_chips().unwrap();
    let chip = chips.first().unwrap();
    test_create_vchip();
    chip.destroy_virtual_chip(65535).unwrap();
}

#[test]
fn test_chip_mod() {
    let dcmi = DCMI::init().unwrap();
    let anti_mode = match dcmi.get_vchip_recover_mode().unwrap() {
        0 => 1u32,
        _ => 0u32,
    };
    dcmi.set_vchip_recover_mode(anti_mode).unwrap();
    let new_mode = (dcmi.get_vchip_recover_mode().unwrap() != 0) as u32;
    assert_eq!(anti_mode, new_mode);
}
