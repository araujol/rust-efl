// Emotion Rust bindings for EFL.
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

use std::mem::transmute;
use std::ffi::{CString, CStr};
use emotion::libc::c_char;
use evas;
use eina;
use eseful;

#[link(name = "emotion")]
extern "C" {
    fn emotion_object_add(evas: *const evas::Evas) -> *const evas::EvasObject;
    fn emotion_object_init(obj: *const evas::EvasObject, module_filename: *const c_char) -> u8;
    fn emotion_object_file_get(obj: *const evas::EvasObject) -> *const c_char;
    fn emotion_object_file_set(obj: *const evas::EvasObject, filename: *const c_char) -> u8;
    fn emotion_object_play_get(obj: *const evas::EvasObject) -> eina::EinaBool;
    fn emotion_object_play_set(obj: *const evas::EvasObject, play: eina::EinaBool);
}

/// Add an emotion object to the canvas.
pub fn object_add(evas: &evas::Evas) -> Box<evas::EvasObject> {
    unsafe { transmute(emotion_object_add(evas)) }
}

/// Initializes an emotion object with the specified module.
pub fn object_init(obj: &evas::EvasObject, module_filename: &str) -> eina::EinaBool {
    let c_mf = CString::new(module_filename).unwrap();
    unsafe {
        emotion_object_init(obj, c_mf.as_ptr()) as eina::EinaBool
    }
}

/// Get the filename of the file associated with the emotion object.
pub fn object_file_get(obj: &evas::EvasObject) -> String {
    unsafe {
        // https://doc.rust-lang.org/std/ffi/struct.CStr.html
        CStr::from_ptr(emotion_object_file_get(obj)).to_string_lossy().into_owned()
        // TODO does this old code check for null? Does the code above?
        /*
        (match CString::new(emotion_object_file_get(obj), false).as_str() {
            None => "", Some(s) => s
        }).to_string()
        */
    }
}

/// Set the file to be played in the Emotion object.
pub fn object_file_set(obj: &evas::EvasObject, filename: &str) -> eina::EinaBool {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        emotion_object_file_set(obj, c_filename.as_ptr()) as eina::EinaBool
    }
}

/// Get play/pause state of the media file.
pub fn object_play_get(obj: &evas::EvasObject) -> bool {
    unsafe { eseful::from_eina_to_bool(emotion_object_play_get(obj)) }
}

/// Set play/pause state of the media file.
pub fn object_play_set(obj: &evas::EvasObject, play: bool) {
    unsafe { 
        emotion_object_play_set(obj, eseful::from_bool_to_eina(play))
    }
}
