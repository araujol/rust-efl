/*
 * Test eldbus bindings.
 */

/* Enable macros */
#![feature(phase)]
#[phase(syntax,link)]
extern crate efl;

extern crate libc;

use libc::c_char;
use std::c_str::CString;
use efl::ecore;
use efl::eldbus;
use std::ptr;


fn _on_method_call(data: &int,
                   msg: &eldbus::EldbusMessage,
                   pending: &eldbus::EldbusPending) {    
    let mut errname: ~str = "".to_owned();
    let mut errmsg: ~str = "".to_owned();

    if eldbus::message_error_get(msg, Some(&mut errname), Some(&mut errmsg)) {
        println!("Error: {}, {}", errname, errmsg);
        return;
    }

    // Receive the message value in val
    let mut val: *c_char = ptr::null();
    let signature = "s";
    message_arguments_get!(msg, signature, &mut val);
    unsafe {
        match CString::new(val, false).as_str() {
            None => println!("Not valid string"),
            Some(s) => println!("{}", s)
        }
    }
}

fn main() {

    ecore::init();
    eldbus::init();

    let conn: ~eldbus::EldbusConnection =
        eldbus::connection_get(eldbus::EldbusConnectionTypeSystem);

    let obj: ~eldbus::EldbusObject =
        eldbus::object_get(conn, "org.freedesktop.DBus", "/");
    //let obj: ~eldbus::EldbusObject =
    //    eldbus::object_get(conn, "org.freedesktop.Avahi", "/");

    let manager: &eldbus::EldbusProxy = eldbus::proxy_get(obj, "org.freedesktop.DBus.Introspectable");
    //let manager: &eldbus::EldbusProxy = eldbus::proxy_get(obj, "org.freedesktop.Avahi.Server");

    let member = "Introspect";
    //let member = "GetAlternativeHostName";
    //let hostname = "name-test";
    let signature = "";
    //let signature = "s";
    let data = &0;
    let timeout = -1.0f64;

    proxy_call!(manager, member, _on_method_call,
                data, timeout, signature);
    /*
    hostname.with_c_str(|c_hostname| unsafe {
        proxy_call!(manager, member, _on_method_call, data,
                    timeout, signature, c_hostname)
    });
     */
    ecore::main_loop_begin();

    eldbus::proxy_unref(manager);
    eldbus::object_unref(obj);
    eldbus::connection_unref(conn);

    eldbus::shutdown();
    ecore::shutdown();

}
