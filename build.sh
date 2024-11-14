#!/bin/bash

cargo build --release
cp target/release/lotrc_rs ./lotrc_rs.bin
cp target/release/liblotrc.so ./lotrc.so
rm lotrc_blender.zip
pushd lotrc_blender
zip -r ../lotrc_blender.zip * -i*.*
popd
zip lotrc_blender.zip lotrc.so
