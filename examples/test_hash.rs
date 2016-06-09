/*
 * EinaHash test example.
 */

extern crate efl;
extern crate libc;

use efl::eina;
use libc::{free, c_char};
use std::mem::transmute;
use std::c_str::CString;


fn _free_hash(data: &c_char) {
    unsafe { free(transmute(data)) }
}

fn add_to_hash(hash: *mut eina::_EinaHash<c_char>, key: &str, data: &str) -> bool {
    let c_key = CString::new(key).unwrap();
    unsafe {
        // Unwrap the cstring to forget ownership of the value so it can be
        // saved in the hash.
        // This *c_char will be freed by _free_hash
        let cstr = data.to_c_str();
        let cchar = cstr.unwrap();
        eina::hash_add(hash, transmute(c_key.as_ptr()), transmute(cchar))
    }
}

fn find_value(hash: *mut eina::_EinaHash<c_char>, key: &str) -> String {
    let c_key = CString::new(key).unwrap();
    unsafe {
        let value: &c_char = eina::hash_find(hash, transmute(c_key.as_ptr()));
        let cstr = CString::new(value, false);
        if cstr.is_not_null() {
            match cstr.as_str() {
                None => panic!("Not valid string"),
                Some(s) => s.to_string()
            }
        } else {
            panic!("Null string!")
        }
    }
}


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
