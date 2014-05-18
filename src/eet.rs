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
use std::mem::transmute;
use std::fmt::{Show, Formatter, Result};


/// EetFile object.
pub struct EetFile {
    _eo: *_EetFile
}

/// EetValue object.
/// This object is a convenient wrapper around values returned by 'read'.
pub struct EetValue<T> {
    _value: *T
}

impl<T> EetValue<T> {
    pub fn getVal(&self) -> *T { self._value }
}

#[unsafe_destructor]
impl<T> Drop for EetValue<T> {
    fn drop(&mut self) {
        unsafe { free(transmute(self._value)) }
    }
}

impl<T: Show> Show for EetValue<T> {
    fn fmt(&self, _fmt: &mut Formatter) -> Result {
        unsafe { write!(_fmt, "{}", *((*self)._value)) }
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

#[deriving(Show, FromPrimitive)]
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
    fn eet_open(file: *c_char, mode: c_uint) -> *_EetFile;
    fn eet_read(ef: *_EetFile, name: *c_char, size_ret: *mut c_int) -> *c_void;
    fn eet_write(ef: *_EetFile, name: *c_char, data: *c_void,
                 size: c_uint, compress: c_int) -> c_int;
    fn eet_close(ef: *_EetFile) -> c_uint;
    fn eet_sync(ef: *_EetFile) -> c_uint;
}

/// Initialize the EET library.
pub fn init() -> int { unsafe { eet_init() as int } }

/// Shut down the EET library.
pub fn shutdown() -> int { unsafe { eet_shutdown() as int } }

/// Clear eet cache.
pub fn clearcache() { unsafe { eet_clearcache() } }

/// Open an eet file on disk, and returns a handle to it.
pub fn open(file: &str, mode: EetFileMode) -> EetFile {
    file.with_c_str(|c_file| unsafe {
        EetFile { _eo: eet_open(c_file, mode as c_uint) }
    })
}

/// Read a specified entry from an eet file and return data.
pub fn read<T>(ef: EetFile, name: &str, size_ret: &mut i32) -> EetValue<T> {
    name.with_c_str(|c_name| unsafe {
        EetValue {
            _value: transmute::<*c_void,*T>(eet_read(ef._eo, c_name, size_ret))
        }
    })
}

/// Write a specified entry to an eet file handle.
pub fn write<T>(ef: EetFile, name: &str, data: &T,
                size: uint, compress: int) -> int {
    name.with_c_str(|c_name| unsafe {
        eet_write(ef._eo, c_name, transmute(data), size as c_uint, compress as c_int) as int
    })
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
