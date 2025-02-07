# hw_dcmi_wrapper_sys

[![Crates.io version](https://img.shields.io/crates/v/hw_dcmi_wrapper_sys.svg)](https://crates.io/crates/hw_dcmi_wrapper_sys)
[![Crates.io downloads](https://img.shields.io/crates/d/hw_dcmi_wrapper_sys)](https://crates.io/crates/hw_dcmi_wrapper_sys)
[![Docs.rs docs](https://docs.rs/hw_dcmi_wrapper_sys/badge.svg)](https://docs.rs/hw_dcmi_wrapper_sys)

[中文文档](./README_CN.md)

本项目为华为[昇腾卡管理接口][dcmi]（DCMI，基于C语言的编程接口）提供的Rust绑定库，该接口用于监控和管理华为NPU的各类状态。

此库旨在为第三方应用开发提供基础平台，同时也是华为`npu-smi`工具的底层支持库。

如需使用更安全的封装层，请参阅[`hw_dcmi_wrapper`][hw_dcmi_wrapper]项目。

## 关于绑定生成

本绑定库通过[bindgen]生成。若启用`load_dynamic`特性（基于[`libloading`][libloading]），将生成动态链接绑定；否则默认生成静态链接绑定。

默认情况下，库会在`/usr/local/dcmi`目录下搜索DCMI组件。您可通过设置环境变量`HW_DCMI_PATH`来覆盖该路径。

如需重新生成绑定，请设置环境变量`HW_DCMI_BINDING_BUILD`为`true`，生成的绑定文件将保存至：

- 静态链接：`hw_dcmi_wrapper_sys/src/bindings.rs`
- 动态链接：`hw_dcmi_wrapper_sys/src/bindings_dyn.rs`

#### 许可证

本项目遵循双重许可，您可选择遵循 [Apache 2.0 许可证](../../LICENSE-APACHE) 或 [MIT 许可证](../../LICENSE-MIT)。

---

除非您明确声明，否则依据Apache-2.0许可证规定，您有意提交至本仓库的任何贡献均默认按上述双重许可授权，且无需附加任何条款或条件。

[dcmi]: https://support.huawei.com/enterprise/zh/doc/EDOC1100349020/426cffd9

[hw_dcmi_wrapper]: https://github.com/ZhuLegend/hw_dcmi_wrapper

[bindgen]: https://github.com/rust-lang/rust-bindgen

[libloading]: https://github.com/nagisa/rust_libloading