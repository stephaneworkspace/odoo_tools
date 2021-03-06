#!/bin/sh
cargo update
cargo build
cargo test
# Group binary with lipo
cargo +ios-arm64-1.46.0 build --target aarch64-apple-ios --release --lib
#cargo build --target x86_64-apple-darwin --release --lib
cargo build --target x86_64-apple-ios --release
# Print NonFat -> Ok
lipo -info ./target/aarch64-apple-ios/release/libodoo_tools.a
l#ipo -info ./target/x86_64-apple-darwin/release/libodoo_tools.a
lipo -info ./target/x86_64-apple-ios/release/libodoo_tools.a
# Group in one
#lipo -create ./target/aarch64-apple-ios/release/libodoo_tools.a ./target/x86_64-apple-darwin/release/libodoo_tools.a ./target/x86_64-apple-ios/release/libodoo_tools.a -output ./target/libodoo_tools.a
lipo -create ./target/aarch64-apple-ios/release/libodoo_tools.a ./target/x86_64-apple-ios/release/libodoo_tools.a -output ./target/libodoo_tools.a
# Print architecture
lipo -info ./target/libodoo_tools.a
cbindgen . --lang C -o target/libodoo_tools.h
cp ./target/libodoo_tools.a ./swift/libodoo_tools.a
cp ./target/libodoo_tools.h ./swift/libodoo_tools.h
cd ./swift
./arch.sh
#./build_framework.sh