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

use std::cast::forget;
use std::cast::transmute;
use std::ptr;
use std::option::{Option};

use ecore::libc::{c_int, c_char, c_void};
use eseful::{get_c_args, EventInfo};
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

pub struct EcoreEventSignalExit {
    pub interrupt: eina::EinaBool,
    pub quit: eina::EinaBool,
    pub terminate: eina::EinaBool,
    pub ext_data: *c_void,
    pub data: SigInfo
}

type EcoreTimer = evas::Eo;

pub static ECORE_CALLBACK_RENEW: eina::EinaBool = eina::EINA_TRUE;

/* High level callback notation */
pub type EcoreTaskCb<T> = fn (&Option<T>) -> eina::EinaBool;
/* C level callback notation */
type CEcoreTaskCb = fn (*c_void) -> u8;

/* High level callback notation */
pub type EcoreEventHandlerCb<T> = fn (&Option<T>, int, &EventInfo) -> eina::EinaBool;
/* C level callback notation */
type CEcoreEventHandlerCb = fn (*c_void, c_int, *c_void) -> u8;

#[link(name = "ecore")]
extern "C" {
    fn ecore_init() -> c_int;
    fn ecore_app_args_set(argc: c_int, argv: **c_char);
    fn ecore_main_loop_begin();
    fn ecore_main_loop_quit();
    fn ecore_time_get() -> f64;
    fn ecore_time_unix_get() -> f64;
    fn ecore_shutdown() -> c_int;
    fn ecore_timer_add(inv: f64, func: CEcoreTaskCb, data: *c_void);
    fn ecore_event_handler_add(htype: c_int, func: CEcoreEventHandlerCb, 
                               data: *c_void) -> *EcoreEventHandler;
    fn ecore_evas_init() -> c_int;
    fn ecore_evas_shutdown() -> c_int;
    fn ecore_evas_new(engine_name: *c_char, 
                      x: c_int, y: c_int, 
                      w: c_int, h: c_int,
                      extra_options: *c_char) -> *EcoreEvas;
    fn ecore_evas_show(ee: *EcoreEvas);
    fn ecore_evas_get(ee: *EcoreEvas) -> *evas::Evas;
}

#[link(name = "ecore_evas")]
extern "C" {
    fn ecore_evas_free(ee: *EcoreEvas);
}

pub fn event_handler_add<T>(htype: EcoreEvent, 
                            func: EcoreEventHandlerCb<T>, 
                            data: ~Option<T>) -> ~EcoreEventHandler {
    /* Transmute both data and Callback into the C representation */
    let c_data: *c_void = unsafe { transmute(data) };
    let c_func: CEcoreEventHandlerCb = unsafe { transmute(func) };

    unsafe { 
        let eh = ecore_event_handler_add(htype as c_int, c_func, c_data); 
        transmute::<*EcoreEventHandler,~EcoreEventHandler>(eh)
    }
}

pub fn init() -> i32 {
    unsafe { ecore_init() as i32 }
}

pub fn app_args_set(argc: uint, argv: ~[~str]) {
    let vchars_ptr: **c_char = get_c_args(argv);
    unsafe {
        ecore_app_args_set(argc as c_int, vchars_ptr);
        // Forget this value so it can be stored statically from C
        forget(vchars_ptr); 
    }
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

pub fn timer_add<T>(inv: f64, func: EcoreTaskCb<T>, data: ~Option<T>) {
    let c_data: *c_void = unsafe { transmute(data) };
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
pub fn evas_new(engine_name: Option<~str>, 
                x: int, y: int, 
                w: int, h: int,
                extra_options: ~str) -> ~EcoreEvas {

    let ee: *EcoreEvas = 
        match engine_name {
            /* Null pointer */
            None =>
                extra_options.with_c_str(|c_extra_options| unsafe {
                    ecore_evas_new(ptr::null(), x as c_int, y as c_int, 
                                   w as c_int, h as c_int, c_extra_options)
                }),
            Some(ename) =>
                ename.with_c_str(|c_engine_name| unsafe {
                    extra_options.with_c_str(|c_extra_options| {
                        ecore_evas_new(c_engine_name, x as c_int, y as c_int, 
                                       w as c_int, h as c_int, c_extra_options)
                    })
                })
        };
    unsafe { transmute::<*EcoreEvas, ~EcoreEvas>(ee) }

}

pub fn evas_show(ee: &EcoreEvas) {
    unsafe { ecore_evas_show(ee) }
}

pub fn evas_get(ee: &EcoreEvas) -> ~evas::Evas {
    unsafe {
        let ee = ecore_evas_get(ee);
        transmute::<*evas::Evas, ~evas::Evas>(ee)
    }
}

pub fn evas_free(ee: &EcoreEvas) {
    unsafe { ecore_evas_free(ee) }
}