/*
 * Test ecore event handlers.
 */

extern crate efl;

use std::os;
use std::mem::transmute;
use std::option::{Option};

use efl::ecore;
use efl::eina::EinaBool;
use efl::eseful::{EventInfo, Empty};

fn timer(data: &Option<f64>) -> EinaBool {
    match *data {
        None => (),
        Some(stime) =>
            println!("Tick timer: {}", ecore::time_get() - stime)
    }
    return ecore::ECORE_CALLBACK_RENEW
}

fn exit_func<T>(data: &Option<T>,
                htype: int,
                event: &EventInfo) -> bool {
    let e: &ecore::EcoreEventSignalExit = unsafe { transmute(event) };

    if (*e).quit == 1 {
        println!("Exit: quit");
    } else if (*e).interrupt == 1 {
        println!("Exit: interrupt");
    } else if (*e).terminate == 1 {
        println!("Exit: terminate");
    }

    ecore::main_loop_quit();
    return true;
}

fn main() {
    let args: Vec<StrBuf> = os::args();
    let argc: uint = args.len();

    ecore::init();
    ecore::app_args_set(argc, args);

    let start_time: f64 = ecore::time_get();
    /* Add timer and handler */
    /* TODO: Create enum for signals */
    ecore::event_handler_add(ecore::EcoreEventSignalExit, 
                             exit_func, &Empty);
    ecore::timer_add(1.0, timer, &Some(start_time));

    /* Start main event loop */
    ecore::main_loop_begin();
    ecore::shutdown();
}
