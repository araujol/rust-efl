//! EinaHash test example. (requires "eina" feature)
/*
 This code is old and broken at the moment.
 Why do Rust users need eina hash functionality?
 Not maintaining unless user need determined.
 */
extern crate efl;
extern crate libc;

use efl::eina;
use libc::{free, c_char};
use std::mem::transmute;
use std::ffi::{CString, CStr};

fn _free_hash(data: &c_char) {
    unsafe { free(transmute(data)) }
}

fn add_to_hash(hash: *mut eina::_EinaHash<c_char>, key: &str, data: &str) -> bool {
    let c_key = CString::new(key).unwrap();
    unsafe {
        // Unwrap the cstring to forget ownership of the value so it can be
        // saved in the hash.
        // This *c_char will be freed by _free_hash
        let cstr = CString::new(data).unwrap();
        eina::hash_add(hash, transmute(c_key.as_ptr()), transmute(cstr))
    }
}

fn find_value(hash: *mut eina::_EinaHash<c_char>, key: &str) -> String {
    let c_key = CString::new(key).unwrap();
    unsafe {
        let value: &c_char = eina::hash_find(hash, transmute(c_key.as_ptr()));
        CStr::from_ptr(value).to_string_lossy().into_owned()
    }
}

/*
 Don't run this example for now
 This code is messy and dangerous. Is there any need for EinaHash anyway?
*/
#[cfg(feature = "eina")]
fn main() {
    eina::init();

    let hash: *mut eina::_EinaHash<c_char> = eina::hash_string_superfast_new(_free_hash);

    let mut key = "E";
    let mut data = "Enlightenment-v1.10";
    println!("Added key {} ({})", key, add_to_hash(hash, key, data));

    key = "R";
    data = "Rust-master-0.11-pre";
    println!("Added key {} ({})", key, add_to_hash(hash, key, data));

    println!("Number of entries: {}", eina::hash_population(hash));
    println!("Found value for key '{}': '{}'", key, find_value(hash, key));

    eina::hash_free(hash);

    eina::shutdown();
}
