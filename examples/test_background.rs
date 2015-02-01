/*
 * Test background methods.
 */

extern crate efl;

use std::os;

use efl::ecore;
use efl::evas;
use efl::elementary;


fn main() {
    let args: Vec<String> = os::args();
    let argc: uint = args.len();

    let logo_file: &str =
        if args.len() > 1 {
            args.get(1).as_slice()
        } else {
            println!("No logo file. Usage: {} <file>", args.get(0));
            panic!()
        };

    elementary::startup_time(ecore::time_unix_get());
    elementary::init(argc, args.clone());

    elementary::policy_set(elementary::ElmPolicyQuit,
                           elementary::ElmPolicyQuitLastWindowClosed as int);

    let win = elementary::win_add(None, "Rust Logo", elementary::ElmWinBasic);
    elementary::win_title_set(win, "Background Rust");
    elementary::win_autodel_set(win, true);

    let bg = elementary::bg_add(win);
    elementary::bg_load_size_set(bg, (128, 128));
    elementary::bg_option_set(bg, elementary::ElmBgOptionCenter);
    elementary::bg_file_set(bg, logo_file, "");
    evas::object_size_hint_weight_set(bg, evas::EVAS_HINT_EXPAND,
                                      evas::EVAS_HINT_EXPAND);

    elementary::win_resize_object_add(win, bg);
    evas::object_show(bg);

    evas::object_resize(win, 320, 320);
    evas::object_show(win);

    /* Start main event loop */
    elementary::run();
    elementary::shutdown();
}
