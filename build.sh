rm -rf build
cargo build -r
mkdir -p build
mv target/release/artifacter build/art