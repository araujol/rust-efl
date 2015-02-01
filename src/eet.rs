// Eet Rust bindings for EFL.
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
extern crate core;

use eet::libc::{c_int, c_uint, c_char, c_void, free};
use eet::core::num::FromPrimitive;
use std::fmt;
use std::{mem, ffi};

/// EetFile object.
pub struct EetFile {
    _eo: *const _EetFile
}

/// EetValue object.
/// This object is a convenient wrapper around values returned by 'read'.
pub struct EetValue<T> {
    _value: *const T
}

impl<T> EetValue<T> {
    pub fn get_val(&self) -> *const T { self._value }
}

#[unsafe_destructor]
impl<T> Drop for EetValue<T> {
    fn drop(&mut self) {
        unsafe { free(mem::transmute(self._value)) }
    }
}

impl<T: fmt::Debug> fmt::Debug for EetValue<T> {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(_fmt, "{:?}", *((*self)._value)) }
    }
}

/// Internal representation of an EetFile object.
pub enum _EetFile {}

/// Modes that a file can be opened.
pub enum EetFileMode {
    /// File is read-only.
    EetFileModeRead,
    /// File is write-only.
    EetFileModeWrite,
    /// File is for both read and write.
    EetFileModeReadWrite	
}

#[derive(Debug, FromPrimitive)]
pub enum EetError {
    EetErrorNone,
    EetErrorBadObject,
    EetErrorEmpty,
    EetErrorNotWritable,
    EetErrorOutOfMemory,
    EetErrorWriteError,
    EetErrorWriteErrorFileTooBig,
    EetErrorWriteErrorIOError,
    EetErrorWriteErrorOutOfSpace,
    EetErrorWriteErrorFileClosed,
    EetErrorMmapFailed,
    EetErrorX509EncodingFailed,
    EetErrorSignatureFailed,
    EetErrorInvalidSignature,
    EetErrorNotSigned,
    EetErrorNotImplemented,
    EetErrorPrngNotSeeded,
    EetErrorEncryptFailed,
    EetErrorDecryptFailed
}

#[link(name = "eet")]
extern "C" {
    fn eet_init() -> c_int;
    fn eet_shutdown() -> c_int;
    fn eet_clearcache();
    fn eet_open(file: *const c_char, mode: c_uint) -> *const _EetFile;
    fn eet_read(ef: *const _EetFile, name: *const c_char, size_ret: *mut c_int) -> *const c_void;
    fn eet_write(ef: *const _EetFile, name: *const c_char, data: *const c_void,
                 size: c_uint, compress: c_int) -> c_int;
    fn eet_close(ef: *const _EetFile) -> c_uint;
    fn eet_sync(ef: *const _EetFile) -> c_uint;
}

/// Initialize the EET library.
pub fn init() -> isize { unsafe { eet_init() as isize } }

/// Shut down the EET library.
pub fn shutdown() -> isize { unsafe { eet_shutdown() as isize } }

/// Clear eet cache.
pub fn clearcache() { unsafe { eet_clearcache() } }

/// Open an eet file on disk, and returns a handle to it.
pub fn open(file: &str, mode: EetFileMode) -> EetFile {
    let imode = mode as c_uint;
    let c_file = ffi::CString::from_slice(file.as_bytes());
    unsafe {
        EetFile { _eo: eet_open(c_file.as_ptr(), imode) }
    }
}

/// Read a specified entry from an eet file and return data.
pub fn read<T>(ef: EetFile, name: &str, size_ret: &mut i32) -> EetValue<T> {
    let c_name = ffi::CString::from_slice(name.as_bytes());
    unsafe {
        EetValue {
            _value: mem::transmute::<*const c_void,*const T>(eet_read(ef._eo, c_name.as_ptr(), size_ret))
        }
    }
}

/// Write a specified entry to an eet file handle.
pub fn write<T>(ef: EetFile, name: &str, data: &T, size: usize, compress: isize) -> isize {
    let c_name = ffi::CString::from_slice(name.as_bytes());
    unsafe {
        eet_write(ef._eo, c_name.as_ptr(), mem::transmute(data), size as c_uint, compress as c_int) as isize
    }
}

//// Close an eet file handle and flush pending writes.
pub fn close(ef: EetFile) -> EetError {
    let v: Option<EetError> = FromPrimitive::from_u32(unsafe { eet_close(ef._eo) });
    v.unwrap()
}

/// Sync content of an eet file handle, flushing pending writes.
pub fn sync(ef: EetFile) -> EetError {
    let v: Option<EetError> = FromPrimitive::from_u32(unsafe { eet_sync(ef._eo) });
    v.unwrap()
}
