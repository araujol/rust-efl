// Elementary Rust bindings for EFL.
// Copyright (C) 2014  Luis Araujo <araujoc.luisf@gmail.com>

// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA


extern crate libc;

use std::str::raw::from_c_str;
use std::ptr;
use std::cast::forget;

use elementary::libc::{c_int, c_uint, c_char};
use evas;
use eina;
use eseful;


/// Types of windows that can be created.
pub enum ElmWinType {
    ElmWinUnknown = -1,
    /// A normal window. Indicates a normal, top-level
    /// window. Almost every window will be created with this
    /// type.
    ElmWinBasic, 
    /// Used for simple dialog windows
    ElmWinDialogBasic,
    /// For special desktop windows, like a background
    /// window holding desktop icons.
    ElmWinDesktop,
    /// The window is used as a dock or panel. Usually would
    /// be kept on top of any other window by the Window Manager.
    ElmWinDock, 
    /// The window is used to hold a floating toolbar, or similar.
    ElmWinToolbar,
    /// Similar to #ElmWin_TOOLBAR.
    ElmWinMenu,
    /// A persistent utility window, like a toolbox or palette.
    ElmWinUtility,
    /// Splash window for a starting up application.
    ElmWinSplash,
    /// The window is a dropdown menu, as when an
    /// entry in a menubar is clicked.
    ElmWinDropdownMenu,
    /// Like #ElmWin_DROPDOWN_MENU, but for the menu
    /// triggered by right-clicking an object.
    ElmWinPopupMenu,
    /// The window is a tooltip.
    ElmWinTooltip,
    /// A notification window, like a warning about
    /// battery life or a new E-Mail received.
    ElmWinNotification, 
    /// A window holding the contents of a combo box. Not
    /// usually used in the EFL.
    ElmWinCombo,
    /// Used to indicate the window is a representation of an
    /// object being dragged across different windows, or even
    /// applications.
    ElmWinDnd,
    /// The window is rendered onto an image buffer.
    ElmWinInlinedImage,
    /// The window is rendered onto an image buffer
    /// and can be shown other process's plug image object.
    ElmWinSocketImage
}

pub enum ElmPolicy {
    ElmPolicyQuit,
    ElmPolicyExit,
    ElmPolicyThrottle
}

pub enum ElmPolicyQuit {
    ElmPolicyQuitNone,
    ElmPolicyQuitLastWindowClosed
}

pub enum ElmLabelSlideMode {
    /// No slide effect
    ElmLabelSlideModeNone = 0,
    /// Slide only if the label area is bigger than the text width length
    ElmLabelSlideModeAuto,
    /// Slide always
    ElmLabelSlideModeAlways
}

pub enum ElmBgOption {
    /// Center the background image.
    ElmBgOptionCenter,
    /// Scale the background image, retaining aspect ratio.
    ElmBgOptionScalse,
    /// Stretch the background image to fill the widget's area
    ElmBgOptionStretch,
    /// Tile background image at its original size
    ElmBgOptionTile,
    /// Sentinel value, also used to indicate errors
    ElmBgOptionLast
}

#[link(name = "elementary")]
extern "C" {
    static mut _elm_startup_time: f64;
    fn elm_init(argc: c_int, argv: **c_char) -> c_int;
    fn elm_run();
    fn elm_shutdown() -> c_int;
    fn elm_exit();
    fn elm_policy_set(policy: c_int, value: c_int);
    fn elm_object_part_text_get(obj: *evas::EvasObject, part: *c_char) -> *c_char;
    fn elm_object_part_text_set(obj: *evas::EvasObject, part: *c_char, text: *c_char);
    fn elm_object_style_set(obj: *evas::EvasObject, style: *c_char) -> u8;
    fn elm_object_focus_get(obj: *evas::EvasObject) -> u8;
    fn elm_object_focus_set(obj: *evas::EvasObject, focus: eina::EinaBool);

    /* elm_win */
    fn elm_win_add(obj: *evas::EvasObject, name: *c_char, wtype: c_int) -> *evas::EvasObject;
    fn elm_win_autodel_set(obj: *evas::EvasObject, autodel: u8);
    fn elm_win_resize_object_add(obj: *evas::EvasObject, 
                                 subobj: *evas::EvasObject);
    fn elm_win_title_get(obj: *evas::EvasObject) -> *c_char;
    fn elm_win_title_set(obj: *evas::EvasObject, title: *c_char);
    fn elm_win_util_standard_add(name: *c_char, title: *c_char) -> *evas::EvasObject;

    /* elm_box */
    fn elm_box_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_box_pack_start(obj: *evas::EvasObject, subobj: *evas::EvasObject);
    fn elm_box_pack_end(obj: *evas::EvasObject, subobj: *evas::EvasObject);
    fn elm_box_padding_set(obj: *evas::EvasObject, h: c_int, v: c_int);
    fn elm_box_homogeneous_set(obj: *evas::EvasObject, homogeneous: eina::EinaBool);

    /* elm_button */
    fn elm_button_add(parent: *evas::EvasObject) -> *evas::EvasObject;

    /* elm_check */
    fn elm_check_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_check_state_set(obj: *evas::EvasObject, state: eina::EinaBool);
    fn elm_check_state_get(obj: *evas::EvasObject) -> eina::EinaBool;

    /* elm_label */
    fn elm_label_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_label_slide_mode_set(obj: *evas::EvasObject, mode: c_uint);
    fn elm_label_slide_duration_set(obj: *evas::EvasObject, duration: f64);
    fn elm_label_slide_go(obj: *evas::EvasObject);

    /* elm_entry */
    fn elm_entry_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_entry_entry_get(obj: *evas::EvasObject) -> *c_char;
    fn elm_entry_entry_set(obj: *evas::EvasObject, entry: *c_char);
    fn elm_entry_is_empty(obj: *evas::EvasObject) -> eina::EinaBool;
    fn elm_entry_scrollable_set(obj: *evas::EvasObject, scroll: eina::EinaBool);
    fn elm_entry_single_line_set(obj: *evas::EvasObject, single_line: eina::EinaBool);

    /* elm_bg */
    fn elm_bg_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_bg_load_size_set(parent: *evas::EvasObject, w: c_int, h: c_int);
    fn elm_bg_option_set(obj: *evas::EvasObject, option: c_uint);
    fn elm_bg_file_set(obj: *evas::EvasObject, file: *c_char, group: *c_char) -> u8;
    
    /* elm_datetime/calendar */
    fn elm_datetime_add(parent: *evas::EvasObject) -> *evas::EvasObject;
    fn elm_calendar_add(parent: *evas::EvasObject) -> *evas::EvasObject;

    fn elm_layout_sizing_eval(obj: *evas::EvasObject);
    fn elm_fileselector_entry_add(parent: *evas::EvasObject) -> *evas::EvasObject;
}

pub fn init(argc: uint, argv: ~[~str]) -> uint {
    let vchars_ptr: **c_char = eseful::get_c_args(argv);

    let ret = unsafe {
        elm_init(argc as c_int, vchars_ptr)
    };

    // Forget this value so it can be stored statically from C
    unsafe { forget(vchars_ptr); }

    return ret as uint;
}

pub fn startup_time(t: f64) {
    unsafe { _elm_startup_time = t }
}

pub fn run() {
    unsafe { elm_run() }
}

pub fn shutdown() -> c_int {
    unsafe { elm_shutdown() }
}

pub fn exit() {
    unsafe { elm_exit() }
}

pub fn policy_set(policy: ElmPolicy, value: int) {
    unsafe { elm_policy_set(policy as c_int, value as c_int) }
}

/* Object methods */
pub fn object_text_get(obj: &evas::EvasObject) -> ~str {
    unsafe {
        from_c_str(elm_object_part_text_get(obj, ptr::null()))
    }
}

pub fn object_text_set(obj: &evas::EvasObject, text: &str) {
    text.with_c_str(|c_text| unsafe {
        elm_object_part_text_set(obj, ptr::null(), c_text);
    })
}

pub fn object_focus_get(obj: &evas::EvasObject) -> bool {
    unsafe { eseful::from_eina_to_bool(elm_object_focus_get(obj)) }
}

pub fn object_focus_set(obj: &evas::EvasObject, focus: bool) {
    unsafe { elm_object_focus_set(obj, eseful::from_bool_to_eina(focus)) }
}

pub fn object_style_set(obj: &evas::EvasObject, style: &str) -> bool {
    style.with_c_str(|c_style| unsafe {
        eseful::from_eina_to_bool(elm_object_style_set(obj, c_style))
    })
}

/* Window methods */
/// Add a window object.
/// If obj is None this is the first window created.
pub fn win_add(obj: Option<&evas::EvasObject>, name: &str,
               wtype: ElmWinType) -> ~evas::EvasObject {
    name.with_c_str(|c_buf| unsafe {
        match obj {
            /* Null pointer */
            None => evas::cast_to_evas_obj(elm_win_add(ptr::null(), c_buf, wtype as c_int)),
            /* Add win to eobj parent */
            Some(eobj) =>
                evas::cast_to_evas_obj(elm_win_add(eobj, c_buf, wtype as c_int))
        }
    })
}

pub fn win_util_standard_add(name: &str, title: &str) -> ~evas::EvasObject {
    name.with_c_str(|c_name| unsafe {
        title.with_c_str(|c_title| {
            evas::cast_to_evas_obj(elm_win_util_standard_add(c_name, c_title))
        })
    })
}

/// Set the window autodel state.
pub fn win_autodel_set(obj: &evas::EvasObject, autodel: bool) {
    unsafe {
        elm_win_autodel_set(obj, eseful::from_bool_to_eina(autodel))
    }
}

/// Add 'subobj' as a resize object of window 'obj'.
pub fn win_resize_object_add(obj: &evas::EvasObject, subobj: &evas::EvasObject) {
    unsafe { elm_win_resize_object_add(obj, subobj) }
}

/// Get the title window.
pub fn win_title_get(obj: &evas::EvasObject) -> ~str {
    unsafe { from_c_str(elm_win_title_get(obj)) }
}

/// Set the title of the window.
pub fn win_title_set(obj: &evas::EvasObject, title: &str) {
    title.with_c_str(|c_buf| unsafe {
        elm_win_title_set(obj, c_buf)
    })
}

/* Box methods */
/// Add a new box to the parent.
pub fn box_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_box_add(parent)) }
}

/// Add an object to the beginning of the pack list.
pub fn box_pack_start(obj: &evas::EvasObject, subobj: &evas::EvasObject) {
    unsafe { elm_box_pack_start(obj, subobj) }
}

/// Add an object at the end of the pack list.
pub fn box_pack_end(obj: &evas::EvasObject, subobj: &evas::EvasObject) {
    unsafe { elm_box_pack_end(obj, subobj) }
}

/// Set the box to arrange its children homogeneously.
pub fn box_homogeneous_set(obj: &evas::EvasObject, homogeneous: bool) {
    unsafe {
        elm_box_homogeneous_set(obj, eseful::from_bool_to_eina(homogeneous))
    }
}

/// Set the space (padding) between the box's elements.
pub fn box_padding_set(obj: &evas::EvasObject, p: evas::Coord) {
    let (x, y) = p;
    unsafe { elm_box_padding_set(obj, x as c_int, y as c_int) }
}

/* Button methods */
/// Add a new button to the parent's canvas.
pub fn button_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_button_add(parent)) }
}

/* Check methods */
/// Add a new Check object.
pub fn check_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_check_add(parent)) }
}

/// Set the on/off state of the check object.
pub fn check_state_set(obj: &evas::EvasObject, state: bool) {
    unsafe {
        elm_check_state_set(obj, eseful::from_bool_to_eina(state))
    }
}

/// Get the state of the check object.
pub fn check_state_get(obj: &evas::EvasObject) -> bool {
    unsafe { eseful::from_eina_to_bool(elm_check_state_get(obj)) }
}

/* Entry methods */
/// This adds an entry to parent object.
pub fn entry_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_entry_add(parent)) }
}

/// Get whether the entry is empty.
pub fn entry_is_empty(obj: &evas::EvasObject) -> eina::EinaBool {
    unsafe { elm_entry_is_empty(obj) as eina::EinaBool }
}

/// Enable or disable scrolling in entry.
pub fn entry_scrollable_set(obj: &evas::EvasObject, scroll: bool) {
    unsafe {
        elm_entry_scrollable_set(obj, eseful::from_bool_to_eina(scroll))
    }
}

/// Sets the entry to single line mode.
pub fn entry_single_line_set(obj: &evas::EvasObject, single_line: bool) {
    unsafe {
        elm_entry_single_line_set(obj, eseful::from_bool_to_eina(single_line))
    }
}

/// This returns the text currently shown in object entry.
pub fn entry_entry_get(obj: &evas::EvasObject) -> ~str {
    unsafe { from_c_str(elm_entry_entry_get(obj)) }
}

/// This sets the text displayed within the entry to 'entry'.
pub fn entry_entry_set(obj: &evas::EvasObject, entry: &str) {
    entry.with_c_str(|c_buf| unsafe {
        elm_entry_entry_set(obj, c_buf);
    })
}

/* Label methods */
/// Add a new label to the parent.
pub fn label_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_label_add(parent)) }
}

/// Set the slide mode of the label widget.
/// ELM_LABEL_SLIDE_MODE_NONE - no slide effect
/// ELM_LABEL_SLIDE_MODE_AUTO - slide only if the label area is bigger than the text width length
/// ELM_LABEL_SLIDE_MODE_ALWAYS -slide always
pub fn label_slide_mode_set(obj: &evas::EvasObject, mode: ElmLabelSlideMode) {
    unsafe { elm_label_slide_mode_set(obj, mode as c_uint) }
}

/// Set the slide duration of the label.
pub fn label_slide_duration_set(obj: &evas::EvasObject, duration: f64) {
    unsafe { elm_label_slide_duration_set(obj, duration) }
}

/// Start slide effect.
pub fn label_slide_go(obj: &evas::EvasObject) {
    unsafe { elm_label_slide_go(obj) }
}

/* Date/Calendar methods */
pub fn datetime_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_datetime_add(parent)) }
}

pub fn calendar_add(parent: &evas::EvasObject) -> *evas::EvasObject {
    unsafe { elm_calendar_add(parent) }
}

/* Fileselector methods */
pub fn fileselector_entry_add(parent: &evas::EvasObject) -> *evas::EvasObject {
    unsafe { elm_fileselector_entry_add(parent) }
}

pub fn layout_sizing_eval(obj: &evas::EvasObject) {
    unsafe { elm_layout_sizing_eval(obj) }
}

/// Add a new background to the parent.
pub fn bg_add(parent: &evas::EvasObject) -> ~evas::EvasObject {
    unsafe { evas::cast_to_evas_obj(elm_bg_add(parent)) }
}

pub fn bg_load_size_set(parent: &evas::EvasObject, c: evas::Coord) {
    let (w, h) = c;
    unsafe { elm_bg_load_size_set(parent, w as c_int, h as c_int) }
}

pub fn bg_option_set(obj: &evas::EvasObject, option: ElmBgOption) {
    unsafe { elm_bg_option_set(obj, option as c_uint) }
}

pub fn bg_file_set(obj: &evas::EvasObject, file: &str, group: &str) -> eina::EinaBool {
    file.with_c_str(|c_file| unsafe {
        group.with_c_str(|c_group| {
            elm_bg_file_set(obj, c_file, c_group) as eina::EinaBool
        })
    })
}
