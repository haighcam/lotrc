
python python_ffi/gen_cython_bindings.py
cmake -S python_ffi -B target/build
make -C target/build
cp target/build/lotrc.*.so ./lotrc.abi3.so
