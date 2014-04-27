/*
 * Test Eio module.
 *
 * List and process all the rust source files '.rs' in the current dir.
 *
 */


extern crate libc;
extern crate efl;

use libc::{c_char};
use std::c_str::CString;

use efl::ecore;
use efl::eio;


fn _filter_cb(count: &mut int, handler: &eio::EioFile, file: *c_char) -> bool {
    // Get a &str from the raw *c_char type
    let cstring = unsafe { CString::new(file, false) };
    let f = match cstring.as_str() { None => "", Some(s) => s };
    // Let's process only rust source files!
    if !f.ends_with(".rs") {
        println!("Filtering file: {}", f);
        return false
    }
    return true;
}

fn _main_cb(count: &mut int, handler: &eio::EioFile, file: *c_char) {
    let cstring = unsafe { CString::new(file, false) };
    let v = match cstring.as_str() { None => "", Some(s) => s };
    // Count processed files
    *count += 1;
    println!("Processing file: {} ({})", v, *count);
}

fn _done_cb(count: &mut int, handler: &eio::EioFile) {
    println!("Number of processed files: {}", *count);
    println!("Done!");
    ecore::main_loop_quit();
}

fn _error_cb(count: &mut int, handler: &eio::EioFile, error: int) {
    println!("Error!");
    ecore::main_loop_quit();
}


fn main() {
    ecore::init();
    eio::init();

    let count: int = 0;
    eio::file_ls(".", _filter_cb, _main_cb, _done_cb, _error_cb, &count);

    ecore::main_loop_begin();

    eio::shutdown();
    ecore::shutdown();    
}
