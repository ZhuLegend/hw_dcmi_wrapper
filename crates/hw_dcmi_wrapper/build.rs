fn main() {
    // 检查当前编译平台是否为 Linux
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os != "linux" {
        eprintln!();
        eprintln!("错误: hw_dcmi_wrapper 只支持 Linux 平台");
        eprintln!("Error: hw_dcmi_wrapper only supports Linux platforms");
        eprintln!();
        eprintln!("当前目标平台: {}", target_os);
        eprintln!("Current target platform: {}", target_os);
        eprintln!();
        eprintln!("华为 DCMI API 仅在 Linux 系统上可用。");
        eprintln!("Huawei DCMI API is only available on Linux systems.");
        eprintln!();

        std::process::exit(1);
    }

    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}
