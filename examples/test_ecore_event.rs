/*
 * Test ecore event handlers.
 */

extern crate efl;

use std::env;
use std::mem::transmute;
use std::option::{Option};

use efl::ecore;
use efl::ecore::EcoreEvent;
use efl::eina::EinaBool;
use efl::eseful::{EventInfo, EMPTY};

fn timer(data: &Option<f64>) -> EinaBool {
    match *data {
        None => (),
        Some(stime) =>
            println!("Tick timer: {}", ecore::time_get() - stime)
    }
    ecore::ECORE_CALLBACK_RENEW
}

#[allow(unused_variables)]
fn exit_func<T>(data: &Option<T>,
                htype: isize,
                event: &EventInfo) -> bool {
    // TODO do unsafe transmute in the library? Maybe implement patterns and match?
    let e: &ecore::EcoreEventSignalExit = unsafe { transmute(event) };

    if (*e).quit == 1 {
        println!("Exit: quit");
    } else if (*e).interrupt == 1 {
        println!("Exit: interrupt");
    } else if (*e).terminate == 1 {
        println!("Exit: terminate");
    }

    ecore::main_loop_quit();
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    ecore::init();
    ecore::app_args_set(args);

    let start_time: f64 = ecore::time_get();
    /* Add timer and handler */
    ecore::event_handler_add(EcoreEvent::EcoreEventSignalExit,
                             exit_func, &EMPTY);
    ecore::timer_add(1.0, timer, &Some(start_time));

    /* Start main event loop */
    ecore::main_loop_begin();
    ecore::shutdown();
}
