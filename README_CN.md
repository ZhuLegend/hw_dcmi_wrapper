# hw_dcmi_wrapper

[![Crates.io version](https://img.shields.io/crates/v/hw_dcmi_wrapper.svg?style=flat-square)](https://crates.io/crates/hw_dcmi_wrapper)
[![Crates.io downloads](https://img.shields.io/crates/d/hw_dcmi_wrapper.svg?style=flat-square)](https://crates.io/crates/hw_dcmi_wrapper)
[![Docs.rs docs](https://docs.rs/hw_dcmi_wrapper/badge.svg)](https://docs.rs/hw_dcmi_wrapper)

华为昇腾计算设备**第三方**DCMI c 库 **安全** FFI绑定

- hw_dcmi_wrapper提供safe的FFI绑定(由hw_dcmi_sys提供的FFI绑定封装而成)
- hw_dcmi_wrapper_sys提供unsafe的FFI绑定(由bindgen直接生成)

## 使用方法

### 先决条件

项目在Ubuntu 22.04上测试过，使用Atlas 6.0.0 DCMI API，你需要安装以下依赖项:

- DCMI共享库
- 昇腾驱动

默认情况下，库将尝试在`/usr/local/dcmi`中查找`dcmi_interface_api.h`并链接`libdcmi.so`，
你可以提供`HW_DCMI_PATH`环境变量来指定dcmi共享库的路径。

如需重新生成绑定，请设置环境变量`HW_DCMI_BINDING_BUILD`为`true`，生成的绑定文件将保存至：

- 静态链接：`hw_dcmi_wrapper_sys/src/bindings.rs`
- 动态链接：`hw_dcmi_wrapper_sys/src/bindings_dyn.rs`

### 示例

- 在使用任何dcmi api之前，需要先初始化库，你可以使用`DCMI::init`来初始化库。
- 多数API接口均进行了封装以符合人体工程学，例如`Card::query_cards`返回了一个`Vec<Card>`。
- 如果你确信参数没有问题，你可以使用`Card::new_uncheck`来创建一个`Card`实例。

详情请查看[文档](https://docs.rs/hw_dcmi_wrapper)

```rust
use hw_dcmi_wrapper::dcmi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dcmi = DCMI::init().unwrap();

    let dcmi_version = dcmi.get_dcmi_version().unwrap();
    println!("DCMI version: {}", dcmi_version);

    let cards = Card::query_cards(&dcmi).unwrap();
    println!("Card list: {:?}", cards);
}
```

## 版本策略

- 本项目遵循[语义化版本](https://semver.org/lang/zh-CN/)规范
- 本项目的版本号格式为`<major>.<minor>.<patch>`，版本号递增规则如下:
    - major: 不兼容的API变动
    - minor: 向下兼容的功能性新增
    - patch: 向下兼容的问题修复

- 在项目处于`0.0.z`版本时，任何版本号的变动都可能会导致不兼容的API变动。
- 在项目处于`0.y.z`版本时，只有`y`递增时才会导致不兼容的API变动。

## 项目状态

当前，`hw_dcmi_wrapper`处于**开发中**状态，详情请见下文API映射表。

鉴于DCMI API的不稳定性，本项目的会始终处于0.y.z版本，以适应DCMI API可能的变化。

## API映射表

未在本映射表中列出的API，均未被封装，你可以通过`hw_dcmi_wrapper_sys`直接调用。
欢迎提交pr来增加更多API的封装。

暂未为文档中标记会被删除的API提供封装，后续可能会通过提供`legacy`特性来支持。

API来源文档: [Atlas 中心训练卡 23.0.x(23.0.3及其系列版本) DCMI API参考 07](https://support.huawei.com/enterprise/zh/doc/EDOC1100349020/82891499)

### 设备管理接口

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

### 算力切分接口

| hw_dcmi_wrapper                     | DCMI API                                  |
|-------------------------------------|-------------------------------------------|
| `dcmi_set_vdevice_mode`             | `VChip::set_compute_power_splitting_mode` |
| `dcmi_get_vdevice_mode`             | `VChip::get_compute_power_splitting_mode` |
| `dcmi_create_vdevice`               | `VChip::create`                           |
| `dcmi_set_vnpu_config_recover_mode` | `VChip::set_recovery_mode`                |
| `dcmi_get_vnpu_config_recover_mode` | `VChip::get_recovery_mode`                |
| `dcmi_set_destroy_vdevice`          | `VChip::destroy` 与 `VChip::destory_all`   |

## Lisence

本项目遵循双重许可，您可选择遵循 [Apache 2.0 许可证](./LICENSE-APACHE) 或 [MIT 许可证](./LICENSE-MIT)。

---

除非您明确声明，否则依据Apache-2.0许可证规定，您有意提交至本仓库的任何贡献均默认按上述双重许可授权，且无需附加任何条款或条件。