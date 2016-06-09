// Eio Rust bindings for EFL.
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

use types::{int};
use eio::libc::{c_int, c_char, c_void, c_float, c_longlong, mode_t};
use std::mem::transmute;
use std::ffi::CString;

use eina;

pub enum EioFile {}

#[derive(Debug)]
pub enum EioFileOp {
    /// I/O operation is about a specific file copy.
    EioFileCopy,
    /// I/O operation is about a specific file move.
    EioFileMove,
    /// I/O operation is about a specific directory copy.
    EioDirCopy,
    /// I/O operation is about a specific directory move.
    EioDirMove,
    /// I/O operation is about destroying a path: 
    /// source will point to base path to be destroyed, 
    /// and dest will point to to path destroyed by this I/O.
    EioUnlink,
    /// I/O operation is trying to get uid from user name.
    EioFileGetpwnam,
    /// I/O operation is trying to get gid from user name.
    EioFileGetgrnam
}

pub struct EioProgress {
    pub op: EioFileOp,

    pub current: c_longlong,
    pub max: c_longlong,
    pub percent: c_float,

    pub source: *const c_char,
    pub dest: *const c_char
}

pub type EioFilterCb<T> = fn (&mut T, &EioFile, *const c_char) -> bool;
type _CEioFilterCb = fn (*const c_void, *const EioFile, *const c_char) -> eina::EinaBool;

pub type EioMainCb<T> = fn (&mut T, &EioFile, *const c_char);
type _CEioMainCb = fn (*const c_void, *const EioFile, *const c_char);

pub type EioDoneCb<T> = fn (&mut T, &EioFile);
type _CEioDoneCb = fn (*const c_void, *const EioFile);
 
pub type EioErrorCb<T> = fn (&mut T, &EioFile, int);
type _CEioErrorCb = fn (*const c_void, *const EioFile, c_int);

pub type EioProgressCb<T> = fn (&mut T, &EioFile, &EioProgress);
type _CEioProgressCb = fn (*const c_void, *const EioFile, *const EioProgress);

#[link(name = "eio")]
extern "C" {
    fn eio_init() -> c_int;
    fn eio_shutdown() -> c_int;
    fn eio_file_ls(dir: *const c_char, filter_cb: _CEioFilterCb,
                   main_cb: _CEioMainCb, done_cb: _CEioDoneCb,
                   error_cb: _CEioErrorCb, data: *const c_void) -> *const EioFile;
    fn eio_file_mkdir(path: *const c_char, mode: mode_t,
                      done_cb: _CEioDoneCb, error_cb: _CEioErrorCb,
                      data: *const c_void) -> *const EioFile;
    fn eio_file_move(source: *const c_char, dest: *const c_char,
                     progress_cb: _CEioProgressCb, done_cb: _CEioDoneCb,
	             error_cb: _CEioErrorCb, data: *const c_void) -> *const EioFile;
    fn eio_file_copy(source: *const c_char, dest: *const c_char,
                     progress_cb: _CEioProgressCb, done_cb: _CEioDoneCb,
	             error_cb: _CEioErrorCb, data: *const c_void) -> *const EioFile;
}

/// Initialize eio and all it's required submodule.
pub fn init() -> int {
    unsafe { eio_init() as int }
}

/// Shutdown eio and all it's submodule if possible.
pub fn shutdown() -> int {
    unsafe { eio_shutdown() as int }
}

/// List contents of a directory without locking your app.
pub fn file_ls<T>(dir: &str, filter_cb: EioFilterCb<T>,
                  main_cb: EioMainCb<T>, done_cb: EioDoneCb<T>,
                  error_cb: EioErrorCb<T>, data: &T) -> Box<EioFile> {

    let c_dir = CString::new(dir).unwrap();
    unsafe {
        let c_filter_cb: _CEioFilterCb = transmute(filter_cb);
        let c_main_cb: _CEioMainCb = transmute(main_cb);
        let c_done_cb: _CEioDoneCb = transmute(done_cb);
        let c_error_cb: _CEioErrorCb = transmute(error_cb);
        let c_data: *const c_void = transmute(data);

        transmute(eio_file_ls(c_dir.as_ptr(), c_filter_cb, c_main_cb,
                              c_done_cb, c_error_cb, c_data))
    }
}

/// Create a new directory.
pub fn file_mkdir<T>(path: &str, mode: mode_t, done_cb: EioDoneCb<T>,
                     error_cb: EioErrorCb<T>, data: &T) -> Box<EioFile> {
    let c_path = CString::new(path).unwrap();
    unsafe {
        let c_done_cb: _CEioDoneCb = transmute(done_cb);
        let c_error_cb: _CEioErrorCb = transmute(error_cb);
        let c_data: *const c_void = transmute(data);

        transmute(eio_file_mkdir(c_path.as_ptr(), mode, c_done_cb, c_error_cb, c_data))
    }
}

/// Move a file asynchronously.
pub fn file_move<T>(source: &str, dest: &str, 
                    progress_cb: EioProgressCb<T>, done_cb: EioDoneCb<T>, 
                    error_cb: EioErrorCb<T>, data: &T) -> Box<EioFile> {
    let c_source = CString::new(source).unwrap();
    let c_dest = CString::new(dest).unwrap();

    unsafe {
        let c_progress_cb: _CEioProgressCb = transmute(progress_cb);
        let c_done_cb: _CEioDoneCb = transmute(done_cb);
        let c_error_cb: _CEioErrorCb = transmute(error_cb);
        let c_data: *const c_void = transmute(data);

        transmute(eio_file_move(c_source.as_ptr(), c_dest.as_ptr(), c_progress_cb,
                                c_done_cb, c_error_cb, c_data))
    }
}

/// Copy a file asynchronously.
pub fn file_copy<T>(source: &str, dest: &str, 
                    progress_cb: EioProgressCb<T>, done_cb: EioDoneCb<T>, 
                    error_cb: EioErrorCb<T>, data: &T) -> Box<EioFile> {
    let c_source = CString::new(source).unwrap();
    let c_dest = CString::new(dest).unwrap();

    unsafe {
        let c_progress_cb: _CEioProgressCb = transmute(progress_cb);
        let c_done_cb: _CEioDoneCb = transmute(done_cb);
        let c_error_cb: _CEioErrorCb = transmute(error_cb);
        let c_data: *const c_void = transmute(data);

        transmute(eio_file_copy(c_source.as_ptr(), c_dest.as_ptr(), c_progress_cb,
                                c_done_cb, c_error_cb, c_data))
    }
}
