`midas_rs` Python bindings
==========================

[![](http://meritbadge.herokuapp.com/midas_python)](https://crates.io/crates/midas_python)

Python bindings to the Rust port of
https://github.com/bhatiasiddharth/MIDAS

Requires
* `cargo` (part of the Rust toolchain)—install via
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  (Or visit rustup.sh for more detailed installation instructions.)
* `make`
* `python3`

Requirements note: Building has only been tested on Linux (Ubuntu 18.04)
and recent versions of macOS.

To build+test from this directory, run
```sh
make
```

For example code, see [test.py](./test.py)

Note: These bindings currently only expose the `MidasR` variant of the
original implementation (which is the variant that includes time-based
decay logic).

For information on the algorithm or interpreting the output, see the
link to the original as well as the corresponding paper.
