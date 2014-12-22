/*
 * Test Ecore, Evas, and Elementary Rust bindings...
 *
 * Display a text label, entry and button.
 * Either pressing the button or enter in the text entry will update
 * the text label.
 *
 * This example shows how to share data between callbacks.
 *
 */

extern crate efl;

use std::os;
use std::option::{Option};

use efl::ecore;
use efl::evas;
use efl::elementary;
use efl::eseful::{EventInfo, Empty};

struct OnClickedData<'r> {
    label: &'r evas::EvasObject,
    entry: &'r evas::EvasObject
}

fn on_done<T>(data: &Option<T>,
              e: &evas::EvasObject,
              event_info: &EventInfo) {
    elementary::exit()
}

fn on_enter(data: &Option<&evas::EvasObject>,
            e: &evas::EvasObject,
            event_info: &EventInfo) {
    match *data {
        None => (),
        Some(eobj) => {
            elementary::object_text_set(eobj, elementary::entry_entry_get(e).as_slice());
            /* Reset text entry */
            elementary::entry_entry_set(e, "");
        }
    }
}

fn on_clicked(data: &Option<Box<OnClickedData>>,
              e: &evas::EvasObject,
              event_info: &EventInfo) {
    match *data {
        None => (),
        Some(ref onclicked) => {
            let txt = elementary::entry_entry_get(onclicked.entry);
            elementary::object_text_set(onclicked.label, txt.as_slice());
            /* Reset text entry */
            elementary::entry_entry_set(onclicked.entry, "");
        }
    }
}

fn main() {
    let args: Vec<String> = os::args();
    let argc: uint = args.len();

    elementary::startup_time(ecore::time_unix_get());
    elementary::init(argc, args);

    /* Main Window */
    let win: Box<evas::EvasObject> =
        elementary::win_util_standard_add("Rust EFL", "Rust EFL");
    evas::object_move(&*win, (200, 100));
    evas::object_smart_callback_add(&*win, "delete,request", on_done, &Empty);

    /* Box Container */
    let ebox: Box<evas::EvasObject> = elementary::box_add(&*win);
    evas::object_size_hint_weight_set(&*ebox,
                                      evas::EVAS_HINT_EXPAND,
                                      evas::EVAS_HINT_EXPAND);
    elementary::win_resize_object_add(&*win, &*ebox);
    evas::object_show(&*ebox);

    /* Label */
    let lab: Box<evas::EvasObject> = elementary::label_add(&*win);
    elementary::object_text_set(&*lab, "[Enter Text]");
    elementary::box_pack_end(&*ebox, &*lab);
    evas::object_show(&*lab);

    /* Entry */
    let ent: Box<evas::EvasObject> = elementary::entry_add(&*win);
    elementary::entry_scrollable_set(&*ent, true);
    elementary::entry_single_line_set(&*ent, true);
    evas::object_size_hint_weight_set(&*ent,
                                      evas::EVAS_HINT_EXPAND,
                                      evas::EVAS_HINT_EXPAND);
    evas::object_size_hint_align_set(&*ent,
                                     evas::EVAS_HINT_FILL,
                                     evas::EVAS_HINT_FILL);
    evas::object_show(&*ent);
    elementary::box_pack_end(&*ebox, &*ent);
    elementary::object_focus_set(&*ent, true);

    let l: &evas::EvasObject = &*lab;
    evas::object_smart_callback_add(&*ent, "activated", on_enter, &Some(l));

    /* Button */
    let btn = elementary::button_add(&*win);
    evas::object_size_hint_weight_set(&*btn, 
                                      evas::EVAS_HINT_EXPAND, 
                                      evas::EVAS_HINT_EXPAND);
    evas::object_size_hint_align_set(&*btn, 
                                     evas::EVAS_HINT_FILL,
                                     evas::EVAS_HINT_FILL);
    elementary::object_text_set(&*btn, "Ok");
    evas::object_show(&*btn);
    elementary::box_pack_end(&*ebox, &*btn);

    /* Share both the 'label' and 'entry' objects with the button callback */
    let e: &evas::EvasObject = &*ent;
    let onclicked_data: Option<Box<OnClickedData>> =
        Some(box OnClickedData {
            label: l,
            entry: e
        });
    evas::object_smart_callback_add(&*btn, "clicked", on_clicked, &onclicked_data);

    /* Set main window size and show */
    evas::object_resize(&*win, 200, 50);
    evas::object_show(&*win);

    /* Start main event loop */
    elementary::run();
    elementary::shutdown();
}
