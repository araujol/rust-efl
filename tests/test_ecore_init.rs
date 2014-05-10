/*
 * Initialize ecore and run the main event loop only.
 */

extern crate efl;

use std::os;
use efl::ecore;

fn main() {
    let args: Vec<~str> = os::args();
    let argc: uint = args.len();

    ecore::init();
    ecore::app_args_set(argc, args);
    ecore::main_loop_begin();
    ecore::shutdown();
}