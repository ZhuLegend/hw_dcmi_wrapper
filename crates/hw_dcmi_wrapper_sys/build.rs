use std::env;

#[cfg(feature = "bindgen")]
fn init_bindgen_builder(header: impl Into<String>) -> bindgen::Builder {
    bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(header)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(dead_code)]")
}

fn main() {
    // 检查当前编译平台是否为 Linux
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os != "linux" {
        eprintln!();
        eprintln!("错误: hw_dcmi_wrapper_sys 只支持 Linux 平台");
        eprintln!("Error: hw_dcmi_wrapper_sys only supports Linux platforms");
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

    // 读取环境变量HW_DCMI_PATH作为库搜索路径
    println!("cargo:rerun-if-env-changed=HW_DCMI_PATH");
    let hw_dcmi_path = env::var("HW_DCMI_PATH").unwrap_or_else(|_| "/usr/local/dcmi".to_string());
    println!("cargo:rustc-link-search=native={}", hw_dcmi_path);

    #[cfg(not(feature = "load_dynamic"))]
    println!("cargo:rustc-link-lib=dylib=dcmi");

    #[cfg(feature = "bindgen")]
    {
        // 当且仅当HW_DCMI_BINDING_BUILD为true时才生成绑定
        println!("cargo:rerun-if-env-changed=HW_DCMI_BINDING_BUILD");
        if env::var("HW_DCMI_BINDING_BUILD").is_err() {
            return;
        }
        if env::var("HW_DCMI_BINDING_BUILD").unwrap() != "true" {
            return;
        }

        let interface_path = format!("{}/dcmi_interface_api.h", hw_dcmi_path);

        // The bindgen::Builder is the main entry point to bindgen,
        // and lets you build up options for the resulting bindings.

        let builder = init_bindgen_builder(interface_path);

        #[cfg(feature = "load_dynamic")]
        let builder = builder
            .dynamic_library_name("dcmi")
            .dynamic_link_require_all(true);

        let bindings = builder
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // 指定输出文件的路径为 src/hw_dcmi_wrapper_sys.rs
        #[cfg(feature = "load_dynamic")]
        let out_path =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/bindings_dyn.rs");
        #[cfg(not(feature = "load_dynamic"))]
        let out_path =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/bindings.rs");

        // Write the bindings to the specified file.
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
