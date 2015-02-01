/*
 * Test Emotion media player features.
 */

extern crate efl;

use std::os;

use efl::ecore;
use efl::evas;
use efl::emotion;
use efl::eseful::EventInfo;
use std::option::Option;

static WIDTH: int = 360;
static HEIGHT: int = 285;


fn playback_started(data: &Option<&str>,
                    e: &evas::EvasObject,
                    event_info: &EventInfo) {
    match *data {
        None => (),
        Some(ref video) => 
            println!("Video started!: {}", video)
    }
}


fn main() {    
    let args: Vec<String> = os::args();

    let video_file: &str =
        if args.len() > 1 {
            args.get(1).as_slice()
        } else {
            println!("No video file. Usage: {} <file>", args.get(0));
            panic!()
        };

    ecore::evas_init();
    let ee: Box<ecore::EcoreEvas> =
        ecore::evas_new(None, 10, 10, WIDTH, HEIGHT, "");
    ecore::evas_show(ee);

    /* Get the canvas */
    let e: Box<evas::Evas> = ecore::evas_get(ee);

    /* Add a white background */
    let bg: Box<evas::EvasObject>  = evas::object_rectangle_add(e);
    evas::object_name_set(bg, "video-rectangle");
    evas::object_color_set(bg, 255, 255, 255, 255);
    evas::object_move(bg, (0, 0));
    evas::object_resize(bg, WIDTH, HEIGHT);
    evas::object_show(bg);

    /* Set the media module to use */
    let em: Box<evas::EvasObject> = emotion::object_add(e);
    emotion::object_init(em, "gstreamer1");

    let data: Option<&str> = Some(video_file);
    evas::object_smart_callback_add(em, "playback_started",
                                    playback_started, &data);
    emotion::object_file_set(em, video_file);
    
    evas::object_move(em, (0, 0));
    evas::object_resize(em, WIDTH, HEIGHT);
    evas::object_show(em);

    /* Play media */
    emotion::object_play_set(em, true);

    ecore::main_loop_begin();
    
    ecore::evas_free(ee);
    ecore::evas_shutdown();
}
