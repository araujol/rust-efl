// Edje Rust bindings for EFL.
// Copyright (C) 2014  Luis Araujo <araujoc.luisf@gmail.com>

// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

extern crate libc;

use edje::libc::{c_int, c_char};
use std::{mem, ffi};

use evas;
use eina;
use eseful::from_eina_to_bool;


#[link(name = "edje")]
extern "C" {
    fn edje_init() -> c_int;
    fn edje_shutdown() -> c_int;
    fn edje_object_add(evas: *const evas::Evas) -> *const evas::EvasObject;
    fn edje_object_file_set(obj: *const evas::EvasObject,
                            file: *const c_char,
                            group: *const c_char) -> eina::EinaBool;
    fn edje_object_part_text_set(obj: *const evas::EvasObject, part: *const c_char,
                                 text: *const c_char) -> eina::EinaBool;
}

/// Initialize the Edje library.
pub fn init() -> isize {
    unsafe { edje_init() as isize }
}

/// Shutdown the Edje library.
pub fn shutdown() -> isize {
    unsafe { edje_shutdown() as isize }
}

/// Instantiate a new Edje object.
pub fn object_add(evas: &evas::Evas) -> Box<evas::EvasObject> {
    unsafe { mem::transmute(edje_object_add(evas)) }
}

/// Sets the EDJ file (and group within it) to load an Edje object's contents from.
pub fn object_file_set(obj: &evas::EvasObject, file: &str, group: &str) -> bool {
    let c_file = ffi::CString::from_slice(file.as_bytes());
    let c_group = ffi::CString::from_slice(group.as_bytes());
    unsafe {
        from_eina_to_bool(edje_object_file_set(obj, c_file.as_ptr(), c_group.as_ptr()))
    }
}

/// Sets the text for an object part.
pub fn object_part_text_set(obj: &evas::EvasObject, part: &str, text: &str) -> bool {
    let c_part = ffi::CString::from_slice(part.as_bytes());
    let c_text = ffi::CString::from_slice(text.as_bytes());
    unsafe {
        from_eina_to_bool(edje_object_part_text_set(obj, c_part.as_ptr(), c_text.as_ptr()))
    }
}
