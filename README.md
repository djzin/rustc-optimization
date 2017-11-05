rustc optimization tests
========================

All the .rs files in this repository consist of a single function that should "do nothing" - in the case of nullary functions, the function should return with no side-effects, and in the case of unary functions, the function should return its argument with no side-effects.

To run the tests, just `make` and it should print a summary of the tests that failed. You can also set the `RUSTFLAGS` variable to something other than the default (`-C opt-level=3 -C target-cpu=native -C panic=abort`).
