use std::ffi::{CStr, c_char};
use lotrc::macros::export;

pub mod level;
pub mod types;

unsafe fn c_str_ptr(s: Option<&c_char>) -> &str {
    s.and_then(|s| 
        unsafe { CStr::from_ptr(s as _) }.to_str().ok()
    ).unwrap_or_default()
}

#[export]
mod utils {
    use super::*;
    fn hash_string(s: Option<&c_char>) -> u32 {
        lotrc::types::hash_string(
            unsafe { c_str_ptr(s) }.as_bytes(),
            None
        )
}
}
