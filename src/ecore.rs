// Ecore Rust bindings for EFL.
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

use types::{int, uint};
use std::ffi::CString;
use std::ptr;
use std::option::Option;
use std::mem::transmute;

use ecore::libc::{c_int, c_char, c_void};
use eseful::{to_c_args, EventInfo};
use eo;
use eina;
use evas;


pub enum EcoreEvas {}

pub enum EcoreEventHandler {}

pub enum EcoreEvent {
    EcoreEventNone,
    EcoreEventSignalUser,
    EcoreEventSignalHup,
    EcoreEventSignalExit,
    EcoreEventSignalPower,
    EcoreEventSignalRealtime,
    EcoreEventMemoryState,
    EcoreEventPowerState,
    EcoreEventLocaleChanged,
    EcoreEventHostnameChanged,
    EcoreEventSystemTimedateChanged,
    EcoreEventCount
}

pub struct SigInfo;

/*pub struct EcoreEventSignalExit {
    pub interrupt: eina::EinaBool,
    pub quit: eina::EinaBool,
    pub terminate: eina::EinaBool,
    pub ext_data: *const c_void,
    pub data: SigInfo
}*/

type EcoreTimer = eo::Eo;

pub static ECORE_CALLBACK_RENEW: eina::EinaBool = eina::EINA_TRUE;

/* High level callback notation */
pub type EcoreTaskCb<T> = fn (&Option<T>) -> eina::EinaBool;
/* C level callback notation */
type CEcoreTaskCb = fn (*const c_void) -> u8;

/* High level callback notation */
pub type EcoreEventHandlerCb<T> = fn (&Option<T>, int, &EventInfo) -> bool;
/* C level callback notation */
type CEcoreEventHandlerCb = fn (*const c_void, c_int, *const c_void) -> u8;

pub type EcoreEvasEventCb = fn (&EcoreEvas);
type _CEcoreEvasEventCb = fn (*const EcoreEvas);

#[link(name = "ecore")]
extern "C" {
    fn ecore_init() -> c_int;
    fn ecore_app_args_set(argc: c_int, argv: *const *const c_char);
    fn ecore_main_loop_begin();
    fn ecore_main_loop_quit();
    fn ecore_time_get() -> f64;
    fn ecore_time_unix_get() -> f64;
    fn ecore_shutdown() -> c_int;
    fn ecore_timer_add(inv: f64, func: CEcoreTaskCb, data: *const c_void);
    fn ecore_event_handler_add(htype: c_int, func: CEcoreEventHandlerCb, 
                               data: *const c_void) -> *const EcoreEventHandler;
}

#[link(name = "ecore_evas")]
extern "C" {
    fn ecore_evas_init() -> c_int;
    fn ecore_evas_shutdown() -> c_int;
    fn ecore_evas_new(engine_name: *const c_char, 
                      x: c_int, y: c_int, 
                      w: c_int, h: c_int,
                      extra_options: *const c_char) -> *const EcoreEvas;
    fn ecore_evas_show(ee: *const EcoreEvas);
    fn ecore_evas_get(ee: *const EcoreEvas) -> *const evas::Evas;
    fn ecore_evas_data_set(ee: *const EcoreEvas, key: *const c_char, data: *const c_void);
    fn ecore_evas_data_get(ee: *const EcoreEvas, key: *const c_char) -> *const c_void;
    fn ecore_evas_free(ee: *const EcoreEvas);
    fn ecore_evas_callback_resize_set(ee: *const EcoreEvas, func: _CEcoreEvasEventCb);
    fn ecore_evas_geometry_get(ee: *const EcoreEvas,
                               x: *const c_int, y: *const c_int,
                               w: *const c_int, h: *const c_int);
}

pub fn event_handler_add<T>(htype: EcoreEvent, 
                            func: EcoreEventHandlerCb<T>, 
                            data: &Option<T>) -> Box<EcoreEventHandler> {
    unsafe { 
        transmute(ecore_event_handler_add(htype as c_int, transmute(func), transmute(data)))
    }
}

pub fn init() -> i32 {
    unsafe { ecore_init() as i32 }
}

pub fn app_args_set(argc: uint, argv: Vec<String>) {
    let vchars_ptr: *const *const c_char = to_c_args(argv);
    unsafe { ecore_app_args_set(argc as c_int, vchars_ptr) }
}

pub fn main_loop_begin() {
    unsafe { ecore_main_loop_begin() }
}

pub fn main_loop_quit() {
    unsafe { ecore_main_loop_quit() }
}

pub fn shutdown() -> int {
    unsafe { ecore_shutdown() as int }
}

pub fn time_get() -> f64 {
    unsafe { ecore_time_get() }
}

pub fn time_unix_get() -> f64 {
    unsafe { ecore_time_unix_get() }
}

pub fn timer_add<T>(inv: f64, func: EcoreTaskCb<T>, data: &Option<T>) {
    let c_data: *const c_void = unsafe { transmute(data) };
    let c_func: CEcoreTaskCb = unsafe { transmute(func) };
    unsafe { ecore_timer_add(inv, c_func, c_data) }
}

pub fn evas_init() -> int {
    unsafe { ecore_evas_init() as int }
}

pub fn evas_shutdown() -> int {
    unsafe { ecore_evas_shutdown() as int }
}

// Creates a new Ecore_Evas based on engine name and common parameters.
// If 'engine_name' is None, it  use environment variable ECORE_EVAS_ENGINE,
// that can be undefined and in this case this call will try to find the
// first working engine.
pub fn evas_new(engine_name: Option<&str>,
                x: int, y: int,
                w: int, h: int,
                extra_options: &str) -> Box<EcoreEvas> {

    let c_extra_options = CString::new(extra_options).unwrap();
    unsafe {
        transmute(match engine_name {
            /* Null pointer */
            None => {
                ecore_evas_new(ptr::null(), x as c_int, y as c_int,
                               w as c_int, h as c_int, c_extra_options.as_ptr())
            },
            Some(ename) => {
                let c_engine_name = CString::new(ename).unwrap();
                ecore_evas_new(c_engine_name.as_ptr(), x as c_int, y as c_int,
                               w as c_int, h as c_int, c_extra_options.as_ptr())
            }
        })
    }
}

/// Show an Ecore_Evas' window.
pub fn evas_show(ee: &EcoreEvas) {
    unsafe { ecore_evas_show(ee) }
}

/// Get an Ecore_Evas's Evas.
pub fn evas_get(ee: &EcoreEvas) -> Box<evas::Evas> {
    unsafe { transmute(ecore_evas_get(ee)) }
}

/// Free an Ecore_Evas.
pub fn evas_free(ee: &EcoreEvas) {
    unsafe { ecore_evas_free(ee) }
}

/// Get the geometry of an Ecore_Evas.
pub fn evas_geometry_get(ee: &EcoreEvas, x: &int, y: &int, w: &int, h: &int) {
    unsafe {
        ecore_evas_geometry_get(ee, transmute(x), transmute(y),
                                transmute(w), transmute(h))
    }
}

/// Set a callback for Ecore_Evas resize events.
pub fn evas_callback_resize_set(ee: &EcoreEvas, func: EcoreEvasEventCb) {
    unsafe {
        ecore_evas_callback_resize_set(ee, transmute(func))
    }
}

/// Store user data in an Ecore_Evas structure.
pub fn evas_data_set<T>(ee: &EcoreEvas, key: &str, data: &T) {
    let c_key = CString::new(key).unwrap();
    unsafe {
        ecore_evas_data_set(ee, c_key.as_ptr(), transmute(data))
    }
}

/// Retrieve user data associated with an Ecore_Evas.
pub fn evas_data_get<T>(ee: &EcoreEvas, key: &str) -> Box<T> {
    let c_key = CString::new(key).unwrap();
    unsafe {
        transmute(ecore_evas_data_get(ee, c_key.as_ptr()))
    }
}
