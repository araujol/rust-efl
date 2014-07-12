libefl.rlib: src/efl.rs
	rustc src/efl.rs --crate-type=lib

all: libefl.rlib

tests: tests/test_simple.rs
	rustc tests/test_simple.rs -L .

clean:
	rm -f test_*
	rm -f libefl.rlib

.PHONY: all tests
