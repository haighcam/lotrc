#cargo build --release -p lotrc
#cp target/release/liblotrc.so ./lotrc.so

#cargo build --release -p lotrc --features ffi
#cargo run -p gen_ffi generate --language python --library target/release/liblotrc.so --out-dir out
#cp target/release/liblotrc.so out/

cargo build --release -p ffi
RUSTC_BOOTSTRAP=1 cbindgen -c ffi/cbindgen.toml -o python_ffi/lotrc.h --lang c ffi 
RUSTC_BOOTSTRAP=1 cbindgen -c ffi/cbindgen.toml -o python_ffi/lotrc_rs.pxd --lang cython ffi 
#python ffi/gen_py_bindings.py

#RUSTC_BOOTSTRAP=1 cbindgen -c ffi/cbindgen.toml -o ffi/bindings.h --lang c lotrc

#cargo build --release -p ffi_alt
#cargo run --bin gen_ffi --features headers
mkdir -p out
cp target/release/liblotrc_ffi.so out/
#cp ffi/lotrc_rs.py out/
#cargo build --release -p lotrc_rs
