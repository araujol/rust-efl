/*
 * Test for using Eina_Inlist objects
 */
#![feature(phase)]
#[phase(syntax,link)]
extern crate efl;

use efl::eina;

struct Data<'r> {
    // First field in the data struct needs to be
    // named __in_list of type _EinaInlist
    __in_list: eina::_EinaInlist,
    a: int,
    b: f32,
    c: uint,
    greet: &'r str,
}


fn main() {
    eina::init();

    let d: &mut Data = &mut eina::object::<Data>();
    d.a = 10;
    d.b = 32.9;
    d.c = 7;
    d.greet = "Rust Enlightenment!";
    /* Append data and create list for first time */
    let mut list: eina::EinaInlist =
        eina::inlist_append(None, inlist_get!(d));

    let t: &mut Data = &mut eina::object::<Data>();
    t.a = 298;
    t.b = 46.9;
    t.c = 3;
    t.greet = "RustEFl!";
    list = eina::inlist_prepend(Some(list), inlist_get!(t));

    let x: &mut Data = &mut eina::object::<Data>();
    x.a = 64;
    x.b = 100.3;
    x.c = 8;
    x.greet = "Hi Rust efl!";
    list = eina::inlist_append(Some(list), inlist_get!(x));

    let mut val: &Data = eina::inlist_container_get(list);
    println!("First value => a: {}, b: {}, c: {}, greet: {}",
             val.a, val.b, val.c, val.greet);

    /* Place 't' at the end of the list */
    list = eina::inlist_demote(Some(list), inlist_get!(t));
    val = eina::inlist_container_get(list);
    println!("New first value => a: {}, b: {}, c: {}, greet: {}",
             val.a, val.b, val.c, val.greet);

    /* Iterate over the list elements */
    println!("Iterating =>");
    for e in list {
        let elm: &Data = e;
        println!("a: {}, b: {}, c: {}, greet: {}", elm.a, elm.b, elm.c, elm.greet);
    }

    eina::shutdown();
}
