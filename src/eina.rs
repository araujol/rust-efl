// Eina Rust bindings for EFL.
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

use std::cast::transmute;
use std::ptr;
use std::option::{Option};
use eina::libc::{c_void, c_int, c_uint};

pub type EinaBool = u8;
pub static EINA_FALSE: EinaBool = 0u8;
pub static EINA_TRUE:  EinaBool = 1u8;

type EinaMagic = uint;
type _CEinaMagic = c_uint;

/// Rust 'higher-level' representation of an Eina_List.
pub struct EinaList<'r, T> {
    pub data: &'r T,
    pub next: *mut EinaList<'r, T>,
    pub prev: *mut EinaList<'r, T>,
    pub accounting: *EinaListAccounting<'r, T>,

    __magic: EinaMagic
}

pub struct EinaListAccounting<'r, T> {
    pub last: *mut EinaList<'r, T>,
    pub count: uint,

    __magic: EinaMagic
}

/// C representation of an Eina_List.
struct _CEinaList {
    data: *c_void,
    next: *_CEinaList,
    prev: *_CEinaList,
    accounting: *_CEinaListAccounting,

    __magic: _CEinaMagic
}

struct _CEinaListAccounting {
    last: *_CEinaList,
    count: c_uint,

    __magic: _CEinaMagic
}

#[link(name = "eina")]
extern "C" {
    fn eina_init() -> c_int;
    fn eina_shutdown() -> c_int;
    fn eina_list_free(list: *_CEinaList) -> *_CEinaList;
    fn eina_list_append(list: *_CEinaList, data: *c_void) -> *_CEinaList;
    fn eina_list_prepend(list: *_CEinaList, data: *c_void) -> *_CEinaList;
}

/// Initialize the Eina library.
pub fn init() -> int { unsafe { eina_init() as int } }

/// Shut down the Eina library.
pub fn shutdown() -> int { unsafe { eina_shutdown() as int } }

/// Free an entire list and all the nodes, ignoring the data contained.
pub fn list_free<T>(list: *mut EinaList<T>) -> *mut EinaList<T> {
    unsafe {
        transmute::<*_CEinaList,*mut EinaList<T>>(
            eina_list_free(transmute::<*mut EinaList<T>,*_CEinaList>(list)))
    }
}

/// Append the given data to the given linked list.
/// This function appends data to list. If list is 'None', a new list is returned.
pub fn list_append<T>(list: Option<*mut EinaList<T>>, data: &T) -> *mut EinaList<T> {
    unsafe {
        let c_data: *c_void = transmute(data);
        match list {
            None => transmute::<*_CEinaList,*mut EinaList<T>>(
                eina_list_append(ptr::null(), c_data)),
            Some(l) => transmute::<*_CEinaList,*mut EinaList<T>>(
                eina_list_append(transmute::<*mut EinaList<T>,*_CEinaList>(l), c_data))
        }
    }
}

/// Prepends the given data to the given linked list.
/// This function prepends data to list. If list is 'None', a new list is returned.
pub fn list_prepend<T>(list: Option<*mut EinaList<T>>, data: &T) -> *mut EinaList<T> {
    unsafe {
        let c_data: *c_void = transmute(data);
        match list {
            None => transmute::<*_CEinaList,*mut EinaList<T>>(
                eina_list_prepend(ptr::null(), c_data)),
            Some(l) => transmute::<*_CEinaList,*mut EinaList<T>>(
                eina_list_prepend(transmute::<*mut EinaList<T>,*_CEinaList>(l), c_data))
        }
    }
}

/// Get the list node data member.
#[inline]
pub fn list_data_get<'r, T>(list: *mut EinaList<'r, T>) -> Option<&'r T> {
    if list.is_null() { return None }
    unsafe { Some((*list).data) }
}

/// Set the list node data member.
#[inline]
pub fn list_data_set<'r, T>(list: *mut EinaList<'r, T>, new_data: &'r T) -> Option<&'r T> {
    if list.is_null() { return None }
    unsafe {
        let olddata = (*list).data;
        (*list).data = new_data;
        Some(olddata)
    }
}

/// Get the last list node in the list.
#[inline]
pub fn list_last<'a, T>(list: *mut EinaList<'a, T>) -> Option<*mut EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe { Some((*(*list).accounting).last) }
}

/// Get the next list node after the specified list node.
#[inline]
pub fn list_next<'a, T>(list: *mut EinaList<'a, T>) -> Option<*mut EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe {
        // Let's be nice and return None for nullable next
        if (*list).next.is_null() { return None }
        Some((*list).next)
    }
}

/// Get the previous list node before the specified list node.
#[inline]
pub fn list_prev<'a, T>(list: *mut EinaList<'a, T>) -> Option<*mut EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe {
        // Let's be nice and return None for nullable prev
        if (*list).prev.is_null() { return None }
        Some((*list).prev)
    }
}

/// Get the count of the number of items in a list.
#[inline]
pub fn list_count<'r, T>(list: *mut EinaList<'r, T>) -> uint {
    if list.is_null() { return 0 }
    unsafe {
        (*(*list).accounting).count
    }
}

/// Convenient function to get the last list node data member.
#[inline]
pub fn list_last_data_get<'r, T>(list: *mut EinaList<'r, T>) -> Option<&'r T> {
    match list_last(list) {
        None => None,
        Some(last) => list_data_get(last)
    }
}
