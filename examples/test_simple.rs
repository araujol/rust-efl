/*
 * Simple Test for ecore, evas and elementary (EFL) Rust bindings 
 *
 * Display a window with the "Hello Rust Enlightenment!" string
 * in the middle.
 * 
 */

extern crate efl;

use std::env;
use std::option::{Option};

use efl::ecore;
use efl::evas;
use efl::elementary;
use efl::eseful::EventInfo;

fn on_done(data: &Option<bool>,
           e: &evas::EvasObject,
           event_info: &EventInfo) {
    match *data {
        None => (),
        Some(b) => println!("Value passed: {}", b)
    }
    println!("Window title: {}", elementary::win_title_get(e));
    elementary::exit()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    elementary::startup_time(ecore::time_unix_get());
    elementary::init(argc, args);

    /* Main Window */
    let win: Box<evas::EvasObject> =
        elementary::win_util_standard_add("Enlightenment", "Rust EFL");
    let data: Option<bool> = Some(true);
    evas::object_smart_callback_add(&*win, "delete,request",
                                    on_done, &data);
    evas::object_move(&*win, (200, 100));

    /* Box Container */
    let ebox: Box<evas::EvasObject> = elementary::box_add(&*win);
    evas::object_size_hint_weight_set(&*ebox,
                                      evas::EVAS_HINT_EXPAND,
                                      evas::EVAS_HINT_EXPAND);
    elementary::win_resize_object_add(&*win, &*ebox);
    evas::object_show(&*ebox);

    /* Label */
    let lab: Box<evas::EvasObject> = elementary::label_add(&*win);
    elementary::object_text_set(&*lab, "Hello <b>Rust Enlightenment!</b>");
    elementary::box_pack_end(&*ebox, &*lab);
    evas::object_show(&*lab);

    evas::object_resize(&*win, 200, 50);
    evas::object_show(&*win);

    /* Main event loop */
    elementary::run();
    elementary::shutdown();
}
