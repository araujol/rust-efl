/*
 * Test for EinaList.
 */

extern crate efl;

use efl::eina;

fn main() {

    eina::init();

    let mut strlst: *mut eina::_EinaList<&'static str> =
        eina::list_append(None, &("Rust"));

    println!("Last Value? {}", eina::list_last_data_get(strlst));
    println!("Next Value? {}", eina::list_next(strlst));
    println!("Previous Value? {}", eina::list_prev(strlst));

    // Prepend a new node
    strlst = eina::list_prepend(Some(strlst), &("rust-efl"));
    println!("First Value: {}", eina::list_data_get(strlst));

    strlst = eina::list_append(Some(strlst), &("EFL"));
    strlst = eina::list_append(Some(strlst), &("Rust EFL!"));

    let next =
        match eina::list_next(strlst) {
            None => {
                println!("No next! Returning same");
                strlst
            }
            Some(next) => { 
                println!("Next: {}", eina::list_data_get(next));
                next
            }
        };

    let last =
        match eina::list_last(strlst) {
            None => { panic!("No last?!") }
            Some(last) => {
                println!("Last Value: {}", eina::list_data_get(last));
                last
            }
        };

    // Change first value
    let v: &&'static str = &("Rust Enlightenment");
    eina::list_data_set(strlst, v);
    // Change last value
    let n: &&'static str = &("EnLiGhTeNmEnT");
    eina::list_data_set(last, n);

    println!("First New Value: {}", eina::list_data_get(strlst));
    println!("Next still: {}", eina::list_data_get(next));
    println!("Last New Value: {}", eina::list_last_data_get(strlst));

    // Change next value
    let t = &("Enlightenment through Rust!");
    println!("Old next: {}, changing!", eina::list_data_set(next, t));
    println!("Next value changed to: {}", eina::list_data_get(next));

    println!("List count: {}", eina::list_count(strlst));
    // Add new node
    strlst = eina::list_append(Some(strlst), &("e rust"));
    println!("List new count: {}", eina::list_count(strlst));
    println!("New added value {}", eina::list_last_data_get(strlst));

    eina::list_free(strlst);
    eina::shutdown();

}
