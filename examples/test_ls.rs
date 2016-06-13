//! Test Eio module. (requires "eio" feature)
//!
//! List and process all the rust source files '.rs' in the current dir.

extern crate libc;
extern crate efl;

use libc::{c_char};
use std::ffi::CStr;

use efl::types::int;
use efl::ecore;
use efl::eio;

#[allow(unused_variables)]
fn _filter_cb(count: &mut int, handler: &eio::EioFile, file: *const c_char) -> bool {
    unsafe {
        let f: String = CStr::from_ptr(file).to_string_lossy().into_owned();
        // Let's process only rust source files!
        if !f.ends_with(".rs") {
            println!("Filtering file: {}", f);
            return false
        }
    }

    true
}

#[allow(unused_variables)]
fn _main_cb(count: &mut int, handler: &eio::EioFile, file: *const c_char) {
    unsafe {
        let v = CStr::from_ptr(file).to_string_lossy().into_owned();
        // Count processed files
        *count += 1;
        println!("Processing file: {} ({})", v, *count);
    }
}

#[allow(unused_variables)]
fn _done_cb(count: &mut int, handler: &eio::EioFile) {
    println!("Number of processed files: {}", *count);
    println!("Done!");
    ecore::main_loop_quit();
}

#[allow(unused_variables)]
fn _error_cb(count: &mut int, handler: &eio::EioFile, error: int) {
    println!("Error!");
    ecore::main_loop_quit();
}

#[cfg(feature = "eio")]
fn main() {
    ecore::init();
    eio::init();

    let count: int = 0;
    eio::file_ls(".", _filter_cb, _main_cb, _done_cb, _error_cb, &count);

    ecore::main_loop_begin();

    eio::shutdown();
    ecore::shutdown();    
}
