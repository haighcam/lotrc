#cargo build --release -p lotrc
#cp target/release/liblotrc.so ./lotrc.so

#cargo build --release -p lotrc --features ffi
#cargo run -p gen_ffi generate --language python --library target/release/liblotrc.so --out-dir out
#cp target/release/liblotrc.so out/

cargo build --release -p ffi
RUSTC_BOOTSTRAP=1 cbindgen -c ffi/cbindgen.toml -o ffi/bindings.h ffi 
