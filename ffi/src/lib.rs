use std::ffi::{c_void, CStr, c_char};
use std::ptr::{null, NonNull};

pub mod sub_blocks;

use lotrc::{
    level::{
        pak::PakHeaderPc,
        LevelRef
    },
    types::hash_string,
    macros::make_platforms
};
#[make_platforms] 
use lotrc::{
    level::LevelVER,
    sub_blocks::gameobjs::GameObjsVER
};

#[repr(u8)]
pub enum Version {
    Pc,
    Xbox,
    Ps3
}

#[unsafe(no_mangle)]
pub extern "C" fn hash_string_(s: *const c_char) -> u32 {
    assert!(!s.is_null());
    let s = unsafe { CStr::from_ptr(s) }.to_str().expect("invalid string");
    lotrc::types::hash_string(s.as_bytes(), None)
}

#[unsafe(no_mangle)]
pub extern "C" fn lotrc_get_pak_header() -> PakHeaderPc {
    PakHeaderPc::default()
}

#[unsafe(no_mangle)]
/// the returned version indicates if level contains a ptr to LevelPc, LevelXbox or LevelPs3 
/// the obtained level ptr needs to be freed with the corresponding free function
pub extern "C" fn lotrc_level_parse(path: *const c_char, version: Option<NonNull<Version>>) -> Option<NonNull<c_void>> {
    if path.is_null() || version.is_none() { return None; }
    let version = unsafe { version.unwrap().as_mut() };
    let path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(x) => x,
        Err(e) => {
            println!("path: {}", e.to_string());
            return None;
        }
    };
    match LevelRef::from_data(path) {
        Ok(LevelRef::PC(val)) => {
            *version = Version::Pc;
            NonNull::new(Box::into_raw(Box::new(val)) as *mut c_void)
        },
        Ok(LevelRef::XBOX(val)) => {
            *version = Version::Xbox;
            NonNull::new(Box::into_raw(Box::new(val)) as *mut c_void)
        },
        Ok(LevelRef::PS3(val)) => {
            *version = Version::Ps3;
            NonNull::new(Box::into_raw(Box::new(val)) as *mut c_void)
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            None
        }
    }
}

#[make_platforms]
#[unsafe(no_mangle)]
pub extern "C" fn lotrc_level_free_ver(level: Option<NonNull<LevelVER>>) {
    if let Some(level) = level {
        drop(unsafe { Box::from_raw(level.as_ptr()) })
    }
}

#[make_platforms]
#[unsafe(no_mangle)]
pub extern "C" fn lotrc_level_get_level_block_ver(level: *const LevelVER) -> *const GameObjsVER {
    if level.is_null() { return null(); } 
    let level = unsafe { &*level };
    let level_block = level.pak()
        .block1().unwrap()
        .sub_blocks().get(&hash_string(b"level", None)).unwrap()
        .level().unwrap();
    level_block as *const _
}
