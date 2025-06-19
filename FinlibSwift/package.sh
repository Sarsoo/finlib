swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir ./package \
  --ios ../target/aarch64-apple-ios/debug/libFinlibSwift.a \
  --simulator ../target/universal-ios/debug/libFinlibSwift.a \
  --macos ../target/universal-macos/debug/libFinlibSwift.a \
  --name FinlibSwift