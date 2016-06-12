/*
 * Test background methods.
 */

extern crate efl;

use std::env;

use efl::types::int;
use efl::ecore;
use efl::evas;
use efl::elementary;


fn main() {

    use efl::elementary::{ElmPolicy, ElmPolicyQuit, ElmWinType, ElmBgOption};

    let args: Vec<String> = env::args().collect();

    let logo_file: String =
        if args.len() > 1 {
            args[1].clone()  // clone because args in owned by elementary::init later
        } else {
            panic!("No logo file. Usage: {:?} <file>", args[0])
        };

    elementary::startup_time(ecore::time_unix_get());
    elementary::init(args);

    elementary::policy_set(ElmPolicy::Quit, ElmPolicyQuit::LastWindowClosed as int);

    let win = elementary::win_add(None, "Rust Logo", ElmWinType::ElmWinBasic);
    elementary::win_title_set(&win, "Background Rust");
    elementary::win_autodel_set(&win, true);

    let bg = elementary::bg_add(&win);
    elementary::bg_load_size_set(&bg, (128, 128));
    elementary::bg_option_set(&bg, ElmBgOption::ElmBgOptionCenter);
    elementary::bg_file_set(&bg, &logo_file, "");
    evas::object_size_hint_weight_set(&bg, evas::EVAS_HINT_EXPAND,
                                      evas::EVAS_HINT_EXPAND);

    elementary::win_resize_object_add(&win, &bg);
    evas::object_show(&bg);

    evas::object_resize(&win, 320, 320);
    evas::object_show(&win);

    /* Start main event loop */
    elementary::run();
    elementary::shutdown();
}
