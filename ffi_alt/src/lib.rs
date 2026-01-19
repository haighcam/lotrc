use safer_ffi::ffi_export;

pub mod level;
pub mod sub_blocks;
pub mod types;

#[cfg(feature = "headers")]
pub fn gen_ffi() -> std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("filename.h")?
        .generate()
}
