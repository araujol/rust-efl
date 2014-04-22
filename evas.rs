// Evas Rust bindings for EFL.
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

use evas::libc::{c_int, c_char, c_void};
use std::cast::transmute;
use std::option::{Option};
use std::ptr;

use eina;
use eseful;

pub static EVAS_HINT_EXPAND: f64 = 1.0f64;
pub static EVAS_HINT_FILL: f64 = -1.0f64;

pub enum Eo {}
pub enum Evas {}

pub type EvasObject = Eo;

pub type Coord = (int, int);

/* High level callback notation */
pub type EvasSmartCb<T> = fn (&Option<T>, &EvasObject, &eseful::EventInfo) -> ();
/* C level callback notation */
type CEvasSmartCb = fn (*c_void, *EvasObject, *c_void) -> c_void;


#[link(name = "evas")]
extern "C"  {
    fn evas_init() -> c_int;
    fn evas_shutdown() -> c_int;
    fn evas_new() -> *Evas;
    fn evas_free(e: *Evas);
    fn evas_focus_in(e: *Evas);
    fn evas_focus_out(e: *Evas);
    fn evas_object_rectangle_add(e: *Evas) -> *EvasObject;
    fn evas_object_show(e: *EvasObject);
    fn evas_object_resize(e: *EvasObject, w: c_int, h: c_int);
    fn evas_object_move(e: *EvasObject, x: c_int, y: c_int);
    fn evas_object_name_set(obj: *EvasObject, name: *c_char); 
    fn evas_object_color_set(obj: *EvasObject,
                             r: c_int, g: c_int,
                             b: c_int, a: c_int);
    fn evas_object_size_hint_min_set(e: *EvasObject, x: c_int, y: c_int);
    fn evas_object_size_hint_weight_set(e: *EvasObject, x: f64, y: f64);
    fn evas_object_size_hint_align_set(e: *EvasObject, x: f64, y: f64);
    fn evas_object_focus_set(obj: *EvasObject, focus: eina::EinaBool);
    fn evas_object_image_add(e: *Evas) -> *EvasObject;
    fn evas_object_image_filled_add(e: *Evas) -> *EvasObject;
    fn evas_object_image_fill_set(obj: *EvasObject,
                                  x: c_int, y: c_int,
                                  w: c_int, h: c_int);
    fn evas_object_image_file_set(obj: *EvasObject, file: *c_char, key: *c_char);
    fn evas_object_image_size_set(obj: *EvasObject, w: c_int, h: c_int);
    fn evas_object_image_filled_set(obj: *EvasObject, setting: eina::EinaBool);
    fn evas_object_image_preload(obj: *EvasObject, cancel: eina::EinaBool);
    fn evas_object_smart_callback_add(e: *EvasObject, event: *c_char,
                                      cb: CEvasSmartCb, data: *c_void);
}

pub fn init() -> int {
    unsafe { evas_init() as int }
}

pub fn shutdown() -> int {
    unsafe { evas_shutdown() as int }
}

pub fn new() -> ~Evas {
    unsafe { transmute::<*Evas, ~Evas>(evas_new()) }
}

pub fn free(e: &Evas) {
    unsafe { evas_free(e) }
}

pub fn focus_in(e: &Evas) {
    unsafe { evas_focus_in(e) }
}

pub fn focus_out(e: &Evas) {
    unsafe { evas_focus_out(e) }
}

pub fn object_move(e: &EvasObject, c: Coord) {
    let (x, y) = c;
    unsafe { evas_object_move(e, x as c_int, y as c_int) }
}

pub fn object_name_set(obj: &EvasObject, name: &str) {
    name.with_c_str(|c_name| unsafe {
        evas_object_name_set(obj, c_name)
    })
}

pub fn object_color_set(obj: &EvasObject, r: int, g: int, b: int, a: int) {
    unsafe { 
        evas_object_color_set(obj,
                              r as c_int,
                              g as c_int,
                              b as c_int,
                              a as c_int)
    }
}

pub fn object_resize(e: &EvasObject, w: int, h: int) {
    unsafe { evas_object_resize(e, w as c_int, h as c_int) }
}

pub fn object_size_hint_min_set(e: &EvasObject, w: int, h: int) {
    unsafe { evas_object_size_hint_min_set(e, w as c_int, h as c_int) }
}

pub fn object_size_hint_weight_set(e: &EvasObject, x: f64, y: f64) {
    unsafe { evas_object_size_hint_weight_set(e, x, y) }
}

pub fn object_size_hint_align_set(e: &EvasObject, x: f64, y: f64) {
    unsafe { evas_object_size_hint_align_set(e, x, y) }
}

pub fn object_focus_set(obj: &EvasObject, focus: eina::EinaBool) {
    unsafe { evas_object_focus_set(obj, focus) }
}

pub fn object_show(e: &EvasObject) {
    unsafe { evas_object_show(e) } 
}

/// Creates a new image object on the given Evas e canvas.
pub fn object_image_add(e: &Evas) -> ~EvasObject {
    unsafe { cast_to_evas_obj(evas_object_image_add(e)) }
}

/// Creates a new image object that automatically scales its bound image to
/// the object's area, on both axis.
pub fn object_image_filled_add(e: &Evas) -> ~EvasObject {
    unsafe { cast_to_evas_obj(evas_object_image_filled_add(e)) }
}

/// Set how to fill an image object's drawing rectangle given the (real)
/// image bound to it.
pub fn object_image_fill_set(obj: *EvasObject, xy: Coord, wh: Coord) {
    let (x, y) = xy;
    let (w, h) = wh;
    unsafe {
        evas_object_image_fill_set(obj,
                                   x as c_int, y as c_int,
                                   w as c_int, h as c_int)
    }
}

/// Set the source file from where an image object must fetch the real
/// image data (it may be an Eet file, besides pure image ones).
pub fn object_image_file_set(obj: &EvasObject, file: &str, key: Option<&str>) {
    file.with_c_str(|c_file| unsafe {
        match key {
            None => evas_object_image_file_set(obj, c_file, ptr::null()),
            Some(ref k) =>
                k.with_c_str(|c_key|
                             evas_object_image_file_set(obj, c_file, c_key))
        }
    })
}

/// Sets the size of the given image object.
pub fn object_image_size_set(obj: &EvasObject, w: int, h: int) {
    unsafe { evas_object_image_size_set(obj, w as c_int, h as c_int) }
}

/// Set whether the image object's fill property should track the object's size.
pub fn object_image_filled_set(obj: &EvasObject, setting: bool) {
    unsafe {
        evas_object_image_filled_set(obj, eseful::from_bool_to_eina(setting))
    }
}

/// Preload an image object's image data in the background.
pub fn object_image_preload(obj: &EvasObject, cancel: bool) {
    unsafe {
        evas_object_image_preload(obj, eseful::from_bool_to_eina(cancel))
    }
}

pub fn object_smart_callback_add<T>(e: &EvasObject, event: &str,
                                    cb: EvasSmartCb<T>, data: &Option<T>) {
    /* Transmute both Data and Callback into the C representation */
    let c_data: *c_void = unsafe { transmute(data) };
    let c_cb: CEvasSmartCb = unsafe { transmute(cb) };

    event.with_c_str(|c_event| unsafe {
        evas_object_smart_callback_add(e, c_event, c_cb, c_data)
    })
}

pub fn object_rectangle_add(e: &Evas) -> ~EvasObject {
    unsafe { cast_to_evas_obj(evas_object_rectangle_add(e)) }
}

/* Utility functions */
pub fn cast_to_evas_obj(ptr: *EvasObject) -> ~EvasObject {
    unsafe { transmute::<*EvasObject, ~EvasObject>(ptr) }
}
