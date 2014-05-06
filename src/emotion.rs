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

use std::str::raw::from_c_str;
use emotion::libc::c_char;
use evas;
use eina;
use eseful;

#[link(name = "emotion")]
extern "C" {
    fn emotion_object_add(evas: *evas::Evas) -> *evas::EvasObject;
    fn emotion_object_init(obj: *evas::EvasObject, module_filename: *c_char) -> u8;
    fn emotion_object_file_get(obj: *evas::EvasObject) -> *c_char;
    fn emotion_object_file_set(obj: *evas::EvasObject, filename: *c_char) -> u8;
    fn emotion_object_play_get(obj: *evas::EvasObject) -> eina::EinaBool;
    fn emotion_object_play_set(obj: *evas::EvasObject, play: eina::EinaBool);
}

/// Add an emotion object to the canvas.
pub fn object_add(evas: &evas::Evas) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(emotion_object_add(evas)) }
}

/// Initializes an emotion object with the specified module.
pub fn object_init(obj: &evas::EvasObject, module_filename: &str) -> eina::EinaBool {
    module_filename.with_c_str(|c_mf| unsafe {
        emotion_object_init(obj, c_mf) as eina::EinaBool
    })
}

/// Get the filename of the file associated with the emotion object.
pub fn object_file_get(obj: &evas::EvasObject) -> ~str {
    unsafe {
        from_c_str(emotion_object_file_get(obj))
    }
}

/// Set the file to be played in the Emotion object.
pub fn object_file_set(obj: &evas::EvasObject, filename: &str) -> eina::EinaBool {
    filename.with_c_str(|c_filename| unsafe {
        emotion_object_file_set(obj, c_filename) as eina::EinaBool
    })
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
