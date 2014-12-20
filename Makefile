libefl.rlib: src/*.rs
	rustc src/efl.rs --crate-type=lib

all: libefl.rlib

examples: examples/test_simple.rs
	rustc examples/test_simple.rs -L .

clean:
	rm -f test_*
	rm -f libefl.rlib

.PHONY: all examples
