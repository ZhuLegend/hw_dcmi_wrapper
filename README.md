# HW_DCMI_WRAPPER

[中文](./README_CN.md)

[Documentation](https://docs.rs/hw_dcmi_wrapper)

## Description

The HW_DCMI_WRAPPER project provides a set of third-party FFI (Foreign Function Interface) bindings for interacting with
Huawei DCMI. This repository contains two main packages:

1. **hw_dcmi_wrapper**: Provides safe and user-friendly bindings for integrating DCMI functionality into your
   applications.
2. **hw_dcmi_wrapper_sys**: Offers low-level, unsafe bindings that provide direct access to the underlying DCMI system
   calls.

## Getting Started

### Prerequisites

- Ubuntu 22.04 or later
- Atlas 6.0.0 or newer with DCMI support installed
- Required dependencies:
    - DCMI shared library
    - Ascend driver

### Default Configuration

By default, the library searches for DCMI components in the `/usr/local/dcmi` directory.
You can override this path by setting the `HW_DCMI_PATH` environment variable.

## Example Usage

- Before using any DCMI API, you need to initialize the library first. You can Initialize the library with `Dcmi::init`
- Most API interfaces are encapsulated to conform to ergonomics, such as `Card: Query_cards` returned a `Vec<Card>`.
- If you are confident that the parameters are correct, you can use `Card::new_uncheck`to create a `Card` instance.

Please refer to the document for details（ https://docs.rs/hw_dcmi_wrapper )

```rust
use hw_dcmi_wrapper::dcmi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize DCMI
    dcmi::init();

    // Create a new chip instance
    let chip = dcmi::Chip::new();

    // Query the system time from the chip
    let time = chip.get_system_time();
    println!("System Time: {}", time);

    Ok(())
}
```

## Version Strategy

- This project follows the [semantic version](https://semver.org/) Standardization
- The version number format for this project is `<major>< minor>.<patch>`， The rule for increasing version numbers is as
  follows:
    - Major: Incompatible API changes
    - Minor: Added functionality for backward compatibility
    - Patch: Fix backward compatibility issues

- When the project is in version '0.0.z', any change in version number may result in incompatible API changes.
- When the project is in version '0.y.z', only increasing 'y' will result in incompatible API changes.

## Project Status

Currently `Hw_dcmi_wrapper is **under development**, please refer to the API mapping table below for details.

Due to the instability of the DCMI API, this project will always be in version 0. y. z to adapt to possible changes in
the DCMI API.

## API Mapping

### Device Management Interface

| hw_dcmi_wrapper                            | DCMI API                             |
|--------------------------------------------|--------------------------------------|
| `dcmi_init`                                | `DCMI::new`                          |
| `dcmi_get_dcmi_version`                    | `DCMI::get_dcmi_version`             |
| `dcmi_get_driver_version`                  | `DCMI::get_driver_version`           |
| `dcmi_get_card_list`                       | `Card::query_cards`                  |
| `dcmi_get_device_num_in_card`              | `Card::get_chip_num`                 |
| `dcmi_get_device_id_in_card`               | `Card::get_chips`                    |
| `dcmi_get_device_type`                     | `Chip::get_type`                     |
| `dcmi_get_device_chip_info`                | `Chip::get_info`                     |
| `dcmi_get_device_pcie_info`                | `Chip::get_pcie_info`                |
| `dcmi_get_device_pcie_info_v2`             | `Chip::get_domain_pcie_info`         |
| `dcmi_get_device_board_info`               | `Chip::get_board_info`               |
| `dcmi_get_device_elabel_info`              | `Chip::get_elabel_info`              |
| `dcmi_get_device_power_info`               | `Chip::get_power_info`               |
| `dcmi_get_device_die_v2`                   | `Chip::get_die_info`                 |
| `dcmi_get_device_health`                   | `Chip::get_health`                   |
| `dcmi_get_driver_health`                   | `DCMI::get_driver_health`            |
| `dcmi_get_device_errorcode_v2`             | `Chip::get_error_code`               |
| `dcmi_get_driver_errorcode`                | `DCMI::get_driver_error_code`        |
| `dcmi_get_device_errorcode_string`         | `Chip::get_error_code_string`        |
| `dcmi_get_device_flash_count`              | `Chip::get_flash_count`              |
| `dcmi_get_device_flash_info_v2`            | `Chip::get_flash_info`               |
| `dcmi_get_device_aicore_info`              | `Chip::get_ai_core_info`             |
| `dcmi_get_device_aicpu_info`               | `Chip::get_ai_cpu_info`              |
| `dcmi_get_device_system_time`              | `Chip::get_system_time`              |
| `dcmi_get_device_temperature`              | `Chip::get_temperature`              |
| `dcmi_get_device_voltage`                  | `Chip::get_voltage`                  |
| `dcmi_get_device_pcie_error_cnt`           | `Chip::get_pcie_error_cnt`           |
| `dcmi_get_device_ecc_info`                 | `Chip::get_ecc_info`                 |
| `dcmi_get_device_frequency`                | `Chip::get_frequency`                |
| `dcmi_get_device_hbm_info`                 | `Chip::get_hbm_info`                 |
| `dcmi_get_device_memory_info_v3`           | `Chip::get_memory_info`              |
| `dcmi_get_device_utilization_rate`         | `Chip::get_utilization_rate`         |
| `dcmi_get_device_sensor_info`              | `Chip::get_sensor_info`              |
| `dcmi_set_container_service_enable`        | `DCMI::set_container_service_enable` |
| `dcmi_get_device_board_id`                 | `Chip::get_board_id`                 |
| `dcmi_get_device_component_count`          | `Chip::get_component_count`          |
| `dcmi_get_device_component_list`           | `Chip::get_component_list`           |
| `dcmi_get_device_component_static_version` | `Chip::get_component_static_version` |
| `dcmi_get_device_cgroup_info`              | `Chip::get_cgroup_info`              |
| `dcmi_get_device_llc_perf_para`            | `Chip::get_llc_perf`                 |

### Compute power splitting interface

| hw_dcmi_wrapper                     | DCMI API                                  |
|-------------------------------------|-------------------------------------------|
| `dcmi_set_vdevice_mode`             | `VChip::set_compute_power_splitting_mode` |
| `dcmi_get_vdevice_mode`             | `VChip::get_compute_power_splitting_mode` |
| `dcmi_create_vdevice`               | `VChip::create`                           |
| `dcmi_set_vnpu_config_recover_mode` | `VChip::set_recovery_mode`                |
| `dcmi_get_vnpu_config_recover_mode` | `VChip::get_recovery_mode`                |
| `dcmi_set_destroy_vdevice`          | `VChip::destroy` 与 `VChip::destory_all`   |

## License

This project is available under the MIT or Apache-2.0 license.