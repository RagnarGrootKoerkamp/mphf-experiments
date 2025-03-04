extern crate libc;
extern crate ph;
extern crate rayon;

use libc::strlen;
use mem_dbg::SizeFlags;
use ph::fmph::{BuildConf, GOBuildConf};
use ph::{fmph, GetSize};
use ptr_hash::PtrHashParams;
use std::os::raw::c_char;
use std::slice;
use std::str;

////////////////////////////////////////// General methods //////////////////////////////////////////
fn c_strings_to_vec(len: usize, my_strings: *const *const c_char) -> Vec<String> {
    let mut vector = Vec::new();
    let sl = unsafe { std::slice::from_raw_parts(my_strings, len) };
    let mut index = 0;
    while index < sl.len() {
        let c_s = sl[index];
        let s = unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(c_s as *const u8, strlen(c_s) + 1))
        };
        vector.push(String::from(s));
        index += 1;
    }
    return vector;
}

fn c_strings_to_slices(len: usize, my_strings: *const *const c_char) -> Vec<&'static [u8]> {
    let mut vector = Vec::new();
    let sl = unsafe { std::slice::from_raw_parts(my_strings, len) };
    let mut index = 0;
    while index < sl.len() {
        let c_s = sl[index];
        let s = unsafe { slice::from_raw_parts(c_s as *const u8, strlen(c_s) + 1) };
        vector.push(s);
        index += 1;
    }
    return vector;
}

#[no_mangle]
pub extern "C" fn initializeRayonThreadPool(threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();
}

////////////////////////////////////////// Fmph //////////////////////////////////////////
pub struct FmphWrapper {
    vector: Vec<String>,
    hash_func: fmph::Function,
}

#[no_mangle]
pub extern "C" fn createFmphStruct(
    len: usize,
    my_strings: *const *const c_char,
) -> *mut FmphWrapper {
    let struct_instance = FmphWrapper {
        vector: c_strings_to_vec(len, my_strings),
        hash_func: fmph::Function::from(&[] as &[String]),
    };
    let boxx = Box::new(struct_instance);
    Box::into_raw(boxx)
}

#[no_mangle]
pub extern "C" fn constructFmph(struct_ptr: *mut FmphWrapper, gamma: u16) {
    let struct_instance = unsafe { &mut *struct_ptr };
    let mut build_config = BuildConf::default();
    build_config.use_multiple_threads = true;
    build_config.relative_level_size = gamma;
    struct_instance.hash_func =
        fmph::Function::from_slice_with_conf(&struct_instance.vector[..], build_config);
}

#[no_mangle]
pub extern "C" fn queryFmph(
    struct_ptr: *mut FmphWrapper,
    key_c_s: *const c_char,
    length: usize,
) -> u64 {
    let struct_instance = unsafe { &mut *struct_ptr };
    let key = unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(key_c_s as *const u8, length + 1))
    };
    struct_instance.hash_func.get(key).unwrap()
}

#[no_mangle]
pub extern "C" fn sizeFmph(struct_ptr: *mut FmphWrapper) -> usize {
    let struct_instance = unsafe { &mut *struct_ptr };
    struct_instance.hash_func.size_bytes()
}

#[no_mangle]
pub extern "C" fn destroyFmphStruct(struct_instance: *mut FmphWrapper) {
    unsafe {
        let _ = Box::from_raw(struct_instance);
    }
}

////////////////////////////////////////// FmphGO //////////////////////////////////////////
pub struct FmphGoWrapper {
    vector: Vec<String>,
    hash_func: fmph::GOFunction,
}

#[no_mangle]
pub extern "C" fn createFmphGoStruct(
    len: usize,
    my_strings: *const *const c_char,
) -> *mut FmphGoWrapper {
    let struct_instance = FmphGoWrapper {
        vector: c_strings_to_vec(len, my_strings),
        hash_func: fmph::GOFunction::from(&[] as &[String]),
    };
    let boxx = Box::new(struct_instance);
    Box::into_raw(boxx)
}

#[no_mangle]
pub extern "C" fn constructFmphGo(struct_ptr: *mut FmphGoWrapper, gamma: u16) {
    let struct_instance = unsafe { &mut *struct_ptr };
    let mut build_config = GOBuildConf::default();
    build_config.use_multiple_threads = true;
    build_config.relative_level_size = gamma;
    struct_instance.hash_func =
        fmph::GOFunction::from_slice_with_conf(&struct_instance.vector[..], build_config);
}

#[no_mangle]
pub extern "C" fn queryFmphGo(
    struct_ptr: *mut FmphGoWrapper,
    key_c_s: *const c_char,
    length: usize,
) -> u64 {
    let struct_instance = unsafe { &mut *struct_ptr };
    let key = unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(key_c_s as *const u8, length + 1))
    };
    struct_instance.hash_func.get(key).unwrap()
}

#[no_mangle]
pub extern "C" fn sizeFmphGo(struct_ptr: *mut FmphGoWrapper) -> usize {
    let struct_instance = unsafe { &mut *struct_ptr };
    struct_instance.hash_func.size_bytes()
}

#[no_mangle]
pub extern "C" fn destroyFmphGoStruct(struct_instance: *mut FmphGoWrapper) {
    unsafe {
        let _ = Box::from_raw(struct_instance);
    }
}

////////////////////////////////////////// PtrHash //////////////////////////////////////////
type Key = &'static [u8];
type PtrHashFast =
    ptr_hash::PtrHash<Key, ptr_hash::bucket_fn::Linear, Vec<u32>, ptr_hash::hash::Xx64, Vec<u8>>;
type PtrHashCompact = ptr_hash::PtrHash<
    Key,
    ptr_hash::bucket_fn::CubicEps,
    ptr_hash::CachelineEfVec,
    ptr_hash::hash::Xx64,
    Vec<u8>,
>;
type PtrHashDefault = PtrHashCompact;

pub enum PtrHash {
    None,
    Fast(PtrHashFast),
    Compact(PtrHashCompact),
    Default(PtrHashDefault),
}

pub struct PtrHashWrapper {
    vector: Vec<Key>,
    ptr_hash: PtrHash,
}

#[no_mangle]
pub extern "C" fn createPtrHashStruct(
    len: usize,
    my_strings: *const *const c_char,
) -> *mut PtrHashWrapper {
    let struct_instance = PtrHashWrapper {
        vector: c_strings_to_slices(len, my_strings),
        ptr_hash: PtrHash::None,
    };
    let boxx = Box::new(struct_instance);
    Box::into_raw(boxx)
}

#[no_mangle]
pub extern "C" fn constructPtrHash(struct_ptr: *mut PtrHashWrapper, version: usize) {
    let struct_instance = unsafe { &mut *struct_ptr };
    match version {
        0 => {
            struct_instance.ptr_hash = PtrHash::Fast(PtrHashFast::new(
                &struct_instance.vector[..],
                PtrHashParams::default_fast(),
            ));
        }
        1 => {
            struct_instance.ptr_hash = PtrHash::Compact(PtrHashCompact::new(
                &struct_instance.vector[..],
                PtrHashParams::default_compact(),
            ));
        }
        2 => {
            struct_instance.ptr_hash = PtrHash::Default(PtrHashDefault::new(
                &struct_instance.vector[..],
                PtrHashParams::default(),
            ));
        }
        _ => panic!("Invalid version"),
    }
}

#[no_mangle]
pub extern "C" fn queryPtrHash(
    struct_ptr: *mut PtrHashWrapper,
    key_c_s: *const c_char,
    length: usize,
) -> u64 {
    let struct_instance = unsafe { &mut *struct_ptr };
    let key: Key = unsafe { slice::from_raw_parts(key_c_s as *const u8, length + 1) };
    match struct_instance.ptr_hash {
        PtrHash::None => panic!("PtrHash not initialized"),
        PtrHash::Fast(ref mut ptr_hash) => ptr_hash.index_minimal(&key) as u64,
        PtrHash::Compact(ref mut ptr_hash) => ptr_hash.index_minimal(&key) as u64,
        PtrHash::Default(ref mut ptr_hash) => ptr_hash.index_minimal(&key) as u64,
    }
}

#[no_mangle]
pub extern "C" fn queryPtrHashStream(
    struct_ptr: *mut PtrHashWrapper,
    keys: *const *const c_char,
    lengths: *const usize,
    num_keys: usize,
) -> u64 {
    let struct_instance = unsafe { &mut *struct_ptr };
    let keys = (0..num_keys).map(|i| unsafe {
        slice::from_raw_parts(
            *keys.offset(i as isize) as *const u8,
            *lengths.offset(i as isize) + 1,
        )
    });
    match struct_instance.ptr_hash {
        PtrHash::None => panic!("PtrHash not initialized"),
        PtrHash::Fast(ref mut ptr_hash) => {
            ptr_hash.index_stream::<32, true, _>(keys).sum::<usize>() as u64
        }
        PtrHash::Compact(ref mut ptr_hash) => {
            ptr_hash.index_stream::<32, true, _>(keys).sum::<usize>() as u64
        }
        PtrHash::Default(ref mut ptr_hash) => {
            ptr_hash.index_stream::<32, true, _>(keys).sum::<usize>() as u64
        }
    }
}

#[no_mangle]
pub extern "C" fn sizePtrHash(struct_ptr: *mut PtrHashWrapper) -> usize {
    let struct_instance = unsafe { &mut *struct_ptr };
    use mem_dbg::MemSize;
    match struct_instance.ptr_hash {
        PtrHash::None => panic!("PtrHash not initialized"),
        PtrHash::Fast(ref ptr_hash) => ptr_hash.mem_size(SizeFlags::default()),
        PtrHash::Compact(ref ptr_hash) => ptr_hash.mem_size(SizeFlags::default()),
        PtrHash::Default(ref ptr_hash) => ptr_hash.mem_size(SizeFlags::default()),
    }
}

#[no_mangle]
pub extern "C" fn destroyPtrHashStruct(struct_instance: *mut PtrHashWrapper) {
    unsafe {
        let _ = Box::from_raw(struct_instance);
    }
}
