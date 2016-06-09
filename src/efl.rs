// Enlightenment Foundation Libraries Rust bindings.
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

#![crate_name="efl"]
#![crate_type="lib"]

#[macro_use] extern crate enum_primitive;

/* Project-wide Issues

TODO CString pointers review.
 Currently, as_ptr() is used. Should into_raw() and from_raw() be used in some places?

TODO Replace int/uint with appropriate primitive: i32, i64, u32, u64, usize, isize

*/

pub mod eo;
pub mod ecore;
pub mod evas;
pub mod eina;
pub mod eio;
pub mod eet;
pub mod edje;
pub mod eldbus;
pub mod emotion;
pub mod elementary;
pub mod eseful;
pub mod types {
    // Define old int/uint types used by original Rust code
    #[allow(non_camel_case_types)]
    pub type int = isize;
    #[allow(non_camel_case_types)]
    pub type uint = usize;
}
