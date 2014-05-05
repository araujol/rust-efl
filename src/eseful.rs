// Utility functions/data for EFL Rust bindings.
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

/*
 * Data and functions to make different tasks easier and more
 * convenient (mainly handlers/callbacks).
 */

extern crate libc;

use std::c_str::{CString};
use std::cast::forget;

use eseful::libc::c_char;
use eina;

// Empty value handy to use in the 'data' field for callbacks.
pub static Empty: Option<()> = None;

// Callbacks event.
pub struct EventInfo;

pub fn get_c_args(argv: ~[~str]) -> **c_char {
    let mut vcstrs: Vec<CString> = Vec::new();
    let mut vchars: Vec<*c_char> = Vec::new();

    for s in argv.iter() { vcstrs.push(s.to_c_str()); }
        
    for c_s in vcstrs.iter() {
        c_s.with_ref(|c_buf| { vchars.push(c_buf);  });
    }
    
    let vchars_ptr = vchars.as_ptr();

    unsafe {
        // Forget all these values so they can be stored statically from C 
        forget(vcstrs);
        forget(vchars);
    }
    return vchars_ptr;
}

pub fn from_bool_to_eina(b: bool) -> eina::EinaBool {
    match b {
        true => eina::EINA_TRUE,
        false => eina::EINA_FALSE
    }
}

pub fn from_eina_to_bool(eb: eina::EinaBool) -> bool {
    match eb {
        eina::EINA_TRUE => true,
        _ => false
    }
}
