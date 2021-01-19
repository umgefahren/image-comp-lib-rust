cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
lipo -create -output target/universal-apple-darwin target/x86_64-apple-darwin/release/image-compressor target/aarch64-apple-darwin/release/image-compressor
