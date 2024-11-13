#!/bin/bash

cargo build --release
cargo xwin build --target x86_64-pc-windows-msvc --release
cargo +nightly xwin build -Z build-std --target x86_64-win7-windows-msvc --release

cp target/release/lotrc_rs ./lotrc_rs.bin
cp target/x86_64-pc-windows-msvc/release/lotrc_rs.exe ./lotrc_rs.exe
cp target/x86_64-win7-windows-msvc/release/lotrc_rs.exe ./lotrc_rs_win7.exe

cp target/release/liblotrc.so ./lotrc_py.so
cp target/x86_64-pc-windows-msvc/release/lotrc.dll ./lotrc_py.pyd
cp target/x86_64-win7-windows-msvc/release/lotrc.dll ./lotrc_py_win7.dll
