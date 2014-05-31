#  Rust EFL 

Rust bindings for the Enlightenment Foundation Libraries (version 1.10.0-0.0.1.99).

These versions of the bindings currently correspond to the latest code base
both for EFL (1.10.x) and the Rust language, so it is required to fetch and 
install the necessary libraries/implementation from the master git repository 
for both of these projects.

- EFL: http://git.enlightenment.org/
- Rust: https://github.com/mozilla/rust.git

Requirements:

- efl *EFL core libraries*
- evas_generic_loaders *Loaders for Evas using 'generic' module*
- emotion_generic_players *Players for Emotion using 'generic' module*
- elementary *EFL widget toolkit*

## Building library:

In the top level directory of this source code:

    $ rustc src/efl.rs --crate-type=lib

Now you can build programs using the rust-efl library bindings:

    $ rustc tests/test_simple.rs -L.

## Run:

    $ ./test_simple

Enjoy!
