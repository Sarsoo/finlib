#!/bin/bash

set -e

THISDIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd $THISDIR
echo "> building rust"
cargo build

cd $THISDIR/FinLib.NET
echo "> building .net"
dotnet build

cd $THISDIR/finlib-cpp
echo "> building c++"
mkdir -p build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
cmake --build .

cd $THISDIR/finlib-wasm
echo "> building wasm"
wasm-pack build

cd $THISDIR/FinlibSwift
echo "> building swift"
./build.sh
./package.sh

cd $THISDIR/pyfinlib
echo "> building python"
source .venv/bin/activate; maturin develop