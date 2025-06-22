extern crate cbindgen;
extern crate csbindgen;

use cbindgen::Config;
use std::env;

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
        .input_extern_file("src/price.rs")
        .input_extern_file("src/curve.rs")
        .input_extern_file("src/swap.rs")
        .input_extern_file("src/options.rs")
        .input_extern_file("src/strategy.rs")
        .input_extern_file("../finlib/src/portfolio/portfolio.rs")
        .input_extern_file("../finlib/src/portfolio/asset.rs")
        .input_extern_file("../finlib/src/portfolio/strategy/strategy.rs")
        .input_extern_file("../finlib/src/price/price.rs")
        .input_extern_file("../finlib/src/price/enums.rs")
        .input_extern_file("../finlib/src/curve/curve.rs")
        .input_extern_file("../finlib/src/curve/point.rs")
        .input_extern_file("../finlib/src/derivatives/swaps/mod.rs")
        .input_extern_file("../finlib/src/derivatives/options/mod.rs")
        .input_extern_file("../finlib/src/derivatives/options/option_contract.rs")
        .input_extern_file("../finlib/src/derivatives/options/blackscholes/mod.rs")
        .input_extern_file("../finlib/src/derivatives/options/blackscholes/option_surface.rs")
        .csharp_dll_name("libfinlib_ffi")
        .csharp_namespace("FinLib")
        .csharp_type_rename(|rust_type_name| match rust_type_name.as_str() {
            // optional, default: `|x| x`
            "NullableFloat" => "NullableFloat".into(),
            "Tuple" => "Tuple".into(),
            _ => rust_type_name + "_native",
        })
        .generate_csharp_file("../FinLib.NET/FinLib/NativeMethods.g.cs")
        .unwrap();
}
