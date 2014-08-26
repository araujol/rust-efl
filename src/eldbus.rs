// Eldbus Rust bindings for EFL.
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

use std::mem::transmute;
use std::c_str::CString;
use std::option::Option;
use std::ptr;

use eldbus::libc::{c_int, c_uint, c_char};
// Re-export for macro usage
pub use self::libc::{c_void, c_double};

use eina;
use eseful::from_eina_to_bool;

/// Represents a connection of one the type of connection with the DBus daemon.
pub enum EldbusConnection {}

/// Represents an object path already attached with bus name or unique id.
pub enum EldbusObject {}

/// Represents an interface of an object path.
pub enum EldbusProxy {}

/// Represents the way data is sent and received in DBus.
pub enum EldbusMessage {}

/// Represents a message that has been sent but has not yet reached its destination.
pub enum EldbusPending {}

pub enum EldbusConnectionType
{
    EldbusConnectionTypeUnknown = 0,
    EldbusConnectionTypeSession,
    EldbusConnectionTypeSystem,
    EldbusConnectionTypeStarter,
    EldbusConnectionTypeAddress,
    EldbusConnectionTypeLast
}

pub type EldbusMessageCb<T> = fn (&T, &EldbusMessage, &EldbusPending);
/* This C equivalent type needs to be public for proper macro expansion */
pub type _CEldbusMessageCb = fn (*const c_void, *const EldbusMessage, *const EldbusPending);

#[link(name = "eldbus")]
extern "C" {
    fn eldbus_init() -> c_int;
    fn eldbus_shutdown() -> c_int;
    fn eldbus_connection_get(conn_type: c_uint) -> *const EldbusConnection;
    fn eldbus_object_get(conn: *const EldbusConnection, 
                         bus: *const c_char,
                         path: *const c_char) -> *const EldbusObject;
    fn eldbus_proxy_get(obj: *const EldbusObject, interface: *const c_char) -> *const EldbusProxy;
    fn eldbus_proxy_call(proxy: *const EldbusProxy, member: *const c_char,
                         cb: _CEldbusMessageCb, cb_data: *const c_void,
                         timeout: c_double, signature: *const c_char,
                         ...) -> *const EldbusPending;
    fn eldbus_message_arguments_get(msg: *const EldbusMessage, signature: *const c_char,
 	                            ...) -> eina::EinaBool;
    fn eldbus_proxy_unref(proxy: *const EldbusProxy);
    fn eldbus_object_unref(obj:	*const EldbusObject);
    fn eldbus_connection_unref(conn: *const EldbusConnection);
    fn eldbus_message_error_get(msg: *const EldbusMessage, name: *const *mut c_char,
                                text: *const *mut c_char) -> eina::EinaBool;
}

/// Initialize eldbus.
pub fn init() -> int { unsafe { eldbus_init() as int } }

/// Shutdown eldbus.
pub fn shutdown() -> int { unsafe { eldbus_shutdown() as int } }

/// Establish a connection to bus and integrate it with the ecore main loop.
pub fn connection_get(conn_type: EldbusConnectionType) -> Box<EldbusConnection> {
    unsafe { 
        transmute(eldbus_connection_get(conn_type as c_uint))
    }
}

/// Get an object of the given bus and path.
pub fn object_get(conn: &EldbusConnection, bus: &str, path: &str) -> Box<EldbusObject> {
    bus.with_c_str(|c_bus| unsafe {
        path.with_c_str(|c_path| {
            transmute(eldbus_object_get(conn, c_bus, c_path))
        })
    })
}

/// Get a proxy of the following interface name in a EldbusObject.
pub fn proxy_get(obj: &EldbusObject, interface: &str) -> Box<EldbusProxy> {
    interface.with_c_str(|c_interface| unsafe {
        transmute(eldbus_proxy_get(obj, c_interface))
    })
}

/// Decrease proxy reference.
pub fn proxy_unref(proxy: &EldbusProxy) {
    unsafe { eldbus_proxy_unref(proxy) }
}

/// Decrease object reference.
pub fn object_unref(obj: &EldbusObject) {
    unsafe { eldbus_object_unref(obj) }
}

/// Decrement connection reference count.
pub fn connection_unref(conn: &EldbusConnection) {
    unsafe { eldbus_connection_unref(conn) }
}

/// Call a method in proxy.
/// This is just a convenient function that sends no data value.
/// To send data values, use the proxy_call! macro.
pub fn proxy_call<T>(proxy: &EldbusProxy, member: &str,
                     cb: EldbusMessageCb<T>, cb_data: &T,
                     timeout: f64, signature: &str) -> Box<EldbusPending> {
    member.with_c_str(|c_member| unsafe {
        signature.with_c_str(|c_signature| {
            transmute(eldbus_proxy_call(proxy, c_member,
                                        transmute(cb), transmute(cb_data),
                                        timeout as c_double, c_signature))
        })
    })
}

/// Get the arguments from an Eldbus_Message.
/// This is just a convenient function for receiving a value of signature T.
/// To receive a variable list of values, use the message_arguments_get! macro.
pub fn message_arguments_get<T>(msg: &EldbusMessage, signature: &str,
 	                        arg: &T) -> bool {
    signature.with_c_str(|c_signature| unsafe {
        let c_arg: *const c_char = transmute(arg);
        from_eina_to_bool(eldbus_message_arguments_get(msg, c_signature, c_arg))
    })
}

/// Get the error text and name from a Eldbus_Message.
pub fn message_error_get(msg: &EldbusMessage, name: Option<&mut String>, text: Option<&mut String>) -> bool {
    unsafe {
        let errname: *mut c_char = ptr::mut_null();
        let errmsg: *mut c_char = ptr::mut_null();

        let b =
            if name.is_some() {
                if text.is_some() {
                    from_eina_to_bool(eldbus_message_error_get(msg, &errname, &errmsg))
                } else {
                    from_eina_to_bool(eldbus_message_error_get(msg, &errname, ptr::null()))
                }
            } else {
                if text.is_some() {
                    from_eina_to_bool(eldbus_message_error_get(msg, ptr::null(), &errmsg))
                } else {
                    from_eina_to_bool(eldbus_message_error_get(msg, ptr::null(), ptr::null()))
                }
            };

        if errname.is_not_null() {
            let name_cstr = CString::new(transmute::<_,*const c_char>(errname), false);
            let _name = name.unwrap();
            *_name = match name_cstr.as_str() {
                None => fail!("Not valid string"), Some(s) => s.to_string()
            };
        }

        if errmsg.is_not_null() {
            let text_cstr = CString::new(transmute::<_,*const c_char>(errmsg), false);
            let _text = text.unwrap();
            *_text = match text_cstr.as_str() {
                None => fail!("Not valid string"), Some(s) => s.to_string()
            };
        }

        return b;
    }
}

/// Call a method in proxy.
/// This is the macro version of the function 'eldbus_proxy_call'
/// which should allow passing full variable argument lists.
#[macro_export]
macro_rules! proxy_call(
    ($proxy:ident, $member:ident, $cb:ident, $cb_data:ident, $timeout:ident, $signature:ident $(, $obj:ident)*) => (
        $member.with_c_str(|c_member| unsafe {
            use std::mem::transmute;
            use efl::eldbus;
            $signature.with_c_str(|c_signature| {
                let c_cb: eldbus::_CEldbusMessageCb = transmute($cb);
                let c_cb_data: *const *const eldbus::c_void = transmute($cb_data);
                transmute::<*const *const eldbus::EldbusPending,Box<eldbus::EldbusPending>>(eldbus::eldbus_proxy_call(transmute::<&eldbus::EldbusProxy,*const *const eldbus::EldbusProxy>($proxy), c_member, c_cb, c_cb_data, $timeout as eldbus::c_double, c_signature $(, $obj)*))
            })
        })
    );
)

/// Get the arguments from an Eldbus_Message.
/// This is the macro version of the function 'eldbus_message_arguments_get'
/// which should allow receiving full variable argument lists.
#[macro_export]
macro_rules! message_arguments_get(
    ($msg:ident, $signature:ident $(, $args:expr)*) => (
        $signature.with_c_str(|c_signature| unsafe {
            use efl::{eldbus, eseful};
            eseful::from_eina_to_bool(eldbus::eldbus_message_arguments_get($msg, c_signature $(, $args)*))
        })
    )
)
