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

use std::{mem, ffi, str};
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
    unsafe { mem::transmute(emotion_object_add(evas)) }
}

/// Initializes an emotion object with the specified module.
pub fn object_init(obj: &evas::EvasObject, module_filename: &str) -> eina::EinaBool {
    let mod_file = ffi::CString::from_slice(module_filename.as_bytes());
    unsafe {
        emotion_object_init(obj, mod_file.as_ptr()) as eina::EinaBool
    }
}

/// Get the filename of the file associated with the emotion object.
pub fn object_file_get(obj: &evas::EvasObject) -> String {
    unsafe {
        match str::from_utf8(ffi::c_str_to_bytes(&emotion_object_file_get(obj))) {
            Ok(s) => s,
            Err(_) => panic!("filename is not utf-8")
        }.to_string()
    }
}

/// Set the file to be played in the Emotion object.
pub fn object_file_set(obj: &evas::EvasObject, filename: &str) -> eina::EinaBool {
    let file = ffi::CString::from_slice(filename.as_bytes());
    unsafe {
        emotion_object_file_set(obj, file.as_ptr()) as eina::EinaBool
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
