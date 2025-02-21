extern crate cbindgen;
extern crate csbindgen;

use std::env;
use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = Config::from_file("./cbindgen.toml").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../finlib-cpp/include/finlib/finlib-native.h");

    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/portfolio.rs")
        .input_extern_file("src/indicators.rs")
        .input_extern_file("../finlib/src/risk/portfolio.rs")
        .csharp_dll_name("libfinlib_ffi")
        .csharp_namespace("FinLib")
        .csharp_type_rename(|rust_type_name| match rust_type_name.as_str() {    // optional, default: `|x| x`
            "Portfolio" => "Portfolio_native".into(),
            "PortfolioAsset" => "PortfolioAsset_native".into(),
            _ => rust_type_name,
        })
        .generate_csharp_file("../FinLib.NET/FinLib/NativeMethods.g.cs")
        .unwrap();
}