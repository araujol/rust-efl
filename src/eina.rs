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
extern crate core;

use std::ptr;
use std::mem::transmute;
use std::option::Option;
use eina::core::mem::uninitialized;
use eina::libc::{c_void, c_int, c_uint};
use eseful;


pub type EinaBool = u8;
pub static EINA_FALSE: EinaBool = 0u8;
pub static EINA_TRUE:  EinaBool = 1u8;

type _EinaMagic = uint;
type _CEinaMagic = c_uint;

/*
 * EinaList object.
 */
/// EinaList object.
pub struct EinaList<'r, T> {
    _eo: *mut _EinaList<'r, T>
}

/// Representation of an Eina_List.
pub struct _EinaList<'r, T> {
    data: &'r T,
    next: *mut _EinaList<'r, T>,
    prev: *mut _EinaList<'r, T>,
    accounting: *_EinaListAccounting<'r, T>,

    __magic: _EinaMagic
}

pub struct _EinaListAccounting<'r, T> {
    last: *mut _EinaList<'r, T>,
    count: uint,

    __magic: _EinaMagic
}

/// C representation of an Eina_List.
pub struct _CEinaList {
    data: *c_void,
    next: *_CEinaList,
    prev: *_CEinaList,
    accounting: *_CEinaListAccounting,

    __magic: _CEinaMagic
}

pub struct _CEinaListAccounting {
    last: *_CEinaList,
    count: c_uint,

    __magic: _CEinaMagic
}

/*
 * Inlined list type (EinaInlist).
 */
/// Inlined list type.
pub struct EinaInlist {
    _eo: *_EinaInlist
}

pub struct _EinaInlist {
    /// Next node
    next: *_EinaInlist,
    /// Previous node
    prev: *_EinaInlist,
    /// Last node
    last: *_EinaInlist
}

/*
 * EinaHash type.
 */
/// Type for a generic hash table.
pub struct _EinaHash<T> {
    key_length_cb: EinaKeyLength<T>,
    key_cmp_cb: EinaKeyCmp<T>,
    key_hash_cb: EinaKeyHash<T>,
    data_free_cb: EinaFreeCb<T>,

    buckets: **EinaRbtree,
    size: int,
    mask: int,

    population: int,

    buckets_power_size: int,

    __magic: _EinaMagic
}

pub struct _CEinaHash {
    key_length_cb: _CEinaKeyLength,
    key_cmp_cb: _CEinaKeyCmp,
    key_hash_cb: _CEinaKeyHash,
    data_free_cb: _CEinaFreeCb,

    buckets: **EinaRbtree,
    size: c_int,
    mask: c_int,

    population: c_int,

    buckets_power_size: c_int,

    __magic: _CEinaMagic
}

/// Type for a function to determine the length of a hash key.
pub type EinaKeyLength<T> = fn (&T) -> int;
type _CEinaKeyLength = fn (*c_void) -> c_int;

/// Type for a function to compare two hash keys.
pub type EinaKeyCmp<T> = fn (&T, int, &T, int) -> c_int;
type _CEinaKeyCmp = fn (*c_void, c_int, *c_void, c_int) -> c_int;

/// Type for a function to create a hash key.
pub type EinaKeyHash<T> = fn (&T, int) -> int;
type _CEinaKeyHash = fn (*c_void, c_int) -> c_int;

/// A callback type used to free data when iterating over a container.
pub type EinaFreeCb<T> = fn (&T);
type _CEinaFreeCb = fn (*c_void);

/// Type for a Red-Black tree node. It should be inlined into user's type.
pub struct EinaRbtree {
    son: *[EinaRbtree, ..2],

    color: uint
}


#[link(name = "eina")]
extern "C" {
    fn eina_init() -> c_int;
    fn eina_shutdown() -> c_int;
    fn eina_list_free(list: *_CEinaList) -> *_CEinaList;
    fn eina_list_append(list: *_CEinaList, data: *c_void) -> *_CEinaList;
    fn eina_list_prepend(list: *_CEinaList, data: *c_void) -> *_CEinaList;
    /* Inline list type */
    fn eina_inlist_append(in_list: *_EinaInlist, in_item: *_EinaInlist) -> *_EinaInlist;
    fn eina_inlist_prepend(in_list: *_EinaInlist, in_item: *_EinaInlist) -> *_EinaInlist;
    fn eina_inlist_promote(list: *_EinaInlist, item: *_EinaInlist) -> *_EinaInlist;
    fn eina_inlist_demote(list: *_EinaInlist, item: *_EinaInlist) -> *_EinaInlist;
    fn eina_inlist_remove(in_list: *_EinaInlist, in_item: *_EinaInlist) -> *_EinaInlist;
    /* Hash type */
    fn eina_hash_stringshared_new(data_free_cb: _CEinaFreeCb) -> *_CEinaHash;
    fn eina_hash_string_superfast_new(data_free_cb: _CEinaFreeCb) -> *_CEinaHash;
    fn eina_hash_add(hash: *_CEinaHash, key: *c_void, data: *c_void) -> EinaBool;
    fn eina_hash_find(hash: *_CEinaHash, key: *c_void) -> *c_void;
    fn eina_hash_population(hash: *_CEinaHash) -> c_int;
    fn eina_hash_free(hash: *_CEinaHash);
}


/* Implementations for EinaList type */
impl<'r, T> EinaList<'r, T> {
    /// Create high level EinaList object.
    pub fn new(el: *mut _EinaList<'r, T>) -> EinaList<'r, T> {
        EinaList { _eo: el }
    }

}

/// EinaList implements the Iterator trait.
impl<'r, T> Iterator<&'r T> for EinaList<'r, T> {

    fn next(&mut self) -> Option<&'r T> {
        let v = list_data_get(self._eo);
        *self = match list_next(self._eo) {
            None => EinaList { _eo: ptr::mut_null() },
            Some(l) => EinaList { _eo: l }
        };
        return v
    }

}

/* Implementations for EinaInlist type */
impl EinaInlist {
    pub fn new(el: *_EinaInlist) -> EinaInlist {
        EinaInlist { _eo: el }
    }
}

impl<'r, T> Iterator<&'r T> for EinaInlist {
    fn next(&mut self) -> Option<&'r T> {
        let v = if self._eo.is_null() {
            None
        } else {
            let elm: &T = inlist_container_get(*self);
            Some(elm)
        };
        // Get next value if current value is valid (Some).
        if v.is_some() { self._eo = unsafe { (*self._eo).next } };
        return v
    }
}


/// Initialize the Eina library.
pub fn init() -> int { unsafe { eina_init() as int } }

/// Shut down the Eina library.
pub fn shutdown() -> int { unsafe { eina_shutdown() as int } }

/// Free an entire list and all the nodes, ignoring the data contained.
pub fn list_free<T>(list: *mut _EinaList<T>) -> *mut _EinaList<T> {
    unsafe {
        transmute::<*_CEinaList,*mut _EinaList<T>>(
            eina_list_free(transmute::<*mut _EinaList<T>,*_CEinaList>(list)))
    }
}

/// Append the given data to the given linked list.
/// This function appends data to list. If list is 'None', a new list is returned.
pub fn list_append<T>(list: Option<*mut _EinaList<T>>, data: &T) -> *mut _EinaList<T> {
    unsafe {
        let c_data: *c_void = transmute(data);
        match list {
            None => transmute::<*_CEinaList,*mut _EinaList<T>>(
                eina_list_append(ptr::null(), c_data)),
            Some(l) => transmute::<*_CEinaList,*mut _EinaList<T>>(
                eina_list_append(transmute::<*mut _EinaList<T>,*_CEinaList>(l), c_data))
        }
    }
}

/// Prepends the given data to the given linked list.
/// This function prepends data to list. If list is 'None', a new list is returned.
pub fn list_prepend<T>(list: Option<*mut _EinaList<T>>, data: &T) -> *mut _EinaList<T> {
    unsafe {
        let c_data: *c_void = transmute(data);
        match list {
            None => transmute::<*_CEinaList,*mut _EinaList<T>>(
                eina_list_prepend(ptr::null(), c_data)),
            Some(l) => transmute::<*_CEinaList,*mut _EinaList<T>>(
                eina_list_prepend(transmute::<*mut _EinaList<T>,*_CEinaList>(l), c_data))
        }
    }
}

/// Get the list node data member.
#[inline]
pub fn list_data_get<'r, T>(list: *mut _EinaList<'r, T>) -> Option<&'r T> {
    if list.is_null() { return None }
    unsafe { Some((*list).data) }
}

/// Set the list node data member.
#[inline]
pub fn list_data_set<'r, T>(list: *mut _EinaList<'r, T>, new_data: &'r T) -> Option<&'r T> {
    if list.is_null() { return None }
    unsafe {
        let olddata = (*list).data;
        (*list).data = new_data;
        Some(olddata)
    }
}

/// Get the last list node in the list.
#[inline]
pub fn list_last<'a, T>(list: *mut _EinaList<'a, T>) -> Option<*mut _EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe { Some((*(*list).accounting).last) }
}

/// Get the next list node after the specified list node.
#[inline]
pub fn list_next<'a, T>(list: *mut _EinaList<'a, T>) -> Option<*mut _EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe {
        // Let's be nice and return None for nullable next
        if (*list).next.is_null() { return None }
        Some((*list).next)
    }
}

/// Get the previous list node before the specified list node.
#[inline]
pub fn list_prev<'a, T>(list: *mut _EinaList<'a, T>) -> Option<*mut _EinaList<'a, T>> {
    if list.is_null() { return None }
    unsafe {
        // Let's be nice and return None for nullable prev
        if (*list).prev.is_null() { return None }
        Some((*list).prev)
    }
}

/// Get the count of the number of items in a list.
#[inline]
pub fn list_count<'r, T>(list: *mut _EinaList<'r, T>) -> uint {
    if list.is_null() { return 0 }
    unsafe {
        (*(*list).accounting).count
    }
}

/// Convenient function to get the last list node data member.
#[inline]
pub fn list_last_data_get<'r, T>(list: *mut _EinaList<'r, T>) -> Option<&'r T> {
    match list_last(list) {
        None => None,
        Some(last) => list_data_get(last)
    }
}

/* Inline list functions */
/// Add a new node to end of a list.
pub fn inlist_append(in_list: Option<EinaInlist>, in_item: *_EinaInlist) -> EinaInlist {
    EinaInlist {
        _eo: unsafe {
            match in_list {
                None => eina_inlist_append(ptr::null(), in_item),
                Some(lst) => eina_inlist_append(lst._eo, in_item)
            }
        }
    }
}

/// Add a new node to beginning of list.
pub fn inlist_prepend(in_list: Option<EinaInlist>, in_item: *_EinaInlist) -> EinaInlist {
    EinaInlist {
        _eo: unsafe {
            match in_list {
                None => eina_inlist_prepend(ptr::null(), in_item),
                Some(lst) => eina_inlist_prepend(lst._eo, in_item)
            }
        }
    }
}

/// Move existing node to beginning of list.
pub fn inlist_promote(in_list: Option<EinaInlist>, in_item: *_EinaInlist) -> EinaInlist {
    EinaInlist {
        _eo: unsafe {
            match in_list {
                None => eina_inlist_promote(ptr::null(), in_item),
                Some(lst) => eina_inlist_promote(lst._eo, in_item)
            }
        }
    }
}

/// Move existing node to end of list.
pub fn inlist_demote(in_list: Option<EinaInlist>, in_item: *_EinaInlist) -> EinaInlist {
    EinaInlist {
        _eo: unsafe {
            match in_list {
                None => eina_inlist_demote(ptr::null(), in_item),
                Some(lst) => eina_inlist_demote(lst._eo, in_item)
            }
        }
    }
}

/// Remove node from list.
pub fn inlist_remove(in_list: EinaInlist, in_item: *_EinaInlist) -> EinaInlist {
    EinaInlist {
        _eo: unsafe { eina_inlist_remove(in_list._eo, in_item) }
    }
}

/// Get the container object of an in_list.
pub fn inlist_container_get<T>(in_list: EinaInlist) -> &T {
    unsafe { transmute(in_list._eo) }
}

/// Convenient function for object allocation.
#[inline]
pub fn object<T>() -> T {
    unsafe { uninitialized::<T>() }
}

/// Macro to get the inlist object of a struct.
#[macro_export]
macro_rules! inlist_get(
    ($inlist:ident) => (unsafe {
        use std::mem::transmute;
        transmute(&($inlist.__in_list))
    })
)

/* Hash type functions */
/// Create a new hash table optimized for stringshared values.
pub fn hash_stringshared_new<T>(data_free_cb: EinaFreeCb<T>) -> *mut _EinaHash<T> {
    unsafe { transmute(eina_hash_stringshared_new(transmute(data_free_cb))) }
}

/// Create a new hash table for use with strings.
pub fn hash_string_superfast_new<T>(data_free_cb: EinaFreeCb<T>) -> *mut _EinaHash<T> {
    unsafe { transmute(eina_hash_string_superfast_new(transmute(data_free_cb))) }
}

/// Add an entry to the given hash table.
pub fn hash_add<T>(hash: *mut _EinaHash<T>, key: &T, data: &T) -> bool {
    eseful::from_eina_to_bool(unsafe {
        eina_hash_add(transmute(hash), transmute(key), transmute(data))
    })
}

/// Retrieve a specific entry in the given hash table.
pub fn hash_find<T>(hash: *mut _EinaHash<T>, key: &T) -> &T {
    unsafe { transmute(eina_hash_find(transmute(hash), transmute(key))) }
}

/// Returns the number of entries in the given hash table.
pub fn hash_population<T>(hash: *mut _EinaHash<T>) -> int {
    unsafe { eina_hash_population(transmute(hash)) as int }
}

/// Free the given hash table resources.
pub fn hash_free<T>(hash: *mut _EinaHash<T>) {
    unsafe { eina_hash_free(transmute(hash)) }
}
