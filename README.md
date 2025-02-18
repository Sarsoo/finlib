# finlib

[![Build Binaries](https://github.com/Sarsoo/finlib/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Sarsoo/finlib/actions/workflows/build.yml)

Some quantitative finance functionality written in Rust and consumable from many higher-level languages.

## Derivatives Pricing
- Options
  - Black-Scholes
    - Prices
    - Greeks

## Risk
- Value-at-Risk
  - Historical
  - Variance-Covariance (Parametric)
    - Single Asset
    - Portfolio

# FFI

- C++
  - FFI header files for C++ are generated automatically during build by [cbindgen](https://github.com/mozilla/cbindgen). 
- .NET
  - FFI wrapper code for C# tareting .NET Standard 2.0 is generated automatically using [csbindgen](https://github.com/Cysharp/csbindgen/).
- Python
  - An adapter library for Python is generated usign [PyO3](https://github.com/PyO3/pyo3)
- WASM (Js)
  - A Javascript library is generated using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

## [.NET](./finlib-ffi)

```bash
cargo build
cd FinLib.NET
dotnet build
```

## [Python](./pyfinlib)

```bash
cd pyfinlib
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
maturin develop
```

## [WASM](finlib-wasm)

```bash
cd finlib-wasm
wasm-pack build
```