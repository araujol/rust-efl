// Eo Rust bindings for EFL.
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

use eo::libc::{c_char, c_int};
use eina;
use eseful;


pub enum Eo {}
pub type EoClass = Eo;


#[link(name = "eo")]
extern "C" {
    fn _eo_do_start(obj: *Eo, cur_klass: *EoClass, is_super: eina::EinaBool,
                    file: *c_char, func: *c_char, line: c_int) -> eina::EinaBool;
    fn _eo_do_end(obj: **Eo);
}

pub fn _do_start(obj: *Eo, cur_klass: *EoClass, is_super: bool,
                 file: &str, func: &str, line: int) -> bool {
    file.with_c_str(|c_file| unsafe {
        func.with_c_str(|c_func| {
            eseful::from_eina_to_bool(_eo_do_start(obj, cur_klass, eseful::from_bool_to_eina(is_super),c_file, c_func, line as c_int))
        })
    })
}

pub fn _do_end(obj: **Eo) {
    unsafe { _eo_do_end(obj) }
}
