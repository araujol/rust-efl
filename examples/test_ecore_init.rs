/*
 * Initialize ecore and run the main event loop only.
 */

extern crate efl;

use std::env;
use efl::ecore;

fn main() {
    ecore::init();
    ecore::app_args_set(env::args().collect());
    ecore::main_loop_begin();
    ecore::shutdown();
}
